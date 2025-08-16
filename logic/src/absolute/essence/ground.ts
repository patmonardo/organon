import type { Entity } from '../../schema/entity';
import type { Property } from '../../schema/property';
import type { Relation } from '../../schema/relation';
import type { Morph } from '../../schema/morph';
import type { KriyaOptions } from '../core/kriya';

// Local loose types to avoid friction between strict schema shapes and
// runtime working objects created by the processor. These are intentionally
// permissive and used only inside this module where minimal fields are
// accessed (id, sourceId, targetId, type, provenance, etc.).
type LooseEntity = any;
type LooseProperty = any;
type LooseRelation = any;

/**
 * Ground stage
 * - Applies Morph rules to Entities+Properties to derive Relations and Properties
 * - Runs to a fixpoint (bounded by opts.fixpointMaxIters)
 *
 * Notes:
 * - This is a scaffold: concrete Morph.ruleSpec interpretation belongs here.
 * - All derived items should carry provenance (ruleId, contextVersion).
 */

export type WorkingGraph = {
  entities: LooseEntity[];
  properties: LooseProperty[]; // working properties (mutable in the working copy)
  relations: LooseRelation[]; // working relations (grows as rules derive edges)
};

export type GroundResult = {
  relations: LooseRelation[];
  properties: LooseProperty[];
};

export async function groundStage(
  principles: { morphs: Morph[] },
  graph: { entities: Entity[]; properties: Property[] },
  opts?: KriyaOptions,
): Promise<GroundResult> {
  const maxIters = opts?.fixpointMaxIters ?? 16;
  const morphs = principles.morphs ?? [];
  const working: WorkingGraph = {
    entities: [...graph.entities],
    properties: [...graph.properties],
    relations: [],
  };

  let iter = 0;
  let changed = true;

  // accessors to handle schema-backed shapes and runtime objects
  const getRelationId = (r: LooseRelation) => (r && ((r as any).id ?? (r as any).shape?.core?.id)) as string | undefined;
  const getPropertyId = (p: LooseProperty) => (p && ((p as any).id ?? (p as any).shape?.core?.id)) as string | undefined;
  const getPropertyEntityId = (p: LooseProperty) => (p && ((p as any).entityId ?? (p as any).entity?.id ?? (p as any).entity)) as string | undefined;

  while (changed && iter < maxIters) {
    iter += 1;
    changed = false;

    for (const m of morphs) {
      // applyMorphRule is a domain-specific implementation point.
      // It should return derived relations/properties for this iteration
      const { derivedRelations, derivedProperties } = await applyMorphRule(
        m,
        working,
        opts,
      );

      // merge relations: add new ones only
      for (const r of derivedRelations) {
  const rid = ((r as any)?.id ?? (r as any)?.shape?.core?.id) as string | undefined;
        const exists = working.relations.some(
          (ex) => getRelationId(ex) === rid,
        );
        if (!exists) {
          working.relations.push(r);
          changed = true;
        }
      }

      // merge properties: add or replace based on id (or entityId+key)
      for (const p of derivedProperties) {
  const pid = ((p as any)?.id ?? (p as any)?.shape?.core?.id) as string | undefined;
  const pEntityId = (p as any)?.entityId ?? (p as any)?.entity?.id ?? (p as any)?.entity;
  const pkey = (p as any)?.key;
        const idx = working.properties.findIndex((ex) => {
          return getPropertyId(ex) === pid || (getPropertyEntityId(ex) === pEntityId && (ex as any).key === pkey);
        });
        if (idx === -1) {
          working.properties.push(p);
          changed = true;
        } else {
          // simple equality check; replace if different (shallow)
          const existing = working.properties[idx];
          if (JSON.stringify(existing) !== JSON.stringify(p)) {
            working.properties[idx] = p;
            changed = true;
          }
        }
      }
    }
  }

  return {
    relations: working.relations,
    properties: working.properties,
  };
}

/**
 * Invariant check: every essential relation should point to an Absolute container
 * via `particularityOf`. Throws an Error in dev/test if violated.
 */
