export type NodeRow = { id: string; labels?: string[]; props?: Record<string, any> }
export type EdgeRow = { type: string; from: string; to: string; props?: Record<string, any> }

export type ArtifactLike = {
  dataset: string
  nodes: NodeRow[]
  edges: EdgeRow[]
  clauses?: Array<{ id: string; hloId: string; raw: string; kind: string }>
  tokens?: string[]
  terms?: Array<Record<string, any>>
  counts?: Record<string, number>
  signatures?: Record<string, Array<{ token: string; weight?: number }>>
}

export type CanonRule = (artifact: ArtifactLike) => ArtifactLike

function clone<T>(x: T): T { return JSON.parse(JSON.stringify(x)) }

/** Dedupe edges by (type|from|to) key, keeping first occurrence. */
export const ruleDedupeEdges: CanonRule = (artifact) => {
  const a = clone(artifact)
  const key = (e: EdgeRow) => `${String(e.type)}|${String(e.from)}|${String(e.to)}`
  const seen = new Set<string>()
  const out: EdgeRow[] = []
  for (const e of a.edges ?? []) {
    const k = key(e)
    if (seen.has(k)) continue
    seen.add(k)
    out.push(e)
  }
  a.edges = out
  return a
}

/** Recompute counts from current arrays. */
export const ruleRecomputeCounts: CanonRule = (artifact) => {
  const a = clone(artifact)
  const clauses = a.clauses ?? []
  a.counts = {
    hlos: (a.nodes ?? []).filter(n => (n.labels ?? []).includes('HLO')).length,
    clauses: clauses.length,
    asserts: clauses.filter(c => c.kind === 'assert').length,
    tags: clauses.filter(c => c.kind === 'tag').length,
    unknown: clauses.filter(c => c.kind === 'unknown').length,
    nodes: (a.nodes ?? []).length,
    edges: (a.edges ?? []).length,
  }
  return a
}

/** If tokens missing/empty, infer from term.tokens/aliases/tags. */
export const ruleInferTokensFromTerms: CanonRule = (artifact) => {
  if (artifact.tokens && artifact.tokens.length > 0) return artifact
  const a = clone(artifact)
  const tok = new Set<string>()
  for (const t of a.terms ?? []) {
    const push = (arr?: any[]) => { for (const s of arr ?? []) if (s) tok.add(String(s)) }
    push((t as any).tokens)
    push((t as any).signatureTokens)
    push((t as any).aliases)
    const tags = (t as any).tags
    if (tags && typeof tags === 'object') {
      for (const [k, v] of Object.entries(tags)) {
        if (k) tok.add(String(k))
        if (v) tok.add(String(v))
      }
    }
  }
  a.tokens = Array.from(tok)
  return a
}

/** Apply rules in order. Rules must be pure; returns a new artifact each step. */
export function applyCanonRules(artifact: ArtifactLike, rules: CanonRule[]): ArtifactLike {
  return rules.reduce((acc, r) => r(acc), artifact)
}

/** Default rule set: safe, idempotent. */
export const DEFAULT_CANON_RULES: CanonRule[] = [
  ruleDedupeEdges,
  ruleInferTokensFromTerms,
  ruleRecomputeCounts,
]

export const ruleDeriveRelatedFromLinkToLabel: CanonRule = (artifact) => {
  // Hypothetical syllogism (Objective Logic): if LINK(a,b) and label(b, 'B'), then RELATED(a,b).
  const a = clone(artifact)
  const nodesById = new Map<string, NodeRow>()
  for (const n of a.nodes ?? []) nodesById.set(String(n.id), n as NodeRow)

  const hasRelated = new Set<string>()
  for (const e of a.edges ?? []) {
    if (String(e.type) === 'RELATED') {
      hasRelated.add(`${String(e.from)}|${String(e.to)}`)
    }
  }

  const out: EdgeRow[] = [...(a.edges ?? [])]
  for (const e of a.edges ?? []) {
    if (String(e.type) !== 'LINK') continue
    const from = String(e.from)
    const to = String(e.to)
    const tgt = nodesById.get(to)
    if (!tgt) continue
    const labels = (tgt.labels ?? []).map(String)
    if (!labels.includes('B')) continue

    const key = `${from}|${to}`
    if (hasRelated.has(key)) continue
    hasRelated.add(key)

    out.push({
      type: 'RELATED',
      from,
      to,
      props: {
        modality: 'inferred',
        provenance: {
          kind: 'hypothetical',
          rule: 'related-from-link-to-label',
          sources: [{ type: 'LINK', from, to }],
        },
      },
    })
  }

  a.edges = out
  return a
}

