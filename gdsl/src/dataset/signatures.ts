import type { GraphArtifact } from '../schema/projection'
import { validateFacets, Facets, SignatureToken } from '../schema/signature'

/** Compute per-term facets/signatures from a GraphArtifact (conservative baseline). */
export function computeTermSignatures(artifact: GraphArtifact): Facets {
  // prefer explicit artifact.signatures if present
  if (artifact.signatures && Object.keys(artifact.signatures).length > 0) {
    return validateFacets(artifact.signatures)
  }

  const termMap = new Map<string, Map<string, number>>()

  const ensure = (tid: string) => {
    if (!termMap.has(tid)) termMap.set(tid, new Map())
    return termMap.get(tid)!
  }

  // extract from artifact.terms (common fields)
  for (const t of artifact.terms ?? []) {
    const tid = String((t as any).id ?? (t as any).term ?? '')
    if (!tid) continue
    const m = ensure(tid)
    const collect = (arr?: string[] | undefined) => {
      for (const tok of arr ?? []) if (tok) m.set(String(tok), (m.get(String(tok)) ?? 0) + 1)
    }
    collect((t as any).tokens ?? [])
    collect((t as any).signatureTokens ?? [])
    collect((t as any).aliases ?? [])
    const tags = (t as any).tags
    if (tags && typeof tags === 'object') {
      for (const [k, v] of Object.entries(tags)) {
        if (k) m.set(String(k), (m.get(String(k)) ?? 0) + 1)
        if (v) m.set(String(v), (m.get(String(v)) ?? 0) + 1)
      }
    }
  }

  // edges of convention HLO_TAGS_TERM or similar -> add tokens
  for (const e of artifact.edges ?? []) {
    try {
      const ttype = String(e.type ?? '')
      if (ttype === 'HLO_TAGS_TERM' || ttype === 'TERM_HAS_TOKEN' || ttype === 'TERM_TAG') {
        const termId = String(e.from)
        const token = e.props?.value !== undefined ? String(e.props.value) : String(e.to)
        if (!token) continue
        const m = ensure(termId)
        m.set(token, (m.get(token) ?? 0) + 1)
      }
    } catch {
      // ignore malformed edges
    }
  }

  // flatten to Facets shape
  const out: Facets = {}
  for (const [tid, map] of termMap.entries()) {
    out[tid] = Array.from(map.entries()).map(([token, weight]) => ({ token, weight }))
  }
  return validateFacets(out)
}

/** Convert facets -> simple token => weight map (summing across facets) */
export function facetWeightsFromFacets(facets: Facets): Record<string, number> {
  const out: Record<string, number> = {}
  for (const arr of Object.values(facets ?? {})) {
    for (const { token, weight = 1 } of arr as SignatureToken[]) {
      out[String(token)] = (out[String(token)] ?? 0) + (Number.isFinite(Number(weight)) ? Number(weight) : 1)
    }
  }
  return out
}

/** Merge two facets maps (sum token weights, preserve facet keys) */
export function mergeFacets(a: Facets | undefined, b: Facets | undefined): Facets {
  const result: Record<string, Record<string, number>> = {}

  const add = (name: string | undefined, arr?: SignatureToken[]) => {
    if (!name) return
    if (!result[name]) result[name] = {}
    for (const it of arr ?? []) {
      const token = String(it.token)
      const w = Number.isFinite(Number(it.weight)) ? Number(it.weight) : 1
      result[name][token] = (result[name][token] ?? 0) + w
    }
  }

  for (const [k, v] of Object.entries(a ?? {})) add(k, v)
  for (const [k, v] of Object.entries(b ?? {})) add(k, v)

  const out: Facets = {}
  for (const [k, m] of Object.entries(result)) {
    out[k] = Object.entries(m).map(([token, weight]) => ({ token, weight }))
  }
  return validateFacets(out)
}