export function assertEssentialHasAbsolute(relations: LooseRelation[]) {
  const violations: Array<{ rel: LooseRelation; reason: string }> = [];
  for (const r of relations) {
    if (!isRelationKindEssential(r)) continue;
    const absId = (r as LooseRelation).particularityOf as string | undefined;
    if (!absId) {
      violations.push({ rel: r, reason: 'missing particularityOf' });
      continue;
    }
    const abs = relations.find((x) => (x as LooseRelation).id === absId);
    if (!abs) {
      violations.push({ rel: r, reason: `references missing absolute ${absId}` });
      continue;
    }
    if ((abs as LooseRelation).kind !== 'absolute') {
      violations.push({ rel: r, reason: `referenced relation ${absId} is not kind 'absolute'` });
    }
  }

  if (violations.length === 0) return true;

  const msg = `Invariant: ${violations.length} essential relation(s) with absolute-reference issues`;
  if (process.env.NODE_ENV === 'test' || process.env.NODE_ENV === 'development') {
  throw new Error(msg + ': ' + JSON.stringify(violations.map((v) => ({ id: (v.rel as LooseRelation).id, reason: v.reason }))));
  }
  // In production don't throw; log and return false so callers may degrade gracefully
  // eslint-disable-next-line no-console
  console.warn('[ground.assert] ' + msg, violations.map((v) => ({ id: (v.rel as LooseRelation).id, reason: v.reason })));
  return false;
}

/**
 * Relation-kind predicate: treat 'relation' as an alias for 'essential'
 * within the processor. This allows using the shorter term `Relation`
 * to refer to EssentialRelation in runtime checks and invariants.
 */
export function isRelationKindEssential(rel: LooseRelation) {
  const k = (rel as LooseRelation)?.kind;
  return k === 'essential' || k === 'relation';
}

/**
 * Backwards-compatible wrapper: assert that Relations (shorthand for
 * EssentialRelation) reference an Absolute container.
 */
export function assertRelationHasAbsolute(relations: LooseRelation[]) {
  return assertEssentialHasAbsolute(relations);
}

/**
 * Helper: find the Absolute container relation for a given essential relation id
 */
export function findAbsoluteFor(
  relationId: string,
  relations: LooseRelation[],
) {
  const rel = relations.find((r) => (r as LooseRelation).id === relationId);
  if (!rel) return undefined;
  const absId = (rel as LooseRelation).particularityOf as string | undefined;
  if (!absId) return undefined;
  return relations.find((r) => (r as LooseRelation).id === absId);
}

/**
 * Helper: list particular (essential) relations for a given Absolute id
 */
export function findParticularsFor(absId: string, relations: LooseRelation[]) {
  return relations.filter((r) => (r as LooseRelation).particularityOf === absId);
}

/**
 * Bridge: when a Ground represents a 'whole' (aggregates parts) create or mark
 * an Essential relation as the first moment of Essential Relation (Actuality).
 * - ground: relation that is kind==='absolute' and lists contributingConditionIds
 * - candidates: existing relations to match against or create a new one
 */
export function groundToEssentialBridge(
  ground: LooseRelation,
  candidates: LooseRelation[],
  opts?: { wholeThreshold?: number; confidenceBase?: number },
) {
  const threshold = opts?.wholeThreshold ?? 3;
  const confidenceBase = opts?.confidenceBase ?? 0.6;

  const parts: string[] = (ground as any).contributingConditionIds ?? [];
  const clusterSize = parts.length;
  if (clusterSize < threshold) return null;

  // simple signature: type + sourceId + targetId
  const sig = `${ground.type}:${ground.sourceId}->${ground.targetId}`;

  // find existing candidate with matching signature
  let match = candidates.find(
    (c) =>
      `${(c as any).type}:${(c as any).sourceId}->${(c as any).targetId}` ===
      sig,
  );

  const confidence = Math.min(1, confidenceBase + clusterSize / 10);

  if (!match) {
    // create deterministic id for essential relation
    const relId = `${(ground as any).id}:essential`;
    match = {
      id: relId,
      sourceId: (ground as any).sourceId,
      targetId: (ground as any).targetId,
      type: (ground as any).type.replace(/:absolute$/, '') || 'relation',
      kind: 'essential',
      provenance: {
        source: 'synth',
        timestamp: new Date().toISOString(),
        metaphysics: { role: 'essential', mode: 'relative' },
        modality: { kind: 'actual', confidence },
      },
      particularityOf: (ground as any).id,
    } as any as Relation;
    candidates.push(match);
  } else {
    // if found, ensure it's marked actual and linked
    (match as any).particularityOf = (ground as any).id;
    (match as any).provenance = {
      ...(match as any).provenance,
      modality: { kind: 'actual', confidence },
    };
  }

  // annotate ground's contributing ids if not present
  (ground as any).contributingConditionIds = parts;

  return match;
}

