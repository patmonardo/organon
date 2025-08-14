import { describe, it, expect } from "vitest";
import { ContextSchema } from "../../src/schema";
import { projectContentFromContexts } from "../../src/absolute";

function ctxDoc(id: string, name: string, entityIds: string[]) {
  return ContextSchema.parse({
    shape: {
      core: { id, type: "system.Context", name },
      state: { status: "active", tags: [], meta: {} },
      entities: entityIds.map((eid) => ({ id: eid, type: "system.Thing" })),
      relations: [],
    },
  });
}

describe("processor/form — projectContentFromContexts", () => {
  it("projects deterministic Content ids per (context, entity) and classifies subtle/gross", () => {
    const ctxWorld = ctxDoc("ctx:world", "WorldScope", ["E1", "E2"]);
    const ctxLocal = ctxDoc("ctx:local", "LocalScope", ["E2", "E3"]);

    const out = projectContentFromContexts({
      entities: [],
      relations: [],
      properties: [],
      contexts: [ctxLocal, ctxWorld], // out-of-order to verify stable sort
      morphs: [],
      shapes: [],
      content: [],
      judgments: [],
      syllogisms: [],
    } as any);

    const ids = out.map((c) => c.shape.core.id);
    expect(ids).toEqual([
      "content:subtle:ctx:world:E1",
      "content:subtle:ctx:world:E2",
      "content:gross:ctx:local:E2",
      "content:gross:ctx:local:E3",
    ]);

    const kinds = out.map((c) => c.shape.kind);
    expect(kinds).toEqual(["subtle", "subtle", "gross", "gross"]);

    const carriers = out.map((c) => c.shape.of.id);
    expect(carriers).toEqual(["E1", "E2", "E2", "E3"]);
  });

  it("dedupes repeated entity refs per-context when dedupeEntities=true", () => {
    const ctx = ctxDoc("ctx:x", "LocalScope", ["A", "A", "B"]);
    const out = projectContentFromContexts(
      {
        entities: [],
        relations: [],
        properties: [],
        contexts: [ctx],
        morphs: [],
        shapes: [],
        content: [],
        judgments: [],
        syllogisms: [],
      } as any,
      { dedupeEntities: true }
    );
    expect(out.map((c) => c.shape.core.id)).toEqual([
      "content:gross:ctx:x:A",
      "content:gross:ctx:x:B",
    ]);
  });
});
