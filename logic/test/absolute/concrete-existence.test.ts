import { describe, it, expect } from "vitest";
import { runCycle, type StageFns, type Principles } from "../../src/absolute/kriya";
import type { Entity } from "../../src/schema/entity";
import type { Property } from "../../src/schema/property";
import type { Relation } from "../../src/schema/relation";

describe("Concrete Existence — E/P/R under Context", () => {
  it("Contextualizes properties and grounds essential relations to a fixpoint (stubbed)", async () => {
    const fns: StageFns = {
      // Shape → Entity
      seed: async (_p) => ({
        entities: [{ id: "e1" } as unknown as Entity, { id: "e2" } as unknown as Entity],
      }),
      // Context → Property (attach provenance)
      contextualize: async (_p, _g) => ({
        properties: [
          { id: "p:e1:A", entityId: "e1", key: "A", contextId: "ctx", contextVersion: 1 } as any as Property,
          { id: "p:e2:A", entityId: "e2", key: "A", contextId: "ctx", contextVersion: 1 } as any as Property,
        ],
      }),
      // Morph → Relation (seed one, derive one)
      ground: async (_p, g) => {
        const seeded = [{ id: "r:e1->e2", sourceId: "e1", targetId: "e2", type: "R" } as any as Relation];
        // pretend a rule derives the back edge if missing
        const derived = g.properties.length > 0
          ? [{ id: "r:e2->e1", sourceId: "e2", targetId: "e1", type: "R" } as any as Relation]
          : [];
        return { relations: [...seeded, ...derived] };
      },
      model: async (graph) => ({ indexes: { E: graph.entities.length, R: graph.relations.length } }),
      control: async (_g, proj) => ({ actions: [{ kind: "audit", args: proj.indexes }] }),
      plan: async (ctrl) => ({
        tasks: [{ id: "t1", kind: ctrl.actions[0].kind, args: ctrl.actions[0].args }],
        workflow: { nodes: ["t1"], edges: [] },
      }),
    };

    const p: Principles = { shapes: [] as any, contexts: [] as any, morphs: [] as any };
    const res = await runCycle(p, fns, { fixpointMaxIters: 8 });

    expect(res.graph.entities.length).toBe(2);
    expect(res.graph.properties.length).toBe(2);
    expect(res.graph.relations.length).toBe(2);
    expect(res.controls.actions[0].kind).toBe("audit");
    expect(res.work.tasks[0].id).toBe("t1");
  });
});
