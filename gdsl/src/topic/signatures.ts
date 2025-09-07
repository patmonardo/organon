export function facetWeightsFromFacets(facets: Record<string, Array<{ token: string; weight?: number }>>): Record<string, number> {
  const out: Record<string, number> = {}
  for (const arr of Object.values(facets ?? {})) {
    for (const { token, weight = 1 } of arr) {
      out[String(token)] = (out[String(token)] ?? 0) + weight
    }
  }
  return out
}
