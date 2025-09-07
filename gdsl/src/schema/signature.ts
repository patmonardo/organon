import { z } from 'zod';

export const SignatureTokenSchema = z.object({
  token: z.string(),
  weight: z.number().optional(),
});

export const FacetsSchema = z.record(z.array(SignatureTokenSchema)).default({});

export type SignatureToken = z.infer<typeof SignatureTokenSchema>;
export type Facets = z.infer<typeof FacetsSchema>;

/** Validate/normalize a facets object */
export function validateFacets(obj: unknown): Facets {
  return FacetsSchema.parse(obj);
}

/** Normalize a single facet array: coalesce duplicate tokens summing weights */
export function normalizeFacetArray(
  arr: SignatureToken[] = [],
): SignatureToken[] {
  const m = new Map<string, number>();
  for (const it of arr) {
    const tok = String(it.token);
    const w = Number.isFinite(Number(it.weight)) ? Number(it.weight) : 1;
    m.set(tok, (m.get(tok) ?? 0) + w);
  }
  return Array.from(m.entries()).map(([token, weight]) => ({ token, weight }));
}
