import { describe, it, expect } from "vitest";
import { runCycle, type StageFns, type Principles } from "../../src/absolute/core";
import type { Entity } from "../../src/schema/entity";
import type { Property } from "../../src/schema/property";
import type { Relation } from "../../src/schema/relation";

describe("runCycle (Ring 1→2→3 flow)", () => {
  it("seeds, contextualizes, grounds, then models/controls/plans", async () => {
    const fns: StageFns = {
      seed: async () => ({
        entities: [{ id: "e1" } as unknown as Entity],
      }),
      contextualize: async () => ({
        properties: [{ id: "p1" } as unknown as Property],
      }),
      ground: async () => ({
        relations: [{ id: "r1" } as unknown as Relation],
      }),
      model: async (g) => ({
        indexes: { entityCount: g.entities.length, relationCount: g.relations.length },
      }),
      control: async (_g, proj) => ({
        actions: [{ kind: "noop", args: proj }],
      }),
      plan: async (ctrl) => ({
        tasks: [{ id: "t1", kind: "noop", args: ctrl.actions[0].args }],
        workflow: { nodes: ["t1"], edges: [] },
      }),
    };

    const principles: Principles = {
      shapes: [] as any,
      contexts: [] as any,
      morphs: [] as any,
    };

    const res = await runCycle(principles, fns, { fixpointMaxIters: 8 });
    expect(res.graph.entities.length).toBe(1);
    expect(res.graph.properties.length).toBe(1);
    expect(res.graph.relations.length).toBe(1);
    expect(res.controls.actions[0].kind).toBe("noop");
    expect(res.work.tasks[0].id).toBe("t1");
  });
});
