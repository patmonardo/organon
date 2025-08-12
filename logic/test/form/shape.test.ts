import { describe, it, expect } from "vitest";
import { FormShape } from "../../src/form/shape";
import { type ShapeData, type ShapeState } from "../../src/schema/shape";

describe("FormShape (wrapper)", () => {
  it("creates with defaults", () => {
    const s = FormShape.create({
      definitionId: "def:shape:1",
      definitionName: "Test Shape",
    });
    expect(s.id).toBeTruthy();
    expect(s.definitionId).toBe("def:shape:1");
    expect(s.definitionName).toBe("Test Shape");
    expect(s.state.status).toBe("idle"); // schema: idle|submitting|success|error
    expect(s.data).toBeUndefined();
  });

  it("sets and merges data (shallow)", () => {
    const s = FormShape.create({
      definitionId: "def:shape:2",
      definitionName: "Data Merge",
    });

    const initial: ShapeData = {
      source: {
        type: "entity",
        entityRef: { entity: "system.Entity", id: "e1" },
      },
      access: { read: { path: "name", cache: true } },
      meta: { version: "1" },
    };

    s.setData(initial);
    s.mergeData({
      // shallow merge replaces meta entirely
      meta: { version: "2" },
    } as Partial<ShapeData>);

    expect(s.data?.meta?.version).toBe("2");
    // shallow merge: access.read remains since we didn't override 'access'
    expect(s.data?.access?.read?.path).toBe("name");
  });

  it("sets and patches state", () => {
    const s = FormShape.create({
      definitionId: "def:shape:3",
      definitionName: "State Patch",
      state: { status: "submitting" } as ShapeState,
    });
    expect(s.state.status).toBe("submitting");
    s.patchState({ status: "success", message: "ok" });
    expect(s.state.status).toBe("success");
    expect((s.state as any).message).toBe("ok");
  });

  it("serializes with toJSON", () => {
    const s = FormShape.create({
      id: "form:abc",
      definitionId: "def:shape:4",
      definitionName: "Serialize",
      data: {
        source: { type: "function", functionRef: { name: "gen" } },
      } as ShapeData,
      state: { status: "submitting" } as ShapeState,
    });
    const json = s.toJSON();
    expect(json.id).toBe("form:abc");
    expect(json.definitionName).toBe("Serialize");
    expect(json.data?.source?.type).toBe("function");
    expect(json.state.status).toBe("submitting");
  });
});
