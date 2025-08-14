import { describe, it, expect } from "vitest";
import { ShapeEngine } from "../../src/form/shape/engine";
import { InMemoryEventBus } from "../../src/form/triad/bus";
import { makeInMemoryRepository } from "../support/inMemoryRepo";

describe("ShapeEngine", () => {
  it("creates, mutates data/state, describes, and deletes (persisting via repo)", async () => {
    const repo = makeInMemoryRepository<any>();
    const bus = new InMemoryEventBus();

    const received: string[] = [];
    const tap = (k: string) => bus.subscribe(k, (e) => received.push(e.kind));
    [
      "shape.created",
      "shape.state.set",
      "shape.state.patched",
      "shape.described",
      "shape.deleted",
    ].forEach(tap);

    const engine = new ShapeEngine(repo as any, bus);

    // create
    const [created] = await engine.handle({
      kind: "shape.create",
      payload: { type: "system.Shape", name: "Demo" },
    } as any);
    expect(created.kind).toBe("shape.created");
    const id = (created.payload as any).id as string;
    expect(id).toBeTruthy();

    // setState
    const [setState] = await engine.handle({
      kind: "shape.setState",
      payload: { id, state: { status: "active" } as any },
    } as any);
    expect(setState.kind).toBe("shape.state.set");
    expect(engine.getShape(id)?.state.status).toBe("active");

    // patchState
    const [patched] = await engine.handle({
      kind: "shape.patchState",
      payload: { id, patch: { meta: { message: "ok" } } as any },
    } as any);
    expect(patched.kind).toBe("shape.state.patched");
    expect((engine.getShape(id)?.state as any).meta.message).toBe("ok");

    // describe
    const [described] = await engine.handle({
      kind: "shape.describe",
      payload: { id },
    } as any);
    expect(described.kind).toBe("shape.described");
    expect((described.payload as any).id).toBe(id);

    // delete
    const [deleted] = await engine.handle({
      kind: "shape.delete",
      payload: { id },
    } as any);
    expect(deleted.kind).toBe("shape.deleted");
    expect(engine.getShape(id)).toBeUndefined();

    expect(received).toEqual([
      "shape.created",
      "shape.state.set",
      "shape.state.patched",
      "shape.described",
      "shape.deleted",
    ]);
  });
});
