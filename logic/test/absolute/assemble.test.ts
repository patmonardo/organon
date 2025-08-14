import { describe, it, expect } from "vitest";
import { assembleWorld } from "../../src/absolute/world/assemble";

function entity(id: string) {
  return {
    shape: { core: { id, type: "system.Entity" } },
  } as any;
}

function relation(kind: string, s: string, t: string, direction: "directed" | "bidirectional" = "directed") {
  return {
    shape: {
      core: { type: "system.Relation", kind },
      source: { id: s, type: "system.Thing" },
      target: { id: t, type: "system.Thing" },
      direction,
      state: { status: "active", tags: [], meta: {} },
    },
  } as any;
}

describe("absolute/world — assembleWorld", () => {
  it("assembles things from entities and edges from relations deterministically", () => {
    const world = assembleWorld({
      entities: [entity("A"), entity("B")],
      relations: [
        relation("related_to", "A", "B"),
        relation("related_to", "A", "B"), // duplicate
      ],
      properties: [],
      contexts: [],
      morphs: [],
      shapes: [],
      content: [],
      judgments: [],
      syllogisms: [],
    } as any);

    expect(world.shape.things.map((t) => t.id)).toEqual(["A", "B"]);
    expect(world.shape.relations).toHaveLength(1);
    expect(world.shape.relations[0]).toMatchObject({
      kind: "related_to",
      direction: "directed",
      source: { id: "A" },
      target: { id: "B" },
    });
  });

  it("normalizes bidirectional dedupe irrespective of source/target order", () => {
    const world = assembleWorld({
      entities: [],
      relations: [
        relation("adjacent_to", "X", "Y", "bidirectional"),
        relation("adjacent_to", "Y", "X", "bidirectional"),
      ],
      properties: [],
      contexts: [],
      morphs: [],
      shapes: [],
      content: [],
      judgments: [],
      syllogisms: [],
    } as any);

    expect(world.shape.things.map((t) => t.id).sort()).toEqual(["X", "Y"]);
    expect(world.shape.relations).toHaveLength(1);
    expect(world.shape.relations[0].direction).toBe("bidirectional");
  });
});
