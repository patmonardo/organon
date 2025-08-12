import { describe, it, expect } from "vitest";
import { FormEntity } from "../../src/form/entity";

describe("FormEntity (wrapper)", () => {
  it("creates with defaults and exposes core fields", () => {
    const e = FormEntity.create({ type: "system.Entity", name: "A" });
    expect(e.id).toBeTruthy();
    expect(e.type).toBe("system.Entity");
    expect(e.name).toBe("A");
    expect(Array.isArray(e.tags)).toBe(true);
    expect(typeof e.meta).toBe("object");
    const ref = e.toRef();
    expect(ref.id).toBe(e.id);
    expect(ref.type).toBe(e.type);
    expect(e.key).toBe(`${ref.type}:${ref.id}`);
  });

  it("updates via schema-safe mutators and bumps revision", () => {
    const e = FormEntity.create({ type: "system.Entity", name: "A" });
    const r0 = e.toSchema().revision;

    e.addTag("root").setName("B").setDescription("desc").patchMeta({ color: "blue" });

    expect(e.name).toBe("B");
    expect(e.description).toBe("desc");
    expect(e.tags).toContain("root");
    expect((e.meta as any).color).toBe("blue");
    expect(e.toSchema().revision).toBeGreaterThan(r0);
  });

  it("removes tags idempotently", () => {
    const e = FormEntity.create({ type: "system.Entity", name: "A" });
    e.addTag("root").addTag("root"); // no duplicates
    expect(e.tags.filter(t => t === "root").length).toBe(1);
    e.removeTag("root");
    expect(e.tags).not.toContain("root");
    e.removeTag("root"); // idempotent
    expect(e.tags).not.toContain("root");
  });
});
