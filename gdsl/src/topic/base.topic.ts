import {
  TopicSchema,
  type Topic,
  type TopicTerm,
  type TopicEdge,
} from '../schema/topic';
import type { GraphArtifact, NodeRow, EdgeRow } from '../schema/projection';

export class BaseTopic {
  data: Topic;

  private constructor(data: Topic) {
    this.data = TopicSchema.parse(data);
  }

  static create(
    id: string,
    opts?: Partial<Omit<Topic, 'id'>>,
  ) {
    const base: Topic = {
      id,
      title: opts?.title,
      description: opts?.description,
      terms: opts?.terms ?? [],
      edges: opts?.edges ?? [],
      signatures: opts?.signatures ?? {},
      provenance: opts?.provenance,
    } as Topic;
    return new BaseTopic(TopicSchema.parse(base));
  }

  static fromArtifact(id: string, artifact: GraphArtifact) {
    const termsFromArtifact = (artifact.terms ?? []).map((t: any) => ({
      id: String(
        t.id ??
          t.term ??
          t.name ??
          `term:${Math.random().toString(36).slice(2)}`,
      ),
      label: String(t.label ?? t.name ?? t.id ?? ''),
      aliases: (t.aliases ?? []) as string[] | undefined,
      desc: t.desc,
      tags: t.tags,
      refs: t.refs,
    })) as TopicTerm[];

    const nodeTerms = (artifact.nodes ?? [])
      .filter(
        (n: NodeRow) =>
          ((n.labels ?? []) as string[]).includes('Term') ||
          (n.props as any)?.isTerm,
      )
      .map((n: NodeRow) => ({
        id: String(n.id),
        label: String((n.props as any)?.label ?? n.id),
        aliases: (n.props as any)?.aliases,
        desc: (n.props as any)?.desc,
        tags: (n.props as any)?.tags,
        refs: (n.props as any)?.refs,
      })) as TopicTerm[];

    const byId = new Map<string, TopicTerm>();
    for (const t of [...termsFromArtifact, ...nodeTerms])
      byId.set(t.id, { ...byId.get(t.id), ...t });
    const terms = Array.from(byId.values());

    const termIds = new Set(terms.map((t) => t.id));
    const edges = (artifact.edges ?? [])
      .filter(
        (e: EdgeRow) =>
          termIds.has(String(e.from)) && termIds.has(String(e.to)),
      )
      .map((e: EdgeRow) => ({
        type: e.type,
        from: String(e.from),
        to: String(e.to),
        props: e.props,
      })) as TopicEdge[];

    return new BaseTopic(
      TopicSchema.parse({
        id,
        title: `Topic — ${artifact.dataset}`,
        description: undefined,
        terms,
        edges,
        signatures: artifact.counts?.signatures
          ? (artifact as any).signatures
          : {},
        provenance: { derivedFrom: artifact.dataset },
      }),
    );
  }

  addTerm(term: Partial<TopicTerm>) {
    if (!term.id) throw new Error('term.id required');
    if (!this.data.terms.find((t) => t.id === term.id))
      this.data.terms.push({
        id: term.id,
        label: term.label ?? term.id,
        aliases: term.aliases,
        desc: term.desc,
        tags: term.tags,
        refs: (term as any).refs,
      });
    return this;
  }

  addEdge(edge: Partial<TopicEdge>) {
    if (!edge.type || !edge.from || !edge.to)
      throw new Error('edge.type/from/to required');
    this.data.edges.push({
      type: edge.type,
      from: edge.from!,
      to: edge.to!,
      props: edge.props,
    });
    return this;
  }

  merge(other: Topic | BaseTopic) {
    const o = other instanceof BaseTopic ? other.data : other;
    const termMap = new Map(this.data.terms.map((t) => [t.id, t]));
    for (const t of o.terms)
      if (!termMap.has(t.id)) {
        this.data.terms.push(t);
        termMap.set(t.id, t);
      }

    const edgeKey = (e: TopicEdge) => `${e.type}|${e.from}|${e.to}`;
    const seen = new Set(this.data.edges.map(edgeKey));
    for (const e of o.edges)
      if (!seen.has(edgeKey(e))) {
        this.data.edges.push(e);
        seen.add(edgeKey(e));
      }

    this.data.title = this.data.title ?? o.title;
    this.data.description = this.data.description ?? o.description;
    this.data.signatures = {
      ...(o.signatures ?? {}),
      ...(this.data.signatures ?? {}),
    };
    this.data.provenance = {
      ...(o.provenance ?? {}),
      ...(this.data.provenance ?? {}),
    };
    return this;
  }

  validate() {
    this.data = TopicSchema.parse(this.data);
    return this;
  }

  toJSON(): Topic {
    this.validate();
    return this.data;
  }
}

export default BaseTopic;
