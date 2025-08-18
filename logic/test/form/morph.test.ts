import { describe, it, expect } from "vitest";
import { FormMorph } from "../../src/form/morph/morph";

describe("FormMorph (principle)", () => {
  it("defines and runs a simple morph", () => {
    const inc = FormMorph.define<number, number>("inc", (x) => x + 1);

    expect(inc.name).toBe("inc");
    expect(inc.options.pure).toBe(true);
    expect(inc.options.cost).toBe(1);
    expect(inc.run(1)).toBe(2);
  });

  it("composes morphs (sequential)", () => {
    const inc = FormMorph.define<number, number>("inc", (x) => x + 1);
    const dbl = FormMorph.define<number, number>("dbl", (x) => x * 2);

    const incThenDbl = FormMorph.compose(inc, dbl);
    expect(incThenDbl.name).toBe("inc➝dbl");
    expect(incThenDbl.options.pure).toBe(true);
    expect(incThenDbl.options.cost).toBe(2);

    expect(incThenDbl.run(2)).toBe(6); // (2 + 1) * 2
  });

  it("identity morph returns input unchanged", () => {
    const id = FormMorph.identity<number>("id");
    expect(id.options.cost).toBe(0);
    expect(id.run(42)).toBe(42);
  });

  it("attaches a post-processor", () => {
    const inc = FormMorph.define<number, number>("inc", (x) => x + 1);
    const withPost = inc.withPostProcessor((u: number) => `v=${u}`);

    expect(withPost.name).toBe("inc➝post");
    expect(withPost.options.pure).toBe(false); // post-processing marks impure
    expect(withPost.options.cost).toBe(2); // step (1) + post (1)

    expect(withPost.run(1)).toBe("v=2");
  });

  it("describe returns name and options", () => {
    const m = FormMorph.define<number, number>("triple", (x) => x * 3);
    const d = m.describe();
    expect(d.name).toBe("triple");
    expect(d.options.cost).toBe(1);
  });
});
