import { z } from 'zod'

export const TokenSchema = z.object({
  id: z.string(),
  token: z.string(),
  arity: z.number().optional(),
  doc: z.string().optional(),
})

export const ChunkSchema = z.object({
  id: z.string(),
  title: z.string().optional(),
  text: z.string().optional(),
  summary: z.string().optional(),
  source: z.string().optional(),
})

export const LinkSchema = z.object({
  // Domain-level relationship
  kind: z.enum(['witness', 'semantic', 'annotation']).default('semantic'),
  type: z.string(),           // e.g., 'supports', 'contradicts', 'tags', 'refers-to'
  from: z.string(),           // id (HLO/term/chunk/etc.)
  to: z.string(),             // id
  props: z.record(z.unknown()).optional(),
})

export const HLOSchema = z.object({
  id: z.string(),
  chunkId: z.string().optional(),
  label: z.string().optional(),
  digest: z.union([z.string(), z.array(z.string())]).optional(),
  clauses: z.array(z.string()).optional(),
  tokens: z.array(z.string()).optional(),

  // Preferred: links
  links: z.array(LinkSchema).optional(),

  // Legacy (DEPRECATED): witnessEdges -> links(kind='witness')
  witnessEdges: z
    .array(
      z.object({
        type: z.string(),
        from: z.string(),
        to: z.string(),
        props: z.record(z.unknown()).optional(),
      }),
    )
    .optional(),
})
  // Normalize legacy witnessEdges into links on parse
  .transform((hlo) => {
    if (!hlo.witnessEdges || hlo.witnessEdges.length === 0) return hlo
    const legacy = hlo.witnessEdges.map((e) => ({
      kind: 'witness' as const,
      type: e.type,
      from: e.from,
      to: e.to,
      props: e.props,
    }))
    const links = [...(hlo.links ?? []), ...legacy]
    const { witnessEdges, ...rest } = hlo as any
    return { ...rest, links }
  })

export const DatasetManifestSchema = z.object({
  id: z.string(),
  title: z.string().optional(),
  provenance: z.record(z.unknown()).optional(),
  assets: z.array(z.string()).optional(), // preferred
  chunks: z.array(z.object({ id: z.string().optional(), source: z.string().optional() })).optional(), // legacy
  hlos: z.array(HLOSchema).optional(),
  terms: z.array(z.record(z.unknown())).optional(), // swap in TermSchema when available
  signatureTokens: z.array(z.string()).optional(),
})

export type Token = z.infer<typeof TokenSchema>
export type Chunk = z.infer<typeof ChunkSchema>
export type Link = z.infer<typeof LinkSchema>
export type HLO = z.infer<typeof HLOSchema>
export type DatasetManifest = z.infer<typeof DatasetManifestSchema>
