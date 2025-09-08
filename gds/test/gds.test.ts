import { describe, it, expect } from "vitest";
import { gdsl } from '@organon/gdsl';

describe("@organon/gdsl resolution", () => {
  it('imports from "@organon/gdsl" and exports "core"', () => {
    expect(gdsl).toBe("gdsl");
  });
});