/**
 * Commit derived ground results to persistence via a small Triad-like proxy.
 * - triad.relation: Repository<Relation>
 * - triad.property: Repository<Property>
 * - triad.bus?: EventBus-like (publish(event))
 *
 * This function is intentionally small and defensive: it will create new docs
 * or update existing ones idempotently using repository semantics.
 */
export async function commitGroundResults(
  triad: any,
  results: GroundResult,
): Promise<void> {
  const { relation: relRepo, property: propRepo, bus } = triad as any;

  // persist relations
  for (const r of results.relations) {
    try {
      const existing = await relRepo.get(r.id);
      if (!existing) {
        await relRepo.create(r as any);
        bus?.publish?.({ kind: 'relation.created', payload: r });
      } else {
        // simple replace/update strategy
        await relRepo.update(r.id, () => r as any);
        bus?.publish?.({ kind: 'relation.updated', payload: r });
      }
    } catch (err) {
      // swallow errors for now; processors should be resilient
      // TODO: surface/log errors via a tracer or explicit error return
    }
  }

  // persist properties
  for (const p of results.properties) {
    try {
      const existing = await propRepo.get(p.id);
      if (!existing) {
        await propRepo.create(p as any);
        bus?.publish?.({ kind: 'property.created', payload: p });
      } else {
        await propRepo.update(p.id, () => p as any);
        bus?.publish?.({ kind: 'property.updated', payload: p });
      }
    } catch (err) {
      // swallow for now
    }
  }
}

/**
 * Richer ruleSpec types (scaffold)
 */
type PropertyPredicate =
  | { op: 'eq'; key: string; value: unknown }
  | { op: 'neq'; key: string; value: unknown }
  | { op: 'exists'; key: string }
  | { op: 'not_exists'; key: string }
  | { op: 'in'; key: string; values: unknown[] }
  | { op: 'gt' | 'lt' | 'gte' | 'lte'; key: string; value: number };

type EntitySelector =
  | { byId: string }
  | { byShape: string }
  | { byTag: string }
  | { all: true };

type TargetSpec =
  | { kind: 'fixed'; targetEntityId: string }
  | { kind: 'select'; selector: EntitySelector };

type DerivePropertyEffect = {
  key: string;
  value: unknown;
  status?: 'derived' | 'computed';
};

type BaseRuleSpec = {
  id?: string;
  priority?: number; // higher first
  condition?:
    | PropertyPredicate
    | { any: PropertyPredicate[] }
    | { all: PropertyPredicate[] }
    | { not: PropertyPredicate };
  source?: EntitySelector;
  target?: TargetSpec;
  idempotent?: boolean; // avoid duplicate effects when true
};

type DeriveRelationSpec = BaseRuleSpec & {
  kind: 'deriveRelation';
  relationType: string;
  directed?: boolean;
  setProperty?: DerivePropertyEffect;
};

type DerivePropertySpec = BaseRuleSpec & {
  kind: 'deriveProperty';
  setProperty: DerivePropertyEffect;
};

type RuleSpec = DeriveRelationSpec | DerivePropertySpec;

/**
 * Helper: evaluate a predicate against a property map
 */
function evalPredicate(
  pred: PropertyPredicate,
  propsByKey: Map<string, LooseProperty[]>,
): boolean {
  const list = propsByKey.get(pred.key) ?? [];
  switch (pred.op) {
    case 'exists':
      return list.length > 0;
    case 'not_exists':
      return list.length === 0;
    case 'eq':
      return list.some((p) => p.value === pred.value);
    case 'neq':
      return list.some((p) => p.value !== pred.value);
    case 'in':
      return list.some((p) => (pred.values as unknown[]).includes(p.value));
    case 'gt':
    case 'lt':
    case 'gte':
    case 'lte':
      return list.some((p) => {
        const v = Number(p.value);
        if (Number.isNaN(v)) return false;
        switch (pred.op) {
          case 'gt':
            return v > (pred as any).value;
          case 'lt':
            return v < (pred as any).value;
          case 'gte':
            return v >= (pred as any).value;
          case 'lte':
            return v <= (pred as any).value;
        }
        return false;
      });
    default:
      return false;
  }
}

/**
 * Build index maps for quicker matching
 */
function indexPropsByEntity(working: WorkingGraph) {
  const byEntity = new Map<string, LooseProperty[]>();
  const propsByKey = new Map<string, LooseProperty[]>();
  for (const p of working.properties) {
    if (!p) continue;
    // entity index
    const arr = byEntity.get(p.entityId) ?? [];
    arr.push(p);
    byEntity.set(p.entityId, arr);
    // key index
    const karr = propsByKey.get(p.key) ?? [];
    karr.push(p);
    propsByKey.set(p.key, karr);
  }
  return { byEntity, propsByKey };
}

