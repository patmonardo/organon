import { z } from 'zod'
import { FacetsSchema } from './signature' // <- use signature schema here

export const NodeRowSchema = z.object({
  id: z.string(),
  labels: z.array(z.string()).optional(),
  props: z.record(z.unknown()).optional(),
})

export const EdgeRowSchema = z.object({
  type: z.string(),
  from: z.string(),
  to: z.string(),
  props: z.record(z.unknown()).optional(),
})

export const ClauseRowSchema = z.object({
  id: z.string(),
  hloId: z.string(),
  raw: z.string(),
  kind: z.enum(['assert', 'tag', 'annotate', 'unknown']),
})

export const GraphArtifactSchema = z.object({
  dataset: z.string(),
  nodes: z.array(NodeRowSchema),
  edges: z.array(EdgeRowSchema),
  clauses: z.array(ClauseRowSchema).optional(),
  tokens: z.array(z.string()).optional(),
  terms: z.array(z.record(z.unknown())).optional(),
  counts: z.record(z.number()).optional(),
  signatures: FacetsSchema.optional(), // <- canonical facets/signatures
})

export type NodeRow = z.infer<typeof NodeRowSchema>
export type EdgeRow = z.infer<typeof EdgeRowSchema>
export type ClauseRow = z.infer<typeof ClauseRowSchema>
export type GraphArtifact = z.infer<typeof GraphArtifactSchema>
