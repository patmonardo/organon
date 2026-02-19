import { z } from 'zod';

export const engineLayerSchema = z.enum([
  'being-ingress',
  'fact-store',
  'relation-store',
  'knowledge-store',
  'modeling-layer',
]);

export const beingIngressPayloadSchema = z.object({
  id: z.string(),
  source: z.literal('mathematic-pure-reason'),
  payload: z.record(z.string(), z.unknown()),
  encoding: z.literal('json'),
  target: z.literal('being'),
});

export const factAssertionSchema = z.object({
  id: z.string(),
  lexeme: z.string(),
  proposition: z.string(),
  grounded: z.boolean(),
  conditionState: z.enum(['potential', 'conditioned', 'concrete']),
  provenance: z.object({
    stateId: z.string(),
    irId: z.string(),
  }),
});

export const relationAssertionSchema = z.object({
  id: z.string(),
  relationType: z.string(),
  subjectFactId: z.string(),
  objectFactId: z.string(),
  validatedByFactStore: z.literal(true),
  schemaReady: z.boolean(),
  provenance: z.object({
    stateId: z.string(),
    irId: z.string(),
  }),
});

export const knowledgeAssertionSchema = z.object({
  id: z.string(),
  concept: z.string(),
  appearanceBinding: z.array(z.string()),
  usesFactIds: z.array(z.string()),
  usesRelationIds: z.array(z.string()),
  mode: z.enum(['theoretical', 'practical', 'unified']),
});

export const modelingHandoffSchema = z.object({
  source: z.literal('relation-store'),
  destination: z.literal('modeling-layer'),
  relationIds: z.array(z.string()).min(1),
  schemaProjection: z.record(z.string(), z.unknown()),
});

export const engineBoundaryContractSchema = z.object({
  version: z.literal('v1'),
  principles: z.object({
    mathematicPureReasonIsGdsKernel: z.literal(true),
    sphereOfBeingHasNoStore: z.literal(true),
    beingInputIsJson: z.literal(true),
    essenceIsPersistence: z.literal(true),
    knowledgeStoreIsSpecialPersistence: z.literal(true),
    formDbIsFactStoreSchemaInternal: z.literal(true),
    essenceStoreLanguageIsDiscursive: z.literal(true),
    factStoreIsNotGenericDataStore: z.literal(true),
    factStoreSupportsRelationStoreAppearances: z.literal(true),
    factStoreFeedsRelationStore: z.literal(true),
    relationStoreFeedsKnowledgeStore: z.literal(true),
    relationStoreFeedsModelingLayer: z.literal(true),
    factStoreFeedsModelingLayer: z.literal(false),
  }),
});

export const defaultEngineBoundaryContract = engineBoundaryContractSchema.parse(
  {
    version: 'v1',
    principles: {
      mathematicPureReasonIsGdsKernel: true,
      sphereOfBeingHasNoStore: true,
      beingInputIsJson: true,
      essenceIsPersistence: true,
      knowledgeStoreIsSpecialPersistence: true,
      formDbIsFactStoreSchemaInternal: true,
      essenceStoreLanguageIsDiscursive: true,
      factStoreIsNotGenericDataStore: true,
      factStoreSupportsRelationStoreAppearances: true,
      factStoreFeedsRelationStore: true,
      relationStoreFeedsKnowledgeStore: true,
      relationStoreFeedsModelingLayer: true,
      factStoreFeedsModelingLayer: false,
    },
  },
);

export type EngineLayer = z.infer<typeof engineLayerSchema>;
export type BeingIngressPayload = z.infer<typeof beingIngressPayloadSchema>;
export type FactAssertion = z.infer<typeof factAssertionSchema>;
export type RelationAssertion = z.infer<typeof relationAssertionSchema>;
export type KnowledgeAssertion = z.infer<typeof knowledgeAssertionSchema>;
export type ModelingHandoff = z.infer<typeof modelingHandoffSchema>;
export type EngineBoundaryContract = z.infer<
  typeof engineBoundaryContractSchema
>;
