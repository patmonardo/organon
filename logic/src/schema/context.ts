import { z } from "zod";
import { randomUUID } from "crypto";
import { BaseSchema, BaseState, BaseCore, Type, Id, touch } from "./base";
import { EntityRef } from "./entity";

// Core/state
export const ContextCore = BaseCore.extend({
  type: Type,
});
export type ContextCore = z.infer<typeof ContextCore>;

export const ContextState = BaseState;
export type ContextState = z.infer<typeof ContextState>;

// Shape: core/state + memberships
export const ContextShape = z.object({
  core: ContextCore,
  state: ContextState,
  entities: z.array(EntityRef).default([]),
  relations: z.array(Id).default([]), // relation ids (first pass)
});
export type ContextShape = z.infer<typeof ContextShape>;

// Schema
export const ContextSchema = BaseSchema.extend({
  shape: ContextShape,
});
export type Context = z.infer<typeof ContextSchema>;

// Create/update
export function createContext(input: {
  id?: string;
  type: z.input<typeof Type>;
  name?: string;
  description?: string;
  state?: z.input<typeof ContextState>;
  entities?: z.input<typeof EntityRef>[];
  relations?: z.input<typeof Id>[];
  ext?: Record<string, unknown>;
  version?: string;
}): Context {
  const core = ContextCore.parse({
    id: input.id ?? randomUUID(),
    type: input.type,
    name: input.name,
    description: input.description,
  });
  const state = ContextState.parse(input.state ?? {});
  return ContextSchema.parse({
    shape: {
      core,
      state,
      entities: input.entities ?? [],
      relations: input.relations ?? [],
    },
    revision: 0,
    version: input.version,
    ext: input.ext ?? {},
  });
}

export function updateContext(
  current: Context,
  patch: Partial<{
    core: Partial<z.input<typeof ContextCore>>;
    state: Partial<z.input<typeof ContextState>>;
    entities: z.input<typeof EntityRef>[];
    relations: z.input<typeof Id>[];
    version: string;
    ext: Record<string, unknown>;
  }>
): Context {
  const core = ContextCore.parse(
    touch({ ...current.shape.core, ...(patch.core ?? {}) })
  );
  const state = ContextState.parse({
    ...current.shape.state,
    ...(patch.state ?? {}),
  });
  const entities =
    patch.entities !== undefined
      ? z.array(EntityRef).parse(patch.entities)
      : current.shape.entities;
  const relations =
    patch.relations !== undefined
      ? z.array(Id).parse(patch.relations)
      : current.shape.relations;

  return ContextSchema.parse({
    ...current,
    shape: { core, state, entities, relations },
    revision: current.revision + 1,
    version: patch.version ?? current.version,
    ext: { ...current.ext, ...(patch.ext ?? {}) },
  });
}

// Ergonomics
export function addEntitiesToContext(
  ctx: Context,
  entities: z.input<typeof EntityRef>[]
): Context {
  const exists = new Set(ctx.shape.entities.map((e) => `${e.type}:${e.id}`));
  const toAdd = (entities ?? []).filter(
    (e) => !exists.has(`${e.type}:${e.id}`)
  );
  if (toAdd.length === 0) return ctx;
  return updateContext(ctx, { entities: [...ctx.shape.entities, ...toAdd] });
}

export function addRelationsToContext(
  ctx: Context,
  relationIds: z.input<typeof Id>[]
): Context {
  const exists = new Set(ctx.shape.relations);
  const toAdd = (relationIds ?? []).filter((id) => !exists.has(id));
  if (toAdd.length === 0) return ctx;
  return updateContext(ctx, { relations: [...ctx.shape.relations, ...toAdd] });
}
