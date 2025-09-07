import {
  validateLens,
  LensTermSchema,
  LensEdgeSchema,
  type Lens,
  type LensTerm,
  type LensEdge,
} from '../schema/lens';

/**
 * BaseLens - abstract base class for concrete lenses.
 * - keeps validated Lens data
 * - provides ergonomic mutators (addTerm, addEdge, merge)
 * - subclasses provide domain-specific constructors / helpers
 */
export abstract class BaseLens {
  protected data: Lens;

  protected constructor(data: Lens) {
    this.data = validateLens(data);
  }

  get id(): string {
    return this.data.id;
  }

  get title(): string | undefined {
    return this.data.title;
  }

  get lens(): Lens {
    return this.data;
  }

  addTerm(term: Partial<LensTerm>) {
    const parsed = LensTermSchema.parse({
      id:
        term.id ??
        (() => {
          throw new Error('term.id required');
        })(),
      label: term.label ?? term.id,
      aliases: term.aliases,
      desc: term.desc,
      tags: term.tags,
    });
    if (!this.data.terms.find((t) => t.id === parsed.id))
      this.data.terms.push(parsed);
    return this;
  }

  addEdge(edge: Partial<LensEdge>) {
    const parsed = LensEdgeSchema.parse({
      type:
        edge.type ??
        (() => {
          throw new Error('edge.type required');
        })(),
      from:
        edge.from ??
        (() => {
          throw new Error('edge.from required');
        })(),
      to:
        edge.to ??
        (() => {
          throw new Error('edge.to required');
        })(),
      props: edge.props,
    });
    this.data.edges.push(parsed);
    return this;
  }

  merge(other: Lens | BaseLens) {
    const o = other instanceof BaseLens ? other.data : other;
    const termMap = new Map(this.data.terms.map((t) => [t.id, t]));
    for (const t of o.terms)
      if (!termMap.has(t.id)) {
        this.data.terms.push(t);
        termMap.set(t.id, t);
      }

    const edgeKey = (e: LensEdge) => `${e.type}|${e.from}|${e.to}`;
    const seen = new Set(this.data.edges.map(edgeKey));
    for (const e of o.edges)
      if (!seen.has(edgeKey(e))) {
        this.data.edges.push(e);
        seen.add(edgeKey(e));
      }

    this.data.title = this.data.title ?? o.title;
    this.data.refs = Array.from(
      new Set([...(this.data.refs ?? []), ...(o.refs ?? [])]),
    );
    this.data.provenance = {
      ...(o.provenance ?? {}),
      ...(this.data.provenance ?? {}),
    };
    return this;
  }

  validate() {
    this.data = validateLens(this.data);
    return this;
  }

  toJSON(): Lens {
    this.validate();
    return this.data;
  }
}

export default BaseLens;
