import { describe, it, expect } from "vitest";
import { Repos } from "../../src/repository";
import {
  createEntity,
  createEntityRef,
  createRelation,
  createContext,
  createProperty,
} from "../../src/schema";

describe("InMemoryRepository + Pillars", () => {
  it("creates and updates Entity", async () => {
    const entities = Repos.entity();
    const e = await entities.create(createEntity({ type: "system.Entity", name: "A" }));
    expect(e.shape.core.id).toBeTruthy();
    expect(e.revision).toBe(0);

    const updated = await entities.update(
      e.shape.core.id,
      cur => ({
        ...cur,
        shape: { ...cur.shape, state: { ...cur.shape.state, tags: ["root"] } },
      }),
      { expectedRevision: e.revision },
    );

    expect(updated.revision).toBe(1);
    expect(updated.shape.state.tags).toContain("root");
  });

  it("creates Relation between two Entities", async () => {
    const entities = Repos.entity();
    const rels = Repos.relation();

    const a = await entities.create(createEntity({ type: "system.Entity", name: "A" }));
    const b = await entities.create(createEntity({ type: "system.Entity", name: "B" }));

    const r = await rels.create(
      createRelation({
        type: "system.Relation",
        kind: "related_to",
        source: createEntityRef(a),
        target: createEntityRef(b),
        direction: "directed",
      }),
    );

    expect(r.shape.core.kind).toBe("related_to");
    expect(r.shape.source.id).toBe(a.shape.core.id);
    expect(r.shape.target.id).toBe(b.shape.core.id);
  });

  it("creates Context with memberships", async () => {
    const entities = Repos.entity();
    const ctxs = Repos.context();
    const rels = Repos.relation();

    const a = await entities.create(createEntity({ type: "system.Entity", name: "A" }));
    const b = await entities.create(createEntity({ type: "system.Entity", name: "B" }));

    const r = await rels.create(
      createRelation({
        type: "system.Relation",
        kind: "related_to",
        source: createEntityRef(a),
        target: createEntityRef(b),
      }),
    );

    const ctx = await ctxs.create(
      createContext({
        type: "system.Context",
        name: "World",
        entities: [createEntityRef(a), createEntityRef(b)],
        relations: [r.shape.core.id],
      }),
    );

    expect(ctx.shape.entities).toHaveLength(2);
    expect(ctx.shape.relations).toHaveLength(1);
  });

  it("creates Property bound to an Entity", async () => {
    const entities = Repos.entity();
    const props = Repos.property();
    const ctxs = Repos.context();

    const e = await entities.create(createEntity({ type: "system.Entity", name: "K" }));
    const ctx = await ctxs.create(createContext({ type: "system.Context", name: "C" }));

    const p = await props.create(
      createProperty({
        type: "system.Property",
        key: "title",
        contextId: ctx.shape.core.id,
        entity: createEntityRef(e),
        value: "Hello",
        valueType: "string",
      }),
    );

    expect(p.shape.core.key).toBe("title");
    expect(p.shape.entity?.id).toBe(e.shape.core.id);
  });
});
