import { z } from 'zod'

export const TopicTermSchema = z.object({
  id: z.string(),
  label: z.string(),
  aliases: z.array(z.string()).optional(),
  desc: z.string().optional(),
  tags: z.record(z.string()).optional(),      // free key/value tags
  refs: z.array(z.string()).optional(),       // cross-refs (URLs, ids)
})

export const TopicEdgeSchema = z.object({
  type: z.string(),                            // e.g., 'broader', 'related', 'part-of'
  from: z.string(),                            // term id
  to: z.string(),                              // term id
  props: z.record(z.unknown()).optional(),
})

export const SignatureTokenSchema = z.object({
  token: z.string(),                           // symbol in the signature
  weight: z.number().default(1),
})

export const TopicSchema = z.object({
  id: z.string(),
  title: z.string().optional(),
  description: z.string().optional(),
  terms: z.array(TopicTermSchema).default([]),
  edges: z.array(TopicEdgeSchema).default([]),
  signatures: z.record(z.array(SignatureTokenSchema)).default({}),  // name -> weighted tokens
  provenance: z.record(z.unknown()).optional(),
})

export type TopicTerm = z.infer<typeof TopicTermSchema>
export type TopicEdge = z.infer<typeof TopicEdgeSchema>
export type Topic = z.infer<typeof TopicSchema>
