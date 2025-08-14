import { z } from 'zod';
import { randomUUID } from 'node:crypto';
import { BaseSchema, BaseState, BaseCore, Type, Id, touch } from './base';

// Ref = type + id
export const EntityRef = z.object({
  id: Id,
  type: Type,
});
export type EntityRef = z.infer<typeof EntityRef>;
export const isEntityRef = (v: unknown): v is EntityRef => {
  const r = EntityRef.safeParse(v);
  return r.success;
};

// Specialize core/state for Entity
export const EntityCore = BaseCore.extend({
  type: Type, // required for membership/category
});
export type EntityCore = z.infer<typeof EntityCore>;

export const EntityState = BaseState; // reuse status/tags/meta for now
export type EntityState = z.infer<typeof EntityState>;

// Shape and Schema
export const EntityShape = z.object({
  core: EntityCore,
  state: EntityState,
});
export type EntityShape = z.infer<typeof EntityShape>;

// Override BaseSchema.shape with EntityShape (keeps revision/version/ext)
export const EntitySchema = BaseSchema.extend({
  shape: EntityShape,
});
export type Entity = z.infer<typeof EntitySchema>;

// Constructors (schema-safe)
export function createEntity(input: {
  id?: string;
  type: z.input<typeof Type>;
  name?: string;
  description?: string;
  state?: z.input<typeof EntityState>;
  ext?: Record<string, unknown>;
  version?: string;
}): Entity {
  // Let BaseCore defaults set timestamps via Zod
  const core = EntityCore.parse({
    id: input.id ?? randomUUID(),
    type: input.type,
    name: input.name,
    description: input.description,
  });

  const state = EntityState.parse(input.state ?? {});
  return EntitySchema.parse({
    shape: { core, state },
    revision: 0,
    version: input.version,
    ext: input.ext ?? {},
  });
}
export function updateEntity(
  current: Entity,
  patch: Partial<{
    core: Partial<z.input<typeof EntityCore>>;
    state: Partial<z.input<typeof EntityState>>;
    version: string;
    ext: Record<string, unknown>;
  }>,
): Entity {
  const core = EntityCore.parse(
    touch({ ...current.shape.core, ...(patch.core ?? {}) }),
  );
  const state = EntityState.parse({
    ...current.shape.state,
    ...(patch.state ?? {}),
  });
  return EntitySchema.parse({
    ...current,
    shape: { core, state },
    revision: current.revision + 1,
    version: patch.version ?? current.version,
    ext: { ...current.ext, ...(patch.ext ?? {}) },
  });
}

// Ergonomics
export function createEntityRef(
  e: z.input<typeof EntitySchema> | Entity | EntityRef,
): EntityRef {
  if (isEntityRef(e)) return e;
  const ent = EntitySchema.parse(e as any);
  return { id: ent.shape.core.id, type: ent.shape.core.type };
}

export type EntityKey = string;

export function formatEntityKey(x: Entity | EntityRef): EntityKey {
  const ref =
    'shape' in (x as any) ? createEntityRef(x as Entity) : (x as EntityRef);
  return `${ref.type}:${ref.id}`;
}

export function parseEntityKey(key: string): EntityRef {
  // Split at the last colon to allow types that contain ":" (e.g., "System::Engine")
  const idx = key.lastIndexOf(':');
  const type = key.slice(0, idx);
  const id = key.slice(idx + 1);
  return EntityRef.parse({ type, id });
}
