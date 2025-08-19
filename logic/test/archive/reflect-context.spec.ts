import { describe, it, expect } from "vitest";
import { determineThingContext, reflectStage } from "../../src/absolute/essence/reflect";

describe("Reflect as ActiveContext (ADR 0020)", () => {
  const thing = { id: "t1", type: "system.Thing", essence: { a: 1 } };
  const props = [
    { id: "p1", entity: { id: "t1" }, key: "x", value: 1 },
    { id: "p2", entity: { id: "t1" }, key: "y", value: 2 },
  ];

  it("determineThingContext produces minimal context facet", () => {
    const facet = determineThingContext(thing, props);
    expect(facet.positing.id).toBe("t1");
    expect(facet.external.propertyCount).toBe(2);
    expect(typeof facet.determining.score).toBe("number");
  });

  it("reflectStage in context mode omits property facets and spectrum", async () => {
    const res = await reflectStage([thing], props, { mode: "context" });
    expect(res.thingFacets?.t1?.spectrum).toBeUndefined();
    expect(res.propertyFacets).toBeUndefined();
    expect(res.signatures?.property).toBeUndefined();
    expect(res.signatures?.thing?.t1).toBeTruthy();
  });
});
