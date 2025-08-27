import { describe, it, expect } from "vitest";
import { core } from '@organon/core';
// import { reality } from '@organon/reality';

describe("@organon/core resolution", () => {
  it('imports from "@organon/core and exports "logic"', () => {
    expect(core).toBe("core");
  });
});
