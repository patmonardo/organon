import { z } from 'zod';

export const DialecticalMomentSchema = z.enum(['INTUITIVE', 'CONCEPTUAL', 'REFLECTIVE_SCIENCE']);
export type DialecticalMoment = z.infer<typeof DialecticalMomentSchema>;

export const IntegratedLineRangeSchema = z.object({
  start: z.number().int().positive(),
  end: z.number().int().positive(),
});

export const IntegratedTopicChunkSchema = z.object({
  id: z.string(),
  title: z.string(),
  sourceId: z.string(),
  sourceFile: z.string(),
  lineRange: IntegratedLineRangeSchema,
  description: z.string(),
  keyPoints: z.array(z.string()),
  orderInSource: z.number().int().positive(),
  globalOrder: z.number().int().positive(),
  sourceText: z.string(),
  dialecticalRole: DialecticalMomentSchema,
  tags: z.array(z.string()).default([]),
});

export type IntegratedTopicChunk = z.infer<typeof IntegratedTopicChunkSchema>;

export const IntegratedSourceDocumentSchema = z.object({
  id: z.string(),
  title: z.string(),
  sourceFile: z.string(),
  totalLines: z.number().int().nonnegative(),
  chunks: z.array(IntegratedTopicChunkSchema),
});

export type IntegratedSourceDocument = z.infer<
  typeof IntegratedSourceDocumentSchema
>;

export const IntegratedTopicTraceTypeSchema = z.enum([
  'NEXT',
  'NEGATES',
  'SUBLATES',
  'REFLECTS',
  'MEDIATES',
  'SPIRALS_TO',
  'LAYER_NEGATION',
]);

export type IntegratedTopicTraceType = z.infer<
  typeof IntegratedTopicTraceTypeSchema
>;

export const IntegratedTopicTraceSchema = z.object({
  fromChunkId: z.string(),
  toChunkId: z.string(),
  type: IntegratedTopicTraceTypeSchema,
  reason: z.string(),
});

export type IntegratedTopicTrace = z.infer<typeof IntegratedTopicTraceSchema>;

export const ConsciousnessLayerKindSchema = z.enum([
  'PRINCIPLE',
  'LAW',
  'SCIENCE',
]);

export type ConsciousnessLayerKind = z.infer<
  typeof ConsciousnessLayerKindSchema
>;

export const ConsciousnessLayerSchema = z.object({
  id: z.string(),
  title: z.string(),
  kind: ConsciousnessLayerKindSchema,
  description: z.string(),
  chunkIds: z.array(z.string()),
});

export type ConsciousnessLayer = z.infer<typeof ConsciousnessLayerSchema>;

export const ConsciousnessLayerRelationSchema = z.object({
  fromLayerId: z.string(),
  toLayerId: z.string(),
  operator: z.enum(['SUBLATION', 'NEGATION']),
  description: z.string(),
});

export type ConsciousnessLayerRelation = z.infer<
  typeof ConsciousnessLayerRelationSchema
>;

export const ConsciousnessDoctrineSchema = z.object({
  layers: z.array(ConsciousnessLayerSchema),
  relations: z.array(ConsciousnessLayerRelationSchema),
});

export type ConsciousnessDoctrine = z.infer<typeof ConsciousnessDoctrineSchema>;

export const IntegratedTriadicProtocolSchema = z.object({
  root: z.literal('REFLECTIVE_SCIENCE'),
  members: z.tuple([z.literal('INTUITIVE'), z.literal('CONCEPTUAL')]),
  relation: z.literal('SUBLATED_MEMBERSHIP'),
});

export type IntegratedTriadicProtocol = z.infer<typeof IntegratedTriadicProtocolSchema>;

export const IntegratedTopicMapIRSchema = z.object({
  id: z.string(),
  mode: z.enum(['debug', 'production']),
  title: z.string(),
  section: z.string(),
  sourceDocuments: z.array(IntegratedSourceDocumentSchema),
  traces: z.array(IntegratedTopicTraceSchema),
  doctrine: ConsciousnessDoctrineSchema,
  triadicProtocol: IntegratedTriadicProtocolSchema,
  metadata: z.object({
    totalSources: z.number().int().nonnegative(),
    totalChunks: z.number().int().nonnegative(),
    generatedAt: z.string(),
  }),
});

export type IntegratedTopicMapIR = z.infer<typeof IntegratedTopicMapIRSchema>;
