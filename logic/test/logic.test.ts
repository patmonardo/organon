import { describe, it, expect } from "vitest";
import { core } from '@organon/gds';
// import { reality } from '@organon/reality';

describe("@organon/gds resolution", () => {
  it('imports from "@organon/gds and exports "logic"', () => {
    expect(core).toBe("core");
  });
});
