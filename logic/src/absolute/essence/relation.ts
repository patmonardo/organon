import * as active from '../../schema/active';
import BaseDriver from '../core/driver';
import type { ProcessorInputs } from '../core/contracts';
import { createWorld, type World } from '../../schema/world';
import type { Repository } from '../../repository/repo';
import { makeInMemoryRepository } from '../../repository/memory';
import { RelationSchema, type Relation } from '../../schema/relation';
import type { EventBus } from '../core/triad/bus';
import { RelationEngine } from '../../form/aspect/engine';

export class RelationDriver extends BaseDriver {
  constructor() {
    super('RelationDriver');
  }

  // Minimal world assembly for stage identification
  assemble(_input: ProcessorInputs): World {
    return createWorld({
      type: 'system.World',
      name: 'Relations',
      horizon: { stage: 'relations' },
    });
  }

  // Active conversions
  toActiveRelation(input: unknown): active.ActiveRelation {
    return active.ActiveFactory.parseRelation(input);
  }

  toActiveRelationBatch(inputs: unknown[] = []): active.ActiveRelation[] {
    return inputs.map((i) => this.toActiveRelation(i));
  }

  // Engine bridge
  private createRelationEngine(repo?: Repository<Relation>, bus?: EventBus) {
    // prefer provided repository, otherwise create an in-memory repo for RelationSchema
    const r: Repository<Relation> =
      repo ?? (makeInMemoryRepository(RelationSchema) as unknown as Repository<Relation>);
    return new RelationEngine(r, bus);
  }

  async processActiveRelations(
    relations: active.ActiveRelation[] = [],
    opts?: {
      repo?: Repository<Relation>;
      bus?: EventBus;
      particulars?: any[];
      context?: any;
    },
  ) {
    const engine = this.createRelationEngine(opts?.repo, opts?.bus);
    const list = active.parseActiveRelations(relations);
    return engine.process(list, opts?.particulars ?? [], opts?.context);
  }

  async commitActiveRelations(
    relations: active.ActiveRelation[] = [],
    opts?: {
      repo?: Repository<Relation>;
      bus?: EventBus;
      particulars?: any[];
      context?: any;
    },
  ) {
    const engine = this.createRelationEngine(opts?.repo, opts?.bus);
    const list = active.parseActiveRelations(relations);
    const { actions, snapshot } = await engine.process(
      list,
      opts?.particulars ?? [],
      opts?.context,
    );
    const commitResult = await engine.commit(actions, snapshot);
    return { actions, snapshot, commitResult } as const;
  }
}

export const DefaultRelationDriver = new RelationDriver();

export function toActiveRelation(input: unknown) {
  return DefaultRelationDriver.toActiveRelation(input);
}

export function toActiveRelationBatch(inputs: unknown[] = []) {
  return DefaultRelationDriver.toActiveRelationBatch(inputs);
}

export function processActiveRelations(
  relations: active.ActiveRelation[] = [],
  opts?: {
    repo?: Repository<Relation>;
    bus?: EventBus;
    particulars?: any[];
    context?: any;
  },
) {
  return DefaultRelationDriver.processActiveRelations(relations, opts);
}

export function commitActiveRelations(
  relations: active.ActiveRelation[] = [],
  opts?: {
    repo?: Repository<Relation>;
    bus?: EventBus;
    particulars?: any[];
    context?: any;
  },
) {
  return DefaultRelationDriver.commitActiveRelations(relations, opts);
}

export default DefaultRelationDriver;

// Carrier shape
export type ActiveRelation = active.ActiveRelation;

// Helpers and invariants for ActiveRelation (pure carrier)
export const DEFAULT_ACTIVE_TRUTH_THRESHOLD = 0.5;

const clamp01 = (n: unknown) => {
  const x = Number(n);
  return Number.isFinite(x) ? Math.max(0, Math.min(1, x)) : 0;
};

export function truthScore(rel: ActiveRelation): number {
  if (rel.revoked === true) return 0;
  if (rel.active === true) return 1;
  return clamp01(rel.confidence);
}

export function isActiveRelation(
  rel: ActiveRelation,
  threshold = DEFAULT_ACTIVE_TRUTH_THRESHOLD,
): boolean {
  if (rel.revoked === true) return false;
  if (rel.active === true) return true;
  return truthScore(rel) >= threshold;
}

export function assertActiveRelationInvariants(relations: ActiveRelation[]) {
  const violations: Array<{ rel: ActiveRelation; reason: string }> = [];
  for (const r of relations) {
    if (!r.particularityOf) {
      violations.push({ rel: r, reason: 'missing particularityOf' });
    }
    if (!r.source || !r.source.id || !r.target || !r.target.id) {
      violations.push({ rel: r, reason: 'missing endpoints' });
    }
    if (
      r.revoked !== true &&
      r.active !== true &&
      typeof r.confidence !== 'number'
    ) {
      violations.push({
        rel: r,
        reason: 'must have active=true or numeric confidence (unless revoked)',
      });
    }
  }

  if (violations.length === 0) return true;

  const msg = `ActiveRelation invariant violations: ${violations.length}`;
  if (
    process.env.NODE_ENV === 'test' ||
    process.env.NODE_ENV === 'development'
  ) {
    throw new Error(
      msg +
        ': ' +
        JSON.stringify(
          violations.map((v) => ({ id: v.rel.id, reason: v.reason })),
        ),
    );
  }
  // eslint-disable-next-line no-console
  console.warn(
    '[relation.assert] ' + msg,
    violations.map((v) => ({ id: v.rel.id, reason: v.reason })),
  );
  return false;
}

export function findActiveRelationsFor(
  absId: string,
  relations: ActiveRelation[],
  threshold = DEFAULT_ACTIVE_TRUTH_THRESHOLD,
) {
  return relations.filter(
    (r) => r.particularityOf === absId && isActiveRelation(r, threshold),
  );
}

// Back-compat helper used by orchestrator and some docs/tests
export function isRelationKindEssential(
  rel: { kind?: string } | ActiveRelation,
) {
  const k = (rel as any)?.kind;
  return k === 'essential' || k === 'relation';
}

// Ground weighting: combine confidence and optional weight (if present)
export function groundScore(rel: ActiveRelation): number {
  const base = truthScore(rel);
  const w = rel.weight !== undefined ? clamp01(rel.weight) : 1;
  return clamp01(base * w);
}

// Choose canonical relation for a given absolute id
export function chooseCanonicalTruth(
  absId: string,
  relations: ActiveRelation[],
): ActiveRelation | undefined {
  const candidates = relations.filter(
    (r) => r.particularityOf === absId && r.revoked !== true,
  );
  if (candidates.length === 0) return undefined;
  // Prefer explicit active
  const actives = candidates.filter((r) => r.active === true);
  if (actives.length > 0) {
    return actives.sort((a, b) => (a.id < b.id ? -1 : a.id > b.id ? 1 : 0))[0];
  }
  // Then highest truth score
  const scored = candidates
    .map((r) => ({ r, s: truthScore(r) }))
    .sort(
      (a, b) => b.s - a.s || (a.r.id < b.r.id ? -1 : a.r.id > b.r.id ? 1 : 0),
    );
  const top = scored[0];
  return top?.r;
}
