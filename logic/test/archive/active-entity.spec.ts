import { describe, it, expect } from "vitest";
import { ActiveEntity } from "../../src/absolute/essence/thing";

const mkEntity = (over: Record<string, any> = {}) => ({
  id: over.id,
  type: over.type ?? "system.Entity",
  name: over.name ?? "TestEntity",
  properties: over.properties ?? { a: 1, b: "two" },
});

describe("ActiveEntity driver", () => {
  it("wraps raw entity and exposes signature and facets", () => {
    const e = new ActiveEntity(mkEntity());
    expect(e.signature).toBeTruthy();
    expect(e.facets.length).toBeGreaterThan(0);
  });

  it("runtimeMeta includes signature and facets", () => {
    const e = new ActiveEntity(mkEntity());
    const meta = e.runtimeMeta();
    expect(meta.signature).toBe(e.signature);
    expect(Array.isArray(meta.facets)).toBe(true);
  });
});