/**
 * Select target entity ids according to TargetSpec
 */
function selectTargetIds(
  spec: TargetSpec | undefined,
  working: WorkingGraph,
): string[] {
  if (!spec) return [];
  if (spec.kind === 'fixed') return [spec.targetEntityId];
  const selector = spec.selector;
  if ('byId' in selector) return [selector.byId];
  if ('all' in selector && selector.all)
    return working.entities.map((e) => e.id);
  if ('byShape' in selector)
    return working.entities
      .filter((e) => (e as any).shape?.core?.type === selector.byShape)
      .map((e) => e.id);
  if ('byTag' in selector)
    return working.entities
      .filter((e) =>
        ((e as any).shape?.state?.tags ?? []).includes(selector.byTag),
      )
      .map((e) => e.id);
  return [];
}

/**
 * applyMorphRule
 * - Interpret a Morph.ruleSpec (now richer) against the working graph and return derived items.
 */
export async function applyMorphRule(
  morph: Morph,
  working: WorkingGraph,
  _opts?: KriyaOptions,
): Promise<{
  derivedRelations: LooseRelation[];
  derivedProperties: LooseProperty[];
}> {
  const spec = (morph as any)?.ruleSpec as RuleSpec | undefined;
  if (!spec) return { derivedRelations: [], derivedProperties: [] };

  const derivedRelations: Relation[] = [];
  const derivedProperties: Property[] = [];

  const { propsByKey } = indexPropsByEntity(working);

  // Evaluate condition helpers
  const conditionMatchesEntity = (entityId: string): boolean => {
    if (!spec.condition) return true;
    const c = spec.condition as any;
    if (c.any)
      return (c.any as PropertyPredicate[]).some((p) =>
        evalPredicate(p, propsByKey),
      );
    if (c.all)
      return (c.all as PropertyPredicate[]).every((p) =>
        evalPredicate(p, propsByKey),
      );
    if (c.not) return !evalPredicate(c.not as PropertyPredicate, propsByKey);
    return evalPredicate(c as PropertyPredicate, propsByKey);
  };

  // Determine candidate sources
  let sourceIds: string[] = [];
  if (spec.source) {
    const s = spec.source as any;
    if ('byId' in s) sourceIds = [s.byId];
    else if ('byShape' in s)
      sourceIds = working.entities
        .filter((e) => (e as any).shape?.core?.type === s.byShape)
        .map((e) => e.id);
    else if ('byTag' in s)
      sourceIds = working.entities
        .filter((e) =>
          ((e as any).shape?.state?.tags ?? []).includes(s.byTag),
        )
        .map((e) => e.id);
    else if ('all' in s && s.all) sourceIds = working.entities.map((e) => e.id);
  } else {
    sourceIds = working.entities.map((e) => e.id);
  }

  // Iterate candidate sources and apply effects
  for (const srcId of sourceIds) {
    // debug: log evaluation inputs for this morph/source
    try {
      // noop
    } catch (e) {}
    if (!conditionMatchesEntity(srcId)) continue;

    const targetIds = selectTargetIds(spec.target, working);
    for (const tgtId of targetIds) {
        try {
          // noop
        } catch (e) {}
      if (spec.kind === 'deriveRelation') {
  const relId = `${(spec as any).id ?? (morph as any).id}:${srcId}->${tgtId}`;
        // idempotence guard
        if (spec.idempotent) {
          const exists = working.relations.some((r) => r.id === relId);
          if (exists) continue;
        }
        const triggerProp = working.properties.find(
          (pp) =>
            pp.entityId === srcId && pp.key === (spec.condition as any)?.key,
        );

        const rel: any = {
          id: relId,
          sourceId: srcId,
          targetId: tgtId,
          type: spec.relationType,
          kind: 'essential',
          ruleId: (spec as any).id ?? (morph as any).id,
          // structured provenance with metaphysical annotations
          provenance: {
            ruleId: (spec as any).id ?? (morph as any).id,
            source: 'ground',
            viaTriggerPropertyId: triggerProp?.id ?? undefined,
            timestamp: new Date().toISOString(),
            metaphysics: {
              mode: 'skill',
              role: 'particular',
              faculty: 'ahamkara',
              intuition: 'inner',
            },
          },
          contextId: triggerProp?.contextId ?? null,
          contextVersion: triggerProp?.contextVersion ?? null,
          directed: spec.directed ?? true,
        };
        // Encode the metaphysical mapping: the Essential relation (particularity)
        // is also placed inside an Absolute relation (the containing universality).
        // We represent this by creating an additional 'absolute' relation and
        // linking the essential relation as its particularity. No schema change
        // is performed; this is additional runtime metadata on derived items.
        const absId = `${relId}:absolute`;
        const absRel: any = {
          id: absId,
          // keep same endpoints for the Absolute container
          sourceId: srcId,
          targetId: tgtId,
          // mark it as an absolute container; type names are advisory
           type: `${(spec as any).relationType}:absolute`,
          kind: 'absolute',
           ruleId: (spec as any).id ?? (morph as any).id,
          provenance: {
            ruleId: (spec as any).id ?? (morph as any).id,
            source: 'ground',
            containsParticularityId: relId,
            timestamp: new Date().toISOString(),
            metaphysics: {
              mode: 'vicara',
              role: 'absolute',
              faculty: 'buddhi',
              intuition: 'intellectual',
            },
          },
          // inherit context/provenance from triggering property when available
          contextId: triggerProp?.contextId ?? null,
          contextVersion: triggerProp?.contextVersion ?? null,
          directed: (spec as any).directed ?? true,
        };

        // link the essential relation back to its Absolute container
        rel.particularityOf = absId;

        // Reflection metadata: tag Absolute as vicara (reflection/universal)
        // and mark the Essential relation as the particular (skill/doing).
        absRel.meta = { vicara: true, dialectic: 'absolute' };
        rel.meta = { vicara: false, dialectic: 'particular' };

        // Attach advisory spectrum metadata when available from reflectStage
        try {
          const refl: any = (_opts as any)?.reflectResult;
          if (refl && refl.thingFacets) {
            const tf = refl.thingFacets[srcId];
            if (tf && (tf as any).spectrum) {
              rel.meta = { ...(rel.meta ?? {}), spectrum: (tf as any).spectrum };
              absRel.meta = { ...(absRel.meta ?? {}), spectrum: (tf as any).spectrum };
            }
          }
        } catch (err) {
          // non-fatal: spectrum is advisory
        }

        derivedRelations.push(rel as Relation);
        derivedRelations.push(absRel as Relation);
        try {
          // noop
        } catch (e) {}

        const sp = (spec as any).setProperty;
        if (sp) {
          const propId = `${relId}:prop:${sp.key}`;
          // derive property inheriting context/provenance from trigger (if any)
          const trigger = working.properties.find(
            (pp) =>
              pp.entityId === srcId && pp.key === (spec.condition as any)?.key,
          );
          const newProp: any = {
            id: propId,
            entityId: tgtId,
            key: sp.key,
            value: sp.value,
            contextId: trigger?.contextId ?? null,
            contextVersion: trigger?.contextVersion ?? null,
            status: sp.status ?? 'derived',
            provenance: {
              ruleId: (spec as any).id ?? (morph as any).id,
              source: 'ground',
              viaRelation: relId,
              viaTriggerPropertyId: trigger?.id ?? undefined,
              timestamp: new Date().toISOString(),
              metaphysics: {
                role: 'derivedProperty',
                origin: 'ground',
                faculty: 'ahamkara',
                intuition: 'inner',
              },
            },
          };
          derivedProperties.push(newProp as Property);
        }
      } else if (spec.kind === 'deriveProperty') {
        const propId = `${(spec as any).id ?? (morph as any).id}:${tgtId}:prop:${
          (spec as any).setProperty.key
        }`;
        if (spec.idempotent) {
          const exists = working.properties.some(
            (p) =>
              p.id === propId ||
              (p.entityId === tgtId &&
                p.key === spec.setProperty.key &&
                JSON.stringify(p.value) ===
                  JSON.stringify(spec.setProperty.value)),
          );
          if (exists) continue;
        }
        const newProp: any = {
          id: propId,
          entityId: tgtId,
          key: spec.setProperty.key,
          value: spec.setProperty.value,
          contextId: null,
          contextVersion: null,
            status: (spec as any).setProperty.status ?? 'derived',
            provenance: {
              ruleId: (spec as any).id ?? (morph as any).id,
            source: 'ground',
            timestamp: new Date().toISOString(),
            metaphysics: {
              role: 'derivedProperty',
              origin: 'ground',
              faculty: 'ahamkara',
              intuition: 'inner',
            },
          },
        };
        derivedProperties.push(newProp as Property);
      }
    }
  }

  return { derivedRelations, derivedProperties };
}