/**
 * Disjunctive resolution (Objective Logic):
 * From ONE_OF(subject -> option) edges and NEGATED(subject -> option) evidence,
 * if exactly one option remains unnegated, derive ACCEPT(subject -> option).
 * Adds provenance to enable Knowledge → Truth tracing and is idempotent.
 */
export const ruleResolveDisjunction: CanonRule = (artifact) => {
  const a = clone(artifact)
  const edges = (a.edges ?? []) as EdgeRow[]

  const bySubjOptions = new Map<string, Set<string>>()
  const bySubjOneOfEdges = new Map<string, EdgeRow[]>()
  const negated = new Map<string, Set<string>>()
  const acceptKeys = new Set<string>()

  const key = (from: string, to: string) => `${from}|${to}`

  for (const e of edges) {
    const t = String(e.type ?? '')
    const from = String(e.from)
    const to = String(e.to)
    if (t === 'ONE_OF') {
      if (!bySubjOptions.has(from)) bySubjOptions.set(from, new Set<string>())
      bySubjOptions.get(from)!.add(to)
      const arr = bySubjOneOfEdges.get(from) ?? []
      arr.push(e)
      bySubjOneOfEdges.set(from, arr)
    } else if (t === 'NEGATED') {
      if (!negated.has(from)) negated.set(from, new Set<string>())
      negated.get(from)!.add(to)
    } else if (t === 'ACCEPT') {
      acceptKeys.add(key(from, to))
    }
  }

  const out: EdgeRow[] = [...edges]
  for (const [subj, opts] of bySubjOptions.entries()) {
    const neg = negated.get(subj) ?? new Set<string>()
    const remaining = Array.from(opts).filter(o => !neg.has(o))
    if (remaining.length === 1) {
      const to = remaining[0]
      const k = key(subj, to)
      if (acceptKeys.has(k)) continue
      acceptKeys.add(k)

      // provenance: all ONE_OF edges for this subj + all NEGATED edges used
      const srcs: Array<Pick<EdgeRow, 'type' | 'from' | 'to'>> = []
      for (const e of bySubjOneOfEdges.get(subj) ?? []) {
        srcs.push({ type: 'ONE_OF', from: String(e.from), to: String(e.to) })
      }
      for (const n of neg) {
        srcs.push({ type: 'NEGATED', from: subj, to: n })
      }

      out.push({
        type: 'ACCEPT',
        from: subj,
        to,
        props: {
          modality: 'inferred',
          provenance: {
            kind: 'disjunctive',
            rule: 'resolve-disjunction',
            sources: srcs,
          },
        },
      })
    }
  }

  a.edges = out
  return a
}

/**
 * Hypothetical from Conjunction:
 * If HAS(subject, feature) and IMPLIES(feature, Class), then ONE_OF(subject, Class).
 * Adds provenance and is idempotent.
 */
export const ruleHypothesizeFromConjunction: CanonRule = (artifact) => {
  const a = clone(artifact)
  const edges = (a.edges ?? []) as EdgeRow[]

  const hasBySubject = new Map<string, Set<string>>()           // subj -> features
  const impliesByFeature = new Map<string, Set<string>>()       // feature -> classes
  const oneOfKeys = new Set<string>()                           // subj|class existing

  const key = (from: string, to: string) => `${from}|${to}`

  for (const e of edges) {
    const t = String(e.type ?? '')
    const from = String(e.from)
    const to = String(e.to)
    if (t === 'HAS') {
      if (!hasBySubject.has(from)) hasBySubject.set(from, new Set())
      hasBySubject.get(from)!.add(to)
    } else if (t === 'IMPLIES') {
      if (!impliesByFeature.has(from)) impliesByFeature.set(from, new Set())
      impliesByFeature.get(from)!.add(to)
    } else if (t === 'ONE_OF') {
      oneOfKeys.add(key(from, to))
    }
  }

  const out: EdgeRow[] = [...edges]
  for (const [subj, feats] of hasBySubject.entries()) {
    for (const f of feats) {
      const classes = impliesByFeature.get(f)
      if (!classes) continue
      for (const cls of classes) {
        const k = key(subj, cls)
        if (oneOfKeys.has(k)) continue
        oneOfKeys.add(k)
        out.push({
          type: 'ONE_OF',
          from: subj,
          to: cls,
          props: {
            modality: 'inferred',
            provenance: {
              kind: 'hypothetical',
              rule: 'hypothesize-from-conjunction',
              sources: [
                { type: 'HAS', from: subj, to: f },
                { type: 'IMPLIES', from: f, to: cls },
              ],
            },
          },
        })
      }
    }
  }

  a.edges = out
  return a
}
