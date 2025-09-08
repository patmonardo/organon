import { describe, it, expect } from "vitest";
import { gdsl } from "@organon/gdsl";

describe("@organon/gdsl/gdsl resolution", () => {
  it('imports from "@organon/gdsl/gdsl" and exports "gdsl"', () => {
    expect(gdsl).toBe("gdsl");
  });
});
