import { describe, it, expect } from "vitest";
import { computeSignature, extractFacets, fromFormShape } from "../../src/absolute/essence/essence";

const mkShape = (over: Record<string, any> = {}) => ({
  id: over.id,
  type: over.type ?? "system.Shape",
  name: over.name ?? "TestShape",
  properties: over.properties ?? { a: { t: "number" }, b: { t: "string" } },
  fields: over.fields,
});

describe("ActiveShape driver", () => {
  it("computes deterministic signatures for the same shape", () => {
    const s1 = mkShape();
    const s2 = mkShape();
    // no id set, signatures based on structure should match
    const sig1 = computeSignature(s1);
    const sig2 = computeSignature(s2);
    expect(sig1).toEqual(sig2);
  });

  it("extracts stable facets and includes prop count", () => {
    const s = mkShape({ properties: { x: 1, y: 2, z: 3 } });
    const facets = extractFacets(s);
    const keys = facets.map(f => f.key);
    expect(keys).toContain("__propCount");
    const countFacet = facets.find(f => f.key === "__propCount");
    // extractShapeKeys includes property names and may also include top-level structural keys
    expect(countFacet?.value).toBeGreaterThanOrEqual(3);
  });

  it("fromFormShape produces signature and facets", () => {
    const s = mkShape();
    const active = fromFormShape(s);
    expect(active.signature).toBeTruthy();
    expect(active.facets.length).toBeGreaterThan(0);
  });
});
