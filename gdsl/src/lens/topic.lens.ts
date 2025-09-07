import BaseLens from './base.lens'
import { makeLensId, type Lens } from '../schema/lens'
import type { GraphArtifact, NodeRow, EdgeRow } from '../schema/projection'

/** TopicLens: canonical lens for Topic/Term space (terms + signature links) */
export class TopicLens extends BaseLens {
  private constructor(data: Lens) { super(data) }

  static create(id = makeLensId('topic')) {
    return new TopicLens({ id, title: 'Topic Lens (terms & signatures)', terms: [], edges: [] })
  }

  /** Construct topic lens from artifact. Prefer artifact.terms, fall back to Term-labeled nodes. */
  static fromArtifact(id: string, artifact: GraphArtifact) {
    const termsFromSchema = (artifact.terms ?? []).map((t: any) => ({
      id: String(t.id ?? t.term ?? t.name ?? `term:${Math.random().toString(36).slice(2)}`),
      label: String(t.label ?? t.name ?? t.id ?? ''),
      aliases: t.aliases,
      desc: t.desc,
      tags: t.tags,
    }))

    const nodeTerms = (artifact.nodes ?? [])
      .filter((n: NodeRow) => ((n.labels ?? []) as string[]).includes('Term') || (n.props as any)?.isTerm)
      .map((n: NodeRow) => ({
        id: String(n.id),
        label: String((n.props as any)?.label ?? n.id),
        aliases: (n.props as any)?.aliases,
        desc: (n.props as any)?.desc,
        tags: (n.props as any)?.tags,
      }))

    // merge term lists (dedupe by id)
    const byId = new Map<string, any>()
    for (const t of [...termsFromSchema, ...nodeTerms]) byId.set(t.id, t)
    const terms = Array.from(byId.values())

    const termIds = new Set(terms.map(t => t.id))
    const edges = (artifact.edges ?? [])
      .filter((e: EdgeRow) => termIds.has(String(e.from)) && termIds.has(String(e.to)))
      .map((e: EdgeRow) => ({ type: e.type, from: String(e.from), to: String(e.to), props: e.props }))

    return new TopicLens({ id, title: `Topic Lens — ${artifact.dataset}`, terms, edges })
  }
}
