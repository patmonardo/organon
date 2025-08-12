import { z } from "zod";

// Primitives
export const Id = z.string().min(1, "id required");
export type Id = z.infer<typeof Id>;

export const Label = z.string().min(1, "label required");
export type Label = z.infer<typeof Label>;

export const Type = z.string().min(1, "type required");
export type Type = z.infer<typeof Type>;

export const IsoDateTime = z.string().datetime();
export type IsoDateTime = z.infer<typeof IsoDateTime>;

// Timestamps
export const Timestamps = z.object({
  createdAt: IsoDateTime.default(() => new Date().toISOString()),
  updatedAt: IsoDateTime.default(() => new Date().toISOString()),
});
export type Timestamps = z.infer<typeof Timestamps>;

// Helpers
export function stamp<T extends { createdAt?: string; updatedAt?: string }>(
  x: T
): T & Timestamps {
  const now = new Date().toISOString();
  return {
    createdAt: x.createdAt ?? now,
    updatedAt: x.updatedAt ?? now,
    ...x,
  } as T & Timestamps;
}
export function touch<T extends { updatedAt?: string }>(x: T): T {
  return { ...x, updatedAt: new Date().toISOString() };
}

// BaseCore (id + optional name/description + timestamps)
export const BaseCore = z.object({
  id: Id,
  type: Type,
  name: Label.optional(),
  description: z.string().optional(),
  createdAt: IsoDateTime.default(() => new Date().toISOString()),
  updatedAt: IsoDateTime.default(() => new Date().toISOString()),
});
export type BaseCore = z.infer<typeof BaseCore>;

// BaseState (common state)
export const BaseState = z.object({
  status: z.enum(["active", "archived", "deleted"]).default("active"),
  tags: z.array(Label).default([]),
  meta: z.record(z.unknown()).default({}),
});
export type BaseState = z.infer<typeof BaseState>;

// BaseShape (Core + State)
export const BaseShape = z.object({
  core: BaseCore,
  state: BaseState,
});
export type BaseShape = z.infer<typeof BaseShape>;

// BaseSchema (Shape + meta header)
export const BaseSchema = z.object({
  shape: BaseShape,
  revision: z.number().int().nonnegative().default(0),
  version: z.string().optional(),
  ext: z.record(z.unknown()).default({}),
});
export type Base = z.infer<typeof BaseSchema>;
