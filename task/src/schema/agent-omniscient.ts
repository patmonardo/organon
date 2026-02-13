import { z } from 'zod';

import { ContextDocumentSchema } from './context-document';
import { KnowledgeUnitSchema } from './knowledge';

/**
 * Fact projection from a single Neo4j graph, viewed as operational facts.
 */
export const FactProjectionSchema = z.object({
  source: z.literal('fact-store').default('fact-store'),
  context: ContextDocumentSchema,
  entityIds: z.array(z.string()).default([]),
  propertyIds: z.array(z.string()).default([]),
  aspectIds: z.array(z.string()).default([]),
  metadata: z.record(z.string(), z.any()).optional(),
});
export type FactProjection = z.infer<typeof FactProjectionSchema>;

/**
 * Knowledge projection from the same Neo4j graph, viewed as epistemic units.
 */
export const KnowledgeProjectionSchema = z.object({
  source: z.literal('knowledge-store').default('knowledge-store'),
  units: z.array(KnowledgeUnitSchema).default([]),
  focusTopics: z.array(z.string()).default([]),
  metadata: z.record(z.string(), z.any()).optional(),
});
export type KnowledgeProjection = z.infer<typeof KnowledgeProjectionSchema>;

/**
 * Structural references to specification languages visible to the agent.
 * Kept transport-agnostic and package-decoupled (pure TS-JSON).
 */
export const SpecificationRefSchema = z.object({
  id: z.string(),
  kind: z.enum(['gdsl', 'sdsl']),
  title: z.string().optional(),
  version: z.string().optional(),
  tags: z.array(z.string()).optional(),
  metadata: z.record(z.string(), z.any()).optional(),
});
export type SpecificationRef = z.infer<typeof SpecificationRefSchema>;

/**
 * Unified omniscient context for the agent runtime.
 * Represents two projections (FactStore + KnowledgeStore) over one graph DB.
 */
export const AgentOmniscientContextSchema = z.object({
  id: z.string(),
  graphId: z.string(),
  timestamp: z.string(),
  factProjection: FactProjectionSchema,
  knowledgeProjection: KnowledgeProjectionSchema,
  specifications: z.array(SpecificationRefSchema).default([]),
  metadata: z.record(z.string(), z.any()).optional(),
});
export type AgentOmniscientContext = z.infer<
  typeof AgentOmniscientContextSchema
>;

/**
 * Canonical TS-JSON payload for agent loop bootstrapping from unified context.
 */
export const AgentOmniscientEnvelopeSchema = z.object({
  context: AgentOmniscientContextSchema,
  goal: z
    .object({
      id: z.string(),
      type: z.string(),
      description: z.string(),
    })
    .optional(),
  syscallSurface: z.record(z.string(), z.any()).optional(),
});
export type AgentOmniscientEnvelope = z.infer<
  typeof AgentOmniscientEnvelopeSchema
>;
