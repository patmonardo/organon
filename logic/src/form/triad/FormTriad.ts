import {
  SemanticGraph,
  Monad,
  Dyad,
  Triad,
  TriadCorrespondence,
  triadEdges,
  validateGraph,
  seed,
} from "../../schema/triad";

export class FormTriad {
  private g: SemanticGraph;

  private constructor(g: SemanticGraph) {
    validateGraph(g);
    this.g = g;
  }

  static create(g?: SemanticGraph): FormTriad {
    return new FormTriad(g ?? seed());
  }

  toSchema(): SemanticGraph {
    return this.g;
  }

  monads(): ReadonlyArray<Monad> {
    return this.g.monads;
  }

  dyads(): ReadonlyArray<Dyad> {
    return this.g.dyads;
  }

  triads(): ReadonlyArray<Triad> {
    return this.g.triads;
  }

  correspondences(): ReadonlyArray<TriadCorrespondence> {
    return this.g.correspondences;
  }

  findTriad(id: string): Triad | undefined {
    return this.g.triads.find(t => t.id === id);
  }

  edgesOfTriad(id: string): [string, string][] {
    const t = this.findTriad(id);
    if (!t) throw new Error(`Unknown triad: ${id}`);
    return triadEdges(t);
  }

  // Given a correspondence, return the projected pairs for each vertex label
  projectCorrespondence(id: string): { vertex: string; pair: [Monad, Monad] }[] {
    const c = this.g.correspondences.find(x => x.id === id);
    if (!c) throw new Error(`Unknown correspondence: ${id}`);
    const triadFrom = this.findTriad(c.from)!;
    const byId = new Map(this.g.monads.map(n => [n.id, n]));
    const vLabel = (vKey: "a" | "b" | "c") => byId.get((triadFrom as any)[vKey])!.label;

    const toNode = (nid: string) => {
      const n = byId.get(nid);
      if (!n) throw new Error(`Missing node ${nid}`);
      return n;
    };

    return [
      { vertex: vLabel("a"), pair: [toNode(c.mapping.a[0]), toNode(c.mapping.a[1])] },
      { vertex: vLabel("b"), pair: [toNode(c.mapping.b[0]), toNode(c.mapping.b[1])] },
      { vertex: vLabel("c"), pair: [toNode(c.mapping.c[0]), toNode(c.mapping.c[1])] },
    ];
  }
}
