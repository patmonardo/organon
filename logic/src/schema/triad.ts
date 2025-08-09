import { z } from "zod";

// Minimal conceptual moments (aligns with your Logic: Being/Essence/Concept movement)
export const Moment = z.enum(["immediate", "negative", "unity"]);
export type Moment = z.infer<typeof Moment>;

// Monad: a simple concept/vertex (formerly Node)
export const Monad = z.object({
  id: z.string().min(1),
  label: z.string().min(1),
  moment: Moment.optional(),        // optional: use when you want Hegelian moment
  tags: z.array(z.string()).optional(),
});
export type Monad = z.infer<typeof Monad>;

// Dyad: a compelled pair with a unity monad (sublation) — was Dvandva
export const Dyad = z.object({
  id: z.string().min(1),
  left: z.string().min(1),     // monad id
  right: z.string().min(1),    // monad id
  unity: z.string().min(1),    // monad id (distinct from left/right)
  label: z.string().optional(),// e.g., Becoming, Measure, Balance
});
export type Dyad = z.infer<typeof Dyad>;

// Triad: three nodes mutually determining one another
export const Triad = z.object({
  id: z.string().min(1),
  a: z.string().min(1),        // monad ids
  b: z.string().min(1),
  c: z.string().min(1),
  label: z.string().optional(),
});
export type Triad = z.infer<typeof Triad>;

// A correspondence between two triads where each vertex maps to a pair on the other triad.
// Example (Time→Modality):
// - Present -> {Actuality, Possibility}
// - Past    -> {Actuality, Necessity}
// - Future  -> {Possibility, Necessity}
export const TriadCorrespondence = z.object({
  id: z.string().min(1),
  from: z.string().min(1),   // Triad.id
  to: z.string().min(1),     // Triad.id
  mapping: z.object({
    // keys are 'a' | 'b' | 'c' (vertices of 'from' triad)
    a: z.tuple([z.string().min(1), z.string().min(1)]),
    b: z.tuple([z.string().min(1), z.string().min(1)]),
    c: z.tuple([z.string().min(1), z.string().min(1)]),
  }),
  note: z.string().optional(),
});
export type TriadCorrespondence = z.infer<typeof TriadCorrespondence>;

export const SemanticGraph = z.object({
  monads: z.array(Monad),
  dyads: z.array(Dyad).default([]),
  triads: z.array(Triad).default([]),
  correspondences: z.array(TriadCorrespondence).default([]),
});
export type SemanticGraph = z.infer<typeof SemanticGraph>;

// Helpers

export function validateGraph(g: SemanticGraph): void {
  SemanticGraph.parse(g);
  const ids = new Set(g.monads.map(n => n.id));

  // Dyad sanity
  for (const d of g.dyads) {
    if (!ids.has(d.left) || !ids.has(d.right) || !ids.has(d.unity)) {
      throw new Error(`Dyad ${d.id} references unknown monad`);
    }
    if (d.left === d.right || d.unity === d.left || d.unity === d.right) {
      throw new Error(`Dyad ${d.id} must have three distinct monads`);
    }
  }

  // Triad sanity
  for (const t of g.triads) {
    const verts = [t.a, t.b, t.c];
    for (const v of verts) if (!ids.has(v)) throw new Error(`Triad ${t.id} missing monad ${v}`);
    if (new Set(verts).size !== 3) throw new Error(`Triad ${t.id} needs three distinct monads`);
  }

  // Correspondence sanity (each pair should be nodes of 'to' triad)
  const triadById = new Map(g.triads.map(t => [t.id, t]));
  for (const c of g.correspondences) {
    const from = triadById.get(c.from);
    const to = triadById.get(c.to);
    if (!from || !to) throw new Error(`Correspondence ${c.id} references unknown triad`);

    const toVerts = new Set([to.a, to.b, to.c]);
    const pairs = [c.mapping.a, c.mapping.b, c.mapping.c];
    for (const [x, y] of pairs) {
      if (!toVerts.has(x) || !toVerts.has(y) || x === y) {
        throw new Error(`Correspondence ${c.id} maps to invalid pair (${x}, ${y})`);
      }
    }
  }
}

export function triadEdges(t: Triad): [string, string][] {
  // undirected edges: (a,b), (b,c), (c,a)
  return [[t.a, t.b], [t.b, t.c], [t.c, t.a]];
}

// Seed with your examples
export function seed(): SemanticGraph {
  const monads: Monad[] = [
    // Hegel: Being/Nothing/Becoming
    { id: "pure-being", label: "Pure Being", moment: "immediate", tags: ["hegel"] },
    { id: "pure-nothing", label: "Pure Nothing", moment: "immediate", tags: ["hegel"] },
    { id: "becoming", label: "Becoming", moment: "unity", tags: ["hegel"] },

    // Quality/Quantity/Measure
    { id: "quality", label: "Quality (Determinateness)", moment: "negative", tags: ["hegel"] },
    { id: "quantity", label: "Quantity (Magnitude)", moment: "negative", tags: ["hegel"] },
    { id: "measure", label: "Measure", moment: "unity", tags: ["hegel"] },

    // Yoga: Guna + inner split of Sattva
    { id: "sattva", label: "Sattva", tags: ["guna"] },
    { id: "tamas", label: "Tamas", tags: ["guna"] },
    { id: "rajas", label: "Rajas", tags: ["guna"] },

    { id: "sat", label: "Sat (Being)", moment: "immediate", tags: ["sattva:inner"] },
    { id: "sunya", label: "Śūnya (Nothing)", moment: "immediate", tags: ["sattva:inner"] },
    { id: "sattva-unity", label: "Sattva-as-Unity (Sat/Śūnya)", moment: "unity", tags: ["sattva:inner"] },

    // Time triad
    { id: "present", label: "Present", tags: ["time"] },
    { id: "past", label: "Past", tags: ["time"] },
    { id: "future", label: "Future", tags: ["time"] },

    // Modality triad
    { id: "actuality", label: "Actuality", tags: ["modality"] },
    { id: "possibility", label: "Possibility", tags: ["modality"] },
    { id: "necessity", label: "Necessity", tags: ["modality"] },
  ];

  const dyads: Dyad[] = [
    { id: "d1", left: "pure-being", right: "pure-nothing", unity: "becoming", label: "Becoming" },
    { id: "d2", left: "quality", right: "quantity", unity: "measure", label: "Measure" },
    // Sattva inner dvandva
    { id: "d3", left: "sat", right: "sunya", unity: "sattva-unity", label: "Unity-in-Sattva" },
  ];

  const triads: Triad[] = [
    { id: "t_guna", a: "sattva", b: "rajas", c: "tamas", label: "Guna Triad" },
    { id: "t_time", a: "present", b: "past", c: "future", label: "Time Triad" },
    { id: "t_modal", a: "actuality", b: "possibility", c: "necessity", label: "Modality Triad" },
  ];

  // Your mapping: vertex -> pair on the other triad
  const correspondences: TriadCorrespondence[] = [
    {
      id: "c_time_to_modality",
      from: "t_time",
      to: "t_modal",
      mapping: {
        // Present -> {Actuality, Possibility}
        a: ["actuality", "possibility"],
        // Past -> {Actuality, Necessity}
        b: ["actuality", "necessity"],
        // Future -> {Possibility, Necessity}
        c: ["possibility", "necessity"],
      },
      note: "Edges of Modality triangle read off at the Time vertices",
    },
  ];

  const g: SemanticGraph = { monads, dyads, triads, correspondences };
  validateGraph(g);
  return g;
}