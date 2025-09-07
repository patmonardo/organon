// Simple, dependency-free topic finder utilities.

type TopicTerm = {
  id: string
  label?: string
  aliases?: string[]
  desc?: string
  tags?: Record<string, any>
}

type Topic = {
  id: string
  title?: string
  terms: TopicTerm[]
  edges?: any[]
}

type Index = Map<string, Set<string>>

function norm(s: string | undefined) {
  return String(s ?? '').toLowerCase().trim()
}

function tokensFromText(s: string) {
  return Array.from(
    new Set(
      String(s ?? '')
        .toLowerCase()
        .split(/\W+/)
        .filter(Boolean),
    ),
  )
}

/** Build a simple inverted index from a Topic (label, aliases, desc, tags) */
export function buildInvertedIndex(topic: Topic): Index {
  const idx: Index = new Map()
  for (const t of topic.terms ?? []) {
    const add = (tok: string) => {
      const set = idx.get(tok) ?? new Set<string>()
      set.add(t.id)
      idx.set(tok, set)
    }
    for (const tok of tokensFromText(t.label ?? t.id)) add(tok)
    for (const a of t.aliases ?? []) for (const tok of tokensFromText(a)) add(tok)
    for (const tok of tokensFromText(t.desc ?? '')) add(tok)
    if (t.tags) {
      for (const [k, v] of Object.entries(t.tags)) {
        for (const tok of tokensFromText(k)) add(tok)
        for (const tok of tokensFromText(String(v))) add(tok)
      }
    }
  }
  return idx
}

type SearchOpts = {
  fuzzy?: boolean
  limit?: number
  facetWeights?: Record<string, number>
}

function scoreTerm(term: TopicTerm, queryToks: string[], facetWeights?: Record<string, number>) {
  let score = 0
  const hay = `${term.label ?? ''} ${(term.aliases ?? []).join(' ')} ${term.desc ?? ''}`.toLowerCase()
  for (const q of queryToks) {
    if (hay.includes(q)) score += 10
    if ((term.aliases ?? []).some(a => a.toLowerCase().includes(q))) score += 6
    if (facetWeights && facetWeights[q]) score += Math.round(facetWeights[q] * 5)
  }
  return score
}

/** Find term ids/objects matching query. Returns sorted {id,score,term} */
export function findTerms(topic: Topic, query: string, opts: SearchOpts = {}) {
  const q = norm(query)
  if (!q) return []
  const qToks = tokensFromText(q)
  const idx = buildInvertedIndex(topic)
  const candidates = new Set<string>()
  for (const tok of qToks) {
    const hit = idx.get(tok)
    if (hit) for (const id of hit) candidates.add(id)
  }
  if (opts.fuzzy && candidates.size === 0) {
    for (const t of topic.terms ?? []) {
      if ((t.label ?? '').toLowerCase().includes(q) || (t.aliases ?? []).some(a => a.toLowerCase().includes(q))) {
        candidates.add(t.id)
      }
    }
  }
  const facetWeights = opts.facetWeights ?? undefined
  const results = Array.from(candidates).map(id => {
    const term = (topic.terms ?? []).find(t => t.id === id)!
    return { id, score: scoreTerm(term, qToks, facetWeights), term }
  })
  results.sort((a, b) => b.score - a.score)
  return results.slice(0, opts.limit ?? 50)
}

/** Produce a simple glossary map: id -> { label, aliases, desc } */
export function glossary(topic: Topic) {
  const out: Record<string, { label: string; aliases?: string[]; desc?: string }> = {}
  for (const t of topic.terms ?? []) out[t.id] = { label: t.label ?? t.id, aliases: t.aliases, desc: t.desc }
  return out
}
