/**
 * ActiveEntity — engine-facing Thing driver (ADR-0005 Thing -> ActiveEntity -> EntityEngine).
 *
 * Goals:
 * - Provide deterministic signature and lightweight facets for Entities.
 * - Validate/normalize using Zod schema (schemas.ActiveEntitySchema).
 * - Keep runtime helpers minimal; persistence goes through schema/entity.*.
 */

import { createHash } from 'node:crypto';
import BaseDriver from '../core/driver';
import type { ProcessorInputs } from '../core/contracts';
import { createWorld } from '../../schema/world';
import {
  ActiveFactory,
  type ActiveEntity as ActiveEntitySchemaType,
  ActiveEntitySchema,
} from '../../schema/active';
import type { Repository } from '../../repository/repo';
import { makeInMemoryRepository } from '../../repository/memory';
import { EntitySchema, type Entity } from '../../schema/entity';
import { EntityEngine } from '../../form/entity/engine';
import type { EventBus } from '../core/triad/bus';

export type RawEntity = Record<string, any>;

export type EntityFacet = {
  key: string;
  type?: string;
  value?: unknown;
};

function stableHash(value: unknown): string {
  function canonicalize(v: unknown): any {
    if (v === null || typeof v !== 'object') return v;
    if (Array.isArray(v)) return v.map(canonicalize);
    const keys = Object.keys(v as Record<string, any>).sort();
    const out: Record<string, any> = {};
    for (const k of keys) out[k] = canonicalize((v as any)[k]);
    return out;
  }
  return createHash('sha256')
    .update(JSON.stringify(canonicalize(value)))
    .digest('hex');
}

export function extractEntityKeys(e: RawEntity): string[] {
  const bag = (e?.values ?? e?.properties ?? e?.attrs ?? {}) as Record<
    string,
    unknown
  >;
  return Array.from(new Set(Object.keys(bag))).sort();
}

export function extractEntityFacets(e: RawEntity): EntityFacet[] {
  const keys = extractEntityKeys(e);
  const bag = (e?.values ?? e?.properties ?? e?.attrs ?? {}) as Record<
    string,
    unknown
  >;
  const facets: EntityFacet[] = keys.map((k) => {
    const v = bag[k];
    return {
      key: k,
      type: typeof v,
      value: typeof v === 'string' || typeof v === 'number' ? v : undefined,
    };
  });
  facets.push({ key: '__valueCount', type: 'number', value: keys.length });
  return facets;
}

export function computeEntitySignature(e: RawEntity): string {
  if (!e) return '';
  if (e.id) return String(e.id);
  const labels = Array.isArray(e.labels) ? [...e.labels].sort() : undefined;
  const seed = {
    entityType: e.entityType ?? e.type,
    labels,
    keys: extractEntityKeys(e),
  };
  return stableHash(seed);
}

export type ActiveEntity = ActiveEntitySchemaType & {
  signature?: string;
  facets?: EntityFacet[];
  raw?: RawEntity;
};

export function toActiveEntity(input: unknown): ActiveEntitySchemaType {
  return ActiveEntitySchema.parse(input);
}

export function fromFormEntity(raw: RawEntity): ActiveEntity {
  const signature = computeEntitySignature(raw);
  const entityType = (raw as any)?.entityType ?? (raw as any)?.type;
  const id = (raw as any)?.id ?? signature;
  const labels = Array.isArray((raw as any)?.labels)
    ? (raw as any).labels
    : undefined;
  const base = { id, entityType, labels } as Partial<ActiveEntitySchemaType>;
  const parsed = ActiveEntitySchema.parse({
    id: base.id,
    entityType: base.entityType ?? 'system.Entity',
    labels: base.labels,
  });
  return {
    ...parsed,
    signature,
    facets: extractEntityFacets(raw),
    raw,
  };
}

export default {
  computeEntitySignature,
  extractEntityFacets,
  toActiveEntity,
  fromFormEntity,
};

