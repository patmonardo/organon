import { EntityId, FieldValue, FormId, PropertyId, PropertySpec, WorldId } from './types.js';
import { Principle } from './principle.js';

/**
 * World (Context:Property) — the middleware “platform” of possibilities.
 * Holds the intelligible set of properties available in this context.
 */
export interface World {
  id: WorldId;
  name: string;
  properties: Map<PropertyId, PropertySpec>;
  get(propId: PropertyId): PropertySpec | undefined;
}

/**
 * Form (Being-in-this-World) — a closure over a World that selects which
 * properties can/shall appear for Entities of this Form.
 *
 * open = properties from World are allowed unless explicitly excluded.
 * closed = only properties enumerated here are allowed.
 */
export interface Form {
  id: FormId;
  name: string;
  worldId: WorldId;
  open: boolean;
  allowed: Set<PropertyId>; // if closed, enumerates the shape; if open, acts as a whitelist override
}

export type FormPrinciple = Principle<Form, World>;

/**
 * Entity (Thing) — concrete appearance of a Form within its World.
 * Fields are assertions of properties with values constrained by the World.
 */
export interface Entity {
  id: EntityId;
  formId: FormId;
  worldId: WorldId;
  fields: Map<PropertyId, FieldValue[]>;
}

export const World = {
  create(id: WorldId, name: string, properties?: Iterable<PropertySpec>): World {
    const props = new Map<PropertyId, PropertySpec>();
    if (properties) {
      for (const p of properties) props.set(p.id, p);
    }
    return {
      id,
      name,
      properties: props,
      get: (propId: PropertyId) => props.get(propId),
    };
  },
};

export const Form = {
  create(id: FormId, name: string, world: World, opts?: { open?: boolean; allowed?: Iterable<PropertyId> }): FormPrinciple {
    return {
      immediate: {
        id,
        name,
        worldId: world.id,
        open: opts?.open ?? false,
        allowed: new Set(opts?.allowed ?? []),
      },
      mediacy: world,
    };
  },

  allows(principle: FormPrinciple, propId: PropertyId): boolean {
    const { immediate: form, mediacy: world } = principle;
    const hasInWorld = !!world.get(propId);
    if (!hasInWorld) return false;
    return form.open ? true : form.allowed.has(propId);
  },
};

export const Entity = {
  create(id: EntityId, form: FormPrinciple): Entity {
    return {
      id,
      formId: form.immediate.id,
      worldId: form.mediacy.id,
      fields: new Map(),
    };
  },
};
