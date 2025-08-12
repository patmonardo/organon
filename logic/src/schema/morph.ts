import { z } from "zod";
import { randomUUID } from "node:crypto";
import {
  BaseSchema,
  BaseState,
  BaseCore,
  Type,
  Id,
  Label,
  touch,
} from "./base";

// Core/state
export const MorphCore = BaseCore.extend({
  type: Type, // schema/category, e.g., "system.Morph"
  inputType: Label.default("FormShape"),
  outputType: Label.default("FormShape"),
  transformFn: z.string().optional(), // runtime entry (e.g., "@pkg/mod#symbol")
});
export type MorphCore = z.infer<typeof MorphCore>;

export const MorphState = BaseState.extend({
  optimized: z.boolean().optional(), // optional runtime hint
});
export type MorphState = z.infer<typeof MorphState>;

// Composition (optional, first pass)
export const MorphComposition = z
  .object({
    kind: z.enum(["single", "pipeline", "composite"]).default("single"),
    mode: z.enum(["sequential", "parallel"]).optional(),
    steps: z.array(Id).default([]), // references to other Morph ids
  })
  .default({ kind: "single", steps: [] });
export type MorphComposition = z.infer<typeof MorphComposition>;

// Shape: core/state + composition + config/meta
export const MorphShape = z.object({
  core: MorphCore,
  state: MorphState,
  composition: MorphComposition,
  config: z.record(z.unknown()).default({}),
  meta: z.record(z.unknown()).default({}),
});
export type MorphShape = z.infer<typeof MorphShape>;

// Schema
export const MorphSchema = BaseSchema.extend({
  shape: MorphShape,
});
export type Morph = z.infer<typeof MorphSchema>;

// Create/update
export function createMorph(input: {
  id?: string;
  type: z.input<typeof Type>;
  name?: string;
  description?: string;

  inputType?: z.input<typeof Label>;
  outputType?: z.input<typeof Label>;
  transformFn?: string;

  composition?: z.input<typeof MorphComposition>;
  config?: Record<string, unknown>;
  meta?: Record<string, unknown>;

  state?: z.input<typeof MorphState>;
  version?: string;
  ext?: Record<string, unknown>;
}): Morph {
  const core = MorphCore.parse({
    id: input.id ?? randomUUID(),
    type: input.type,
    name: input.name,
    description: input.description,
    inputType: input.inputType ?? "FormShape",
    outputType: input.outputType ?? "FormShape",
    transformFn: input.transformFn,
  });
  const state = MorphState.parse(input.state ?? {});
  const composition = MorphComposition.parse(input.composition ?? {});
  return MorphSchema.parse({
    shape: {
      core,
      state,
      composition,
      config: input.config ?? {},
      meta: input.meta ?? {},
    },
    revision: 0,
    version: input.version,
    ext: input.ext ?? {},
  });
}

export function updateMorph(
  current: Morph,
  patch: Partial<{
    core: Partial<z.input<typeof MorphCore>>;
    state: Partial<z.input<typeof MorphState>>;
    composition: z.input<typeof MorphComposition>;
    config: Record<string, unknown>;
    meta: Record<string, unknown>;
    version: string;
    ext: Record<string, unknown>;
  }>
): Morph {
  const core = MorphCore.parse(
    touch({ ...current.shape.core, ...(patch.core ?? {}) })
  );
  const state = MorphState.parse({
    ...current.shape.state,
    ...(patch.state ?? {}),
  });
  const composition =
    patch.composition !== undefined
      ? MorphComposition.parse(patch.composition)
      : current.shape.composition;

  return MorphSchema.parse({
    ...current,
    shape: {
      core,
      state,
      composition,
      config: patch.config ?? current.shape.config,
      meta: patch.meta ?? current.shape.meta,
    },
    revision: current.revision + 1,
    version: patch.version ?? current.version,
    ext: { ...current.ext, ...(patch.ext ?? {}) },
  });
}

// Ergonomics
export function defineMorph(config: {
  id?: string;
  type: z.input<typeof Type>;
  name?: string;
  description?: string;
  transformFn?: string;
  inputType?: z.input<typeof Label>;
  outputType?: z.input<typeof Label>;
  config?: Record<string, unknown>;
  meta?: Record<string, unknown>;
}): Morph {
  return createMorph({
    ...config,
    composition: { kind: "single", steps: [] },
  });
}

export function defineMorphPipeline(
  id: string,
  name: string,
  stepIds: z.input<typeof Id>[],
  options?: {
    type?: z.input<typeof Type>;
    description?: string;
    inputType?: z.input<typeof Label>;
    outputType?: z.input<typeof Label>;
    transformFn?: string;
    config?: Record<string, unknown>;
    meta?: Record<string, unknown>;
    version?: string;
    ext?: Record<string, unknown>;
  }
): Morph {
  return createMorph({
    id,
    type: options?.type ?? ("system.Morph" as z.input<typeof Type>),
    name,
    description: options?.description,
    inputType: options?.inputType ?? "FormShape",
    outputType: options?.outputType ?? "FormShape",
    transformFn: options?.transformFn,
    composition: { kind: "pipeline", mode: "sequential", steps: stepIds },
    config: options?.config,
    meta: options?.meta,
    version: options?.version,
    ext: options?.ext,
  });
}
