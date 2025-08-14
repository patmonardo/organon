import { describe, it, expect } from "vitest";
import {
  PropertySchema,
  createProperty,
  updateProperty,
  bindPropertyToEntity,
  bindPropertyToRelation,
} from "../../src/schema/property";

describe("schema/property — happy path", () => {
  it("creates a Property bound to an entity with defaults", () => {
    const p = createProperty({
      type: "system.Property",
      key: "color",
      contextId: "ctx:1",
      entity: { id: "E1", type: "system.Thing" },
      // state omitted -> defaulted
    });

    const parsed = PropertySchema.parse(p);
    expect(parsed.shape.core.type).toBe("system.Property");
    expect(parsed.shape.core.key).toBe("color");
    expect(parsed.shape.contextId).toBe("ctx:1");
    expect(parsed.shape.entity?.id).toBe("E1");
    expect(parsed.shape.relationId).toBeUndefined();
    expect(parsed.revision).toBe(0);
    expect(parsed.ext).toEqual({});
  });

  it("updates value/valueType, core/state, contextId and bumps revision", () => {
    const p0 = createProperty({
      type: "system.Property",
      key: "size",
      contextId: "ctx:1",
      entity: { id: "E2", type: "system.Thing" },
    });

    const p1 = updateProperty(p0, {
      value: 42,
      valueType: "number",
      core: { name: "Size" },
      state: { status: "active", tags: [], meta: {} },
      contextId: "ctx:2",
      version: "1.0.0",
      ext: { a: 1 },
    });

    expect(p1.shape.value).toBe(42);
    expect(p1.shape.valueType).toBe("number");
    expect(p1.shape.core.name).toBe("Size");
    expect(p1.shape.state.status).toBe("active");
    expect(p1.shape.contextId).toBe("ctx:2");
    expect(p1.revision).toBe(p0.revision + 1);
    expect(p1.version).toBe("1.0.0");
    expect(p1.ext).toMatchObject({ a: 1 });

    // Clear value and valueType via explicit undefined
    const p2 = updateProperty(p1, { value: undefined, valueType: undefined });
    expect(p2.shape.value).toBeUndefined();
    expect(p2.shape.valueType).toBeUndefined();
  });

  it("switches binding between entity and relation via helpers (exclusive)", () => {
    const p0 = createProperty({
      type: "system.Property",
      key: "weight",
      contextId: "ctx:9",
      entity: { id: "E3", type: "system.Thing" },
    });

    // Bind to relation (clears entity)
    const p1 = bindPropertyToRelation(p0, "rel:1");
    expect(p1.shape.relationId).toBe("rel:1");
    expect(p1.shape.entity).toBeUndefined();
    expect(p1.revision).toBe(p0.revision + 1);

    // Bind back to entity (clears relation)
    const p2 = bindPropertyToEntity(p1, { id: "E4", type: "system.Thing" });
    expect(p2.shape.entity?.id).toBe("E4");
    expect(p2.shape.relationId).toBeUndefined();
    expect(p2.revision).toBe(p1.revision + 1);

    // Schema validates final doc
    expect(() => PropertySchema.parse(p2)).not.toThrow();
  });
});

