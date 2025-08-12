import { z } from "zod";
import { randomUUID } from "node:crypto";
import { BaseSchema, BaseState, BaseCore, Type, Id, touch } from "./base";
import { EntityRef } from "./entity";

// Value typing (optional hint for engines)
export const ValueType = z.enum(["string", "number", "boolean", "date", "object", "array"]);
export type ValueType = z.infer<typeof ValueType>;

// Core/state
export const PropertyCore = BaseCore.extend({
  type: Type,          // schema/category, e.g., "system.Property"
  key: z.string(),     // property key (required)
});
export type PropertyCore = z.infer<typeof PropertyCore>;

export const PropertyState = BaseState;
export type PropertyState = z.infer<typeof PropertyState>;

// Shape: core/state + bindings and value
export const PropertyShape = z.object({
  core: PropertyCore,
  state: PropertyState,

  // Context membership
  contextId: Id,

  // Binding to a subject (first pass: either entity or relation)
  entity: EntityRef.optional(),
  relationId: Id.optional(),

  // Value and typing
  value: z.unknown().optional(),
  valueType: ValueType.optional(),
});
export type PropertyShape = z.infer<typeof PropertyShape>;

// Schema
export const PropertySchema = BaseSchema.extend({
  shape: PropertyShape,
});
export type Property = z.infer<typeof PropertySchema>;

// Create/update
export function createProperty(input: {
  id?: string;
  type: z.input<typeof Type>;
  key: string;

  contextId: z.input<typeof Id>;
  entity?: z.input<typeof EntityRef>;
  relationId?: z.input<typeof Id>;

  name?: string;
  description?: string;

  state?: z.input<typeof PropertyState>;
  value?: unknown;
  valueType?: z.input<typeof ValueType>;
  ext?: Record<string, unknown>;
  version?: string;
}): Property {
  const core = PropertyCore.parse({
    id: input.id ?? randomUUID(),
    type: input.type,
    key: input.key,
    name: input.name,
    description: input.description,
  });
  const state = PropertyState.parse(input.state ?? {});
  return PropertySchema.parse({
    shape: {
      core,
      state,
      contextId: input.contextId,
      entity: input.entity ? EntityRef.parse(input.entity) : undefined,
      relationId: input.relationId,
      value: input.value,
      valueType: input.valueType,
    },
    revision: 0,
    version: input.version,
    ext: input.ext ?? {},
  });
}

export function updateProperty(
  current: Property,
  patch: Partial<{
    core: Partial<z.input<typeof PropertyCore>>;
    state: Partial<z.input<typeof PropertyState>>;
    contextId: z.input<typeof Id>;
    entity: z.input<typeof EntityRef> | null;     // null to clear
    relationId: z.input<typeof Id> | null;        // null to clear
    value: unknown;                                // explicit undefined should clear
    valueType: z.input<typeof ValueType>;         // explicit undefined should clear
    version: string;
    ext: Record<string, unknown>;
  }>,
): Property {
  const core = PropertyCore.parse(touch({ ...current.shape.core, ...(patch.core ?? {}) }));
  const state = PropertyState.parse({ ...current.shape.state, ...(patch.state ?? {}) });

  const nextEntity =
    patch.entity === null ? undefined :
    patch.entity !== undefined ? EntityRef.parse(patch.entity) :
    current.shape.entity;

  const nextRelationId =
    patch.relationId === null ? undefined :
    patch.relationId !== undefined ? patch.relationId :
    current.shape.relationId;

  // honor explicit undefined to clear
  const nextValue = ("value" in patch) ? patch.value : current.shape.value;
  const nextValueType = ("valueType" in patch) ? patch.valueType : current.shape.valueType;

  return PropertySchema.parse({
    ...current,
    shape: {
      core,
      state,
      contextId: patch.contextId ?? current.shape.contextId,
      entity: nextEntity,
      relationId: nextRelationId,
      value: nextValue,
      valueType: nextValueType,
    },
    revision: current.revision + 1,
    version: patch.version ?? current.version,
    ext: { ...current.ext, ...(patch.ext ?? {}) },
  });
}

// Ergonomics
export function bindPropertyToEntity(p: Property, entity: z.input<typeof EntityRef>): Property {
  return updateProperty(p, { entity, relationId: null });
}
export function bindPropertyToRelation(p: Property, relationId: z.input<typeof Id>): Property {
  return updateProperty(p, { entity: null, relationId });
}
