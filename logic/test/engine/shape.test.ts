import { describe, it, expect } from "vitest";
import { ShapeEngine } from "../../src/form/shape/engine";
import { InMemoryEventBus } from "../../src/form/triad/bus";
import { makeInMemoryRepository } from "../support/inMemoryRepo";

describe("ShapeEngine", () => {
  it("instantiates, mutates data/state, describes, and destroys (persisting via repo)", async () => {
    const repo = makeInMemoryRepository<any>();
    const bus = new InMemoryEventBus();

    const received: string[] = [];
    const tap = (k: string) => bus.subscribe(k, (e) => received.push(e.kind));
    [
      "shape.instantiated",
      "shape.data.set",
      "shape.state.set",
      "shape.state.patched",
      "shape.described",
      "shape.destroyed",
    ].forEach(tap);

    const engine = new ShapeEngine(repo as any, bus);

    // instantiate
    const [instantiated] = await engine.handle({
      kind: "shape.instantiate",
      payload: { definitionId: "def:1", definitionName: "Demo", data: { a: 1 } },
    } as any);
    expect(instantiated.kind).toBe("shape.instantiated");
    const id = (instantiated.payload as any).id as string;
    expect(id).toBeTruthy();

    // setData
    const [setData] = await engine.handle({
      kind: "shape.setData",
      payload: { id, data: { a: 2, b: { x: 1 } } },
    } as any);
    expect(setData.kind).toBe("shape.data.set");
    expect(engine.getShape(id)?.data).toEqual({ a: 2, b: { x: 1 } });

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
      payload: { id, patch: { message: "ok" } as any },
    } as any);
    expect(patched.kind).toBe("shape.state.patched");
    expect((engine.getShape(id)?.state as any).message).toBe("ok");

    // describe
    const [described] = await engine.handle({
      kind: "shape.describe",
      payload: { id },
    } as any);
    expect(described.kind).toBe("shape.described");
    expect((described.payload as any).id).toBe(id);

    // destroy
    const [destroyed] = await engine.handle({
      kind: "shape.destroy",
      payload: { id },
    } as any);
    expect(destroyed.kind).toBe("shape.destroyed");
    expect(engine.getShape(id)).toBeUndefined();

    expect(received).toEqual([
      "shape.instantiated",
      "shape.data.set",
      "shape.state.set",
      "shape.state.patched",
      "shape.described",
      "shape.destroyed",
    ]);
  });
});
