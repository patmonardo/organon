import { describe, it, expect } from "vitest";
import { FormProperty } from "../../src/form/property/property";
import { createEntity, createEntityRef } from "../../src/schema/entity";

describe("FormProperty (wrapper)", () => {
  it("creates with defaults", () => {
    const p = FormProperty.create({
      type: "system.Property",
      key: "title",
      contextId: "ctx:1",
    });
    expect(p.id).toBeTruthy();
    expect(p.type).toBe("system.Property");
    expect(p.key).toBe("title");
    expect(p.contextId).toBe("ctx:1");
    expect(p.tags).toEqual([]);
    expect(p.value).toBeUndefined();
    expect(p.valueType).toBeUndefined();
    expect(p.revision).toBe(0);
  });

  it("binds to entity, switches to relation, and clears binding (exclusive)", () => {
    const e = createEntity({ type: "system.Entity", name: "A" });
    const eRef = createEntityRef(e);

    const p = FormProperty.create({
      type: "system.Property",
      key: "owner",
      contextId: "ctx:2",
    });

    // bind to entity
    p.bindToEntity(eRef);
    expect(p.entity?.id).toBe(eRef.id);
    expect(p.relationId).toBeUndefined();

    // switch to relation
    p.bindToRelation("rel:1");
    expect(p.entity).toBeUndefined();
    expect(p.relationId).toBe("rel:1");

    // clear binding
    p.clearBinding();
    expect(p.entity).toBeUndefined();
    expect(p.relationId).toBeUndefined();
  });

  it("sets and clears value and valueType", () => {
    const p = FormProperty.create({
      type: "system.Property",
      key: "count",
      contextId: "ctx:3",
    });
    p.setValue(42).setValueType("number");
    expect(p.value).toBe(42);
    expect(p.valueType).toBe("number");

    p.clearValue().setValueType(undefined);
    expect(p.value).toBeUndefined();
    expect(p.valueType).toBeUndefined();
  });

  it("updates core/state via schema-safe mutators and bumps revision", () => {
    const p = FormProperty.create({
      type: "system.Property",
      key: "color",
      contextId: "ctx:4",
    });
    const r0 = p.revision;

    p.setName("Color")
      .setDescription("Primary color")
      .setStatus("active")
      .addTag("ui")
      .patchMeta({ group: "style" })
      .setContext("ctx:5");

    expect(p.name).toBe("Color");
    expect(p.description).toBe("Primary color");
    expect(p.status).toBe("active");
    expect(p.tags).toContain("ui");
    expect((p.meta as any).group).toBe("style");
    expect(p.contextId).toBe("ctx:5");
    expect(p.revision).toBeGreaterThan(r0);

    // idempotent tag removal
    p.removeTag("ui").removeTag("ui");
    expect(p.tags).not.toContain("ui");
  });
});
