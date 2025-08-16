import type { Shape } from "../schema/shape";
import type { Context } from "../schema/context";
import type { Morph } from "../schema/morph";
import type { Entity } from "../schema/entity";
import type { Property } from "../schema/property";
import type { Property as ReflectProperty, Thing as ReflectThing, ReflectResult } from "./reflect";
import type { KriyaActionResult, Action } from "./action";
import type { Relation } from "../schema/relation";

export type Principles = {
  shapes: Shape[];
  contexts: Context[];
  morphs: Morph[];
};

export type EssenceGraph = {
  entities: Entity[];
  properties: Property[];
  relations: Relation[];
};

export type Projections = {
  indexes?: Record<string, unknown>;
  views?: Record<string, unknown>;
};

export type Controls = {
  actions: Array<{ kind: string; args?: unknown }>;
};

export type Work = {
  tasks: Array<{ id: string; kind: string; args?: unknown }>;
  workflow: { nodes: string[]; edges: Array<{ from: string; to: string }> };
};

export type KriyaOptions = {
  fixpointMaxIters?: number;
  budgetMs?: number;
  // pass-through options for reflect stage (e.g. { contextId })
  reflectOpts?: Record<string, unknown>;
};

export type KriyaResult = {
  graph: EssenceGraph;
  projections: Projections;
  controls: Controls;
  work: Work;
  // optional reflect results attached when opts.reflect is true and fns.reflect is provided
  reflect?: ReflectResult;
  // optional action results attached when fns.action is provided
  action?: KriyaActionResult;
};

export type StageFns = {
  seed: (p: Principles) => Promise<Pick<EssenceGraph, "entities">>;
  contextualize: (
    p: Principles,
    g: Pick<EssenceGraph, "entities">
  ) => Promise<Pick<EssenceGraph, "properties">>;
  // optional reflect stage: inspects seeded entities + contextualized properties and returns reflective facets/signatures
  reflect?: (
    things: ReflectThing[],
    properties: ReflectProperty[],
    opts?: KriyaOptions
  ) => Promise<ReflectResult>;
  ground: (
    p: Principles,
    g: Pick<EssenceGraph, "entities" | "properties">,
    opts?: KriyaOptions
  ) => Promise<Pick<EssenceGraph, "relations">>;
  // optional action stage: computes actions (reciprocal effects) from the graph and reflect facets
  action?: (
    graph: EssenceGraph,
    reflect?: ReflectResult,
    opts?: { threshold?: number; contextId?: string }
  ) => Promise<KriyaActionResult>;
  model: (g: EssenceGraph) => Promise<Projections>;
  control: (g: EssenceGraph, proj: Projections) => Promise<Controls>;
  plan: (ctrl: Controls) => Promise<Work>;
};

export async function runCycle(
  p: Principles,
  fns: StageFns,
  opts?: KriyaOptions
): Promise<KriyaResult> {
  // Ring 1 — Essence
  const seeded = await fns.seed(p); // Shape → Entity
  const ctxed = await fns.contextualize(p, seeded); // Context → Property
  let reflectResult: ReflectResult | undefined = undefined;
  // Optional reflect stage (compute Citta/vṛtti facets and signatures)
  if (typeof fns.reflect === 'function') {
    // adapt entities/properties to reflect's expected shapes
    // use runtime extraction and any casts to avoid tight coupling to schema types
    const seededAny = seeded as any;
    const ctxAny = ctxed as any;
    const things = (seededAny.entities || []).map((e: any) => {
      const id = e.id ?? e.shape?.core?.id ?? (e as any).core?.id;
      const type = e.type ?? e.shape?.core?.type ?? (e as any).core?.type;
      const essence = e.essence ?? e.ext?.essence ?? undefined;
      return { id, type, essence, properties: undefined };
    });
    const props = (ctxAny.properties || []).map((p0: any) => {
      const id = p0.id ?? p0.shape?.core?.id ?? (p0 as any).core?.id;
      const entityId = p0.entity ?? p0.entityId ?? p0.shape?.core?.entity ?? (p0 as any).owner;
      const entityType = (p0 as any).entityType ?? p0.shape?.core?.entityType ?? undefined;
      const key = p0.key ?? p0.shape?.core?.key ?? (p0 as any).name;
      const value = (p0 as any).value ?? (p0 as any).default ?? null;
      return { id, entity: { id: entityId, type: entityType }, key, value };
    });
    // call reflect with provided reflect options
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    reflectResult = await fns.reflect!(things as any, props as any, opts?.reflectOpts as any);
  }
  const grounded = await fns.ground(p, { ...seeded, ...ctxed }, opts); // Morph → Relation

  const graph: EssenceGraph = {
    entities: seeded.entities,
    properties: ctxed.properties,
    relations: grounded.relations,
  };

  // Rings 2–3 — read/plan layers
  const projections = await fns.model(graph);
  const controls = await fns.control(graph, projections);
  const work = await fns.plan(controls);

  // optional action stage (compute reciprocal effects)
  let actionResult: KriyaActionResult | undefined = undefined;
  if (typeof fns.action === 'function') {
    // pass reflectResult and lightweight opts through (use reflectOpts for context if available)
    const actionOpts = { threshold: (opts as any)?.actionThreshold, contextId: (opts as any)?.reflectOpts?.contextId };
    actionResult = await fns.action!(graph, reflectResult, actionOpts);
  }

  return { graph, projections, controls, work, reflect: reflectResult, action: actionResult };
}
