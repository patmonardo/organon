import { describe, it, expect } from "vitest";
import { FormRelation } from "../../src/form/relation/relation";
import { createEntity, createEntityRef } from "../../src/schema/entity";

describe("FormRelation (principle)", () => {
  it("creates directed relation with defaults", () => {
    const a = createEntity({ type: "system.Entity", name: "A" });
    const b = createEntity({ type: "system.Entity", name: "B" });
    const rel = FormRelation.create({
      type: "system.Relation",
      kind: "related_to",
      source: createEntityRef(a),
      target: createEntityRef(b),
    });

    expect(rel.id).toBeTruthy();
    expect(rel.type).toBe("system.Relation");
    expect(rel.kind).toBe("related_to");
    expect(rel.source.id).toBe(a.shape.core.id);
    expect(rel.target.id).toBe(b.shape.core.id);
    expect(rel.direction).toBe("directed");
    expect(rel.revision).toBe(0);
  });

  it("updates core/state via schema-safe mutators and bumps revision", () => {
    const a = createEntity({ type: "system.Entity", name: "A" });
    const b = createEntity({ type: "system.Entity", name: "B" });
    const rel = FormRelation.create({
      type: "system.Relation",
      kind: "related_to",
      source: createEntityRef(a),
      target: createEntityRef(b),
    });

    const r0 = rel.revision;
    rel
      .setKind("contains")
      .setName("A->B")
      .setDescription("desc")
      .setStatus("active")
      .setStrength(0.7)
      .addTag("graph")
      .patchMeta({ note: "ok" });

    expect(rel.kind).toBe("contains");
    expect(rel.name).toBe("A->B");
    expect(rel.description).toBe("desc");
    expect(rel.status).toBe("active");
    expect(rel.strength).toBeCloseTo(0.7, 5);
    expect(rel.tags).toContain("graph");
    expect((rel.meta as any).note).toBe("ok");
    expect(rel.revision).toBeGreaterThan(r0);

    // idempotent tag removal
    rel.removeTag("graph").removeTag("graph");
    expect(rel.tags).not.toContain("graph");
  });

  it("inverts endpoints for directed relations", () => {
    const a = createEntity({ type: "system.Entity", name: "A" });
    const b = createEntity({ type: "system.Entity", name: "B" });
    const rel = FormRelation.create({
      type: "system.Relation",
      kind: "related_to",
      source: createEntityRef(a),
      target: createEntityRef(b),
    });

    const src0 = rel.source.id;
    const tgt0 = rel.target.id;
    rel.invert();
    expect(rel.source.id).toBe(tgt0);
    expect(rel.target.id).toBe(src0);
  });

  it("invert is a no-op for bidirectional relations", () => {
    const a = createEntity({ type: "system.Entity", name: "A" });
    const b = createEntity({ type: "system.Entity", name: "B" });
    const rel = FormRelation.create({
      type: "system.Relation",
      kind: "peer",
      source: createEntityRef(a),
      target: createEntityRef(b),
      direction: "bidirectional",
    });

    const src0 = rel.source.id;
    const tgt0 = rel.target.id;
    rel.invert();
    expect(rel.direction).toBe("bidirectional");
    expect(rel.source.id).toBe(src0);
    expect(rel.target.id).toBe(tgt0);
  });

  it("sets endpoints and direction explicitly", () => {
    const a = createEntity({ type: "system.Entity", name: "A" });
    const b = createEntity({ type: "system.Entity", name: "B" });
    const rel = FormRelation.create({
      type: "system.Relation",
      kind: "related_to",
      source: createEntityRef(a),
      target: createEntityRef(b),
    });

    rel
      .setEndpoints(createEntityRef(b), createEntityRef(a))
      .setDirection("bidirectional");
    expect(rel.source.id).toBe(b.shape.core.id);
    expect(rel.target.id).toBe(a.shape.core.id);
    expect(rel.direction).toBe("bidirectional");
  });

  it("toJSON returns the underlying schema doc", () => {
    const a = createEntity({ type: "system.Entity", name: "A" });
    const b = createEntity({ type: "system.Entity", name: "B" });
    const rel = FormRelation.create({
      type: "system.Relation",
      kind: "related_to",
      source: createEntityRef(a),
      target: createEntityRef(b),
    });

    const json = rel.toJSON();
    expect(json.shape.core.id).toBe(rel.id);
    expect(json.shape.source.id).toBe(rel.source.id);
    expect(json.shape.target.id).toBe(rel.target.id);
  });
});