// --- Compatibility: class wrapper expected by older tests -------------------------------
export class ActiveEntityWrapper {
  raw: RawEntity;
  signature: string;
  facets: EntityFacet[];

  constructor(raw: RawEntity) {
    this.raw = raw || {};
    this.signature = computeEntitySignature(this.raw);
    this.facets = extractEntityFacets(this.raw);
  }

  runtimeMeta() {
    return { signature: this.signature, facets: this.facets };
  }
}

// Provide a constructor-compatible alias for older tests/imports that expect a class named
// `ActiveEntity`. Consumers can still use the type `ActiveEntity` exported above for typing.
export const ActiveEntity = ActiveEntityWrapper as unknown as {
  new (raw: RawEntity): any;
};

// --- New Driver API --------------------------------------------------------
// Thing as EntityDriver — class-based driver integrating the Active model.
// Keeps functional exports above for backwards compatibility.
export class ThingDriver extends BaseDriver {
  constructor() {
    super('ThingDriver');
  }

  // Assemble a minimal world surface that processors can consume.
  assemble(input: ProcessorInputs) {
    return createWorld({
      type: 'system.World',
      name: 'Thing',
      horizon: { entity: { count: (input?.entities ?? []).length } },
    });
  }

  // Delegate helpers to the functional surface for determinism
  computeEntitySignature(e: Record<string, any>) {
    return computeEntitySignature(e);
  }
  extractEntityFacets(e: Record<string, any>) {
    return extractEntityFacets(e);
  }
  toActiveEntity(input: unknown) {
    return toActiveEntity(input);
  }
  fromFormEntity(raw: RawEntity) {
    return fromFormEntity(raw);
  }

  // BaseDriver Active conversion hooks (entity-focused)
  toActive(input: unknown) {
    return ActiveFactory.parseEntity(input ?? {});
  }
  fromActive(input: unknown) {
    return input;
  }

  // --- Engine bridge ------------------------------------------------------
  private createEngine(repo?: Repository<Entity>, bus?: EventBus) {
    // prefer provided repository, otherwise create a typed in-memory repo for EntitySchema
    const r: Repository<Entity> =
      repo ?? makeInMemoryRepository(EntitySchema);
    return new EntityEngine(r, bus);
  }

  private toActiveBatch(inputs: unknown[]): ActiveEntitySchemaType[] {
    return (inputs ?? []).map((i) => ActiveFactory.parseEntity(i ?? {}));
  }

  async processEntities(
    entities: unknown[],
    opts?: {
      repo?: Repository<Entity>;
      bus?: EventBus;
      particulars?: any[];
      context?: any;
    },
  ) {
    const engine = this.createEngine(opts?.repo, opts?.bus);
    const actives = this.toActiveBatch(entities);
    return engine.process(actives, opts?.particulars ?? [], opts?.context);
  }

  async commitEntities(
    entities: unknown[],
    opts?: {
      repo?: Repository<Entity>;
      bus?: EventBus;
      particulars?: any[];
      context?: any;
    },
  ) {
    const engine = this.createEngine(opts?.repo, opts?.bus);
    const actives = this.toActiveBatch(entities);
    const { actions, snapshot } = await engine.process(
      actives,
      opts?.particulars ?? [],
      opts?.context,
    );
    const commitResult = await engine.commit(actions, snapshot);
    return { actions, snapshot, commitResult } as const;
  }
}

// Default instance for class-based usage
export const DefaultThingDriver = new ThingDriver();

// Thin wrappers for convenience (functional style)
export async function processEntities(
  entities: unknown[],
  opts?: {
    repo?: Repository<Entity>;
    bus?: EventBus;
    particulars?: any[];
    context?: any;
  },
) {
  return DefaultThingDriver.processEntities(entities, opts);
}

export async function commitEntities(
  entities: unknown[],
  opts?: {
    repo?: Repository<Entity>;
    bus?: EventBus;
    particulars?: any[];
    context?: any;
  },
) {
  return DefaultThingDriver.commitEntities(entities, opts);
}
