import { describe, it, expect } from "vitest";
import { createSyllogism } from "../../src/schema/syllogism";
import { deriveSyllogisticEdges } from "../../src/absolute";

describe("processor/judgment — deriveSyllogisticEdges", () => {
  it("derives a directed subset_of edge for AAA, figure 1", () => {
    const syl = createSyllogism({
      type: "system.Syllogism",
      figure: "1",
      mood: "AAA",
      terms: {
        S: { id: "S", type: "system.Thing" },
        P: { id: "P", type: "system.Thing" },
        M: { id: "M", type: "system.Thing" },
      },
    });

    const edges = deriveSyllogisticEdges({
      entities: [],
      relations: [],
      properties: [],
      contexts: [],
      morphs: [],
      shapes: [],
      content: [],
      judgments: [],
      syllogisms: [syl],
    } as any);

    expect(edges.length).toBe(1);
    const e = edges[0];
    expect(e.kind).toBe("subset_of");
    expect(e.direction).toBe("directed");
    expect(e.source.id).toBe("S");
    expect(e.target.id).toBe("P");
  });
});
