import { describe, it, expect } from "vitest";
import { reality } from '@organon/reality';

describe("@organon/reality resolution", () => {
  it('imports from "@organon/reality" and exports "core"', () => {
    expect(reality).toBe("reality");
  });
});
