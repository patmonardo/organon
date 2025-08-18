import { z } from 'zod';

// Shared helpers
const clamp01 = (n: number) => (Number.isFinite(n) ? Math.min(1, Math.max(0, n)) : n);

const ProvenanceSchema = z.any().optional();

const ActivationSchema = z.object({
  active: z.boolean().optional(),
  revoked: z.boolean().optional(),
  confidence: z
    .number()
    .optional()
    .transform((v) => (typeof v === 'number' ? clamp01(v) : v)),
  weight: z.number().finite().optional(),
  provenance: ProvenanceSchema,
});

// Base
export const ActiveBaseSchema = z
  .object({
    id: z.string().min(1),
    kind: z.string().optional(),
  })
  .merge(ActivationSchema);

// ActiveShape (id is optional; engine may derive from name deterministically)
export const ActiveShapeSchema = z
  .object({
    id: z.string().min(1).optional(),
    kind: z.string().optional(),
    name: z.string().optional(),
    schema: z.any().optional(),
    particularityOf: z.string().optional(),
  })
  .merge(ActivationSchema);
export type ActiveShape = z.infer<typeof ActiveShapeSchema>;

// ActiveContext
export const ContextScopeSchema = z.union([
  z
    .object({
      world: z.array(z.string()).optional(),
      ids: z.array(z.string()).optional(),
    })
    .strict(),
  z.string(),
]);

export const ActiveContextSchema = ActiveBaseSchema.merge(
  z.object({
    name: z.string().optional(),
    particularityOf: z.string().optional(),
    scope: ContextScopeSchema.optional(),
    rules: z.any().optional(),
  }),
);
export type ActiveContext = z.infer<typeof ActiveContextSchema>;

// ActiveMorph
export const ActiveMorphSchema = ActiveBaseSchema.merge(
  z.object({
    particularityOf: z.string().optional(),
    transform: z.string().min(1),
    params: z.record(z.any()).optional(),
  }),
);
export type ActiveMorph = z.infer<typeof ActiveMorphSchema>;

// ActiveEntity
export const ActiveEntitySchema = z
  .object({
    id: z.string().min(1),
    entityType: z.string().min(1),
    particularityOf: z.string().optional(),
    labels: z.array(z.string()).optional(),
  })
  .merge(ActivationSchema);
export type ActiveEntity = z.infer<typeof ActiveEntitySchema>;

// ActiveProperty
export const ActivePropertySchema = z
  .object({
    id: z.string().min(1),
    subjectId: z.string().min(1),
    key: z.string().min(1),
    value: z.unknown(),
    dtype: z.string().optional(),
    particularityOf: z.string().optional(),
  })
  .merge(ActivationSchema);
export type ActiveProperty = z.infer<typeof ActivePropertySchema>;

// ActiveRelation
export const RelationEndpointSchema = z
  .object({
    id: z.string().min(1),
    type: z.string().optional(),
  })
  .strict()
  .optional();

export const ActiveRelationSchema = z
  .object({
    id: z.string().min(1),
    kind: z.enum(['relation', 'essential']).optional(),
    particularityOf: z.string().min(1),
    source: RelationEndpointSchema,
    target: RelationEndpointSchema,
    type: z.string().optional(),
  })
  .merge(ActivationSchema);
export type ActiveRelation = z.infer<typeof ActiveRelationSchema>;

// Batch validators
export const parseActiveShapes = (data: unknown) => z.array(ActiveShapeSchema).parse(data);
export const parseActiveContexts = (data: unknown) => z.array(ActiveContextSchema).parse(data);
export const parseActiveMorphs = (data: unknown) => z.array(ActiveMorphSchema).parse(data);
export const parseActiveEntities = (data: unknown) => z.array(ActiveEntitySchema).parse(data);
export const parseActiveProperties = (data: unknown) => z.array(ActivePropertySchema).parse(data);
export const parseActiveRelations = (data: unknown) => z.array(ActiveRelationSchema).parse(data);

// Single validators (safeParse variants provided for convenience)
export const safeParseActiveShape = (data: unknown) => ActiveShapeSchema.safeParse(data);
export const safeParseActiveContext = (data: unknown) => ActiveContextSchema.safeParse(data);
export const safeParseActiveMorph = (data: unknown) => ActiveMorphSchema.safeParse(data);
export const safeParseActiveEntity = (data: unknown) => ActiveEntitySchema.safeParse(data);
export const safeParseActiveProperty = (data: unknown) => ActivePropertySchema.safeParse(data);
export const safeParseActiveRelation = (data: unknown) => ActiveRelationSchema.safeParse(data);

// Normalizers
export function normalizeActivation<T extends { confidence?: number | undefined }>(
  obj: T,
): T {
  if (typeof obj.confidence === 'number') {
    (obj as any).confidence = clamp01(obj.confidence);
  }
  return obj;
}
