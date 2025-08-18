import { describe, it, expect } from "vitest";
import { computePropertyFacets } from "../../src/absolute/essence/world";
import { reflectStage } from "../../src/absolute/essence/reflect";

describe("World property analytics mates with Reflect property facets", () => {
  const thing = { id: "t1" } as any;
  const props = [
    { id: "p1", entity: { id: "t1" }, key: "x", value: 1 },
    { id: "p2", entity: { id: "t1" }, key: "y", value: 2 },
  ];

  it("produces matching signatures and basic facet shapes", async () => {
    const worldRes = computePropertyFacets(props);
    const refl = await reflectStage([thing], props, { mode: "full" });
    const refFacets = refl.propertyFacets!;
    // signatures match for shared property ids
    for (const pid of Object.keys(worldRes.signatures)) {
      expect(worldRes.signatures[pid]).toEqual(refl.signatures!.property![pid]);
      expect(refFacets[pid].positing.key).toEqual((worldRes.propertyFacets[pid].positing as any).key);
      expect(typeof refFacets[pid].external.valueType).toBe("string");
      expect(typeof (worldRes.propertyFacets[pid].external as any).valueType).toBe("string");
    }
  });
});
