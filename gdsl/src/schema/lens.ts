import { z } from 'zod'

export const LensTermSchema = z.object({
  id: z.string(),
  label: z.string(),
  aliases: z.array(z.string()).optional(),
  desc: z.string().optional(),
  tags: z.record(z.string()).optional(),
})

export const LensEdgeSchema = z.object({
  type: z.string(),
  from: z.string(),
  to: z.string(),
  props: z.record(z.unknown()).optional(),
})

export const LensSchema = z.object({
  id: z.string(),
  title: z.string().optional(),
  terms: z.array(LensTermSchema).default([]),
  edges: z.array(LensEdgeSchema).default([]),
  refs: z.array(z.string()).optional(),
  provenance: z.record(z.unknown()).optional(),
})

export type Lens = z.infer<typeof LensSchema>
export type LensTerm = z.infer<typeof LensTermSchema>
export type LensEdge = z.infer<typeof LensEdgeSchema>

export function validateLens(lens: unknown): Lens {
  return LensSchema.parse(lens)
}

/** Factory: create a minimal validated Lens object */
export function createLens(id: string, opts?: Partial<Omit<Lens, 'id' | 'terms' | 'edges'>> & { terms?: LensTerm[]; edges?: LensEdge[] }): Lens {
  const base: Lens = {
    id,
    title: opts?.title,
    refs: opts?.refs,
    provenance: opts?.provenance,
    terms: opts?.terms ?? [],
    edges: opts?.edges ?? [],
  } as Lens
  return validateLens(base)
}

/** Small updater: apply partial patch and revalidate */
export function updateLens(lens: Lens, patch: Partial<Lens>): Lens {
  const merged = { ...lens, ...patch, terms: patch.terms ?? lens.terms, edges: patch.edges ?? lens.edges }
  return validateLens(merged)
}

/** Create a normalized lens id. Idempotent for inputs already prefixed with 'lens:' */
export function makeLensId(s: string) {
  const id = String(s ?? '').trim()
  if (id === '') throw new Error('makeLensId: empty id')
  if (id.startsWith('lens:')) return id
  return 'lens:' + id.replace(/\s+/g, '-').replace(/[\/:]+/g, '-').toLowerCase()
}
