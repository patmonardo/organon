import { describe, it, expect } from "vitest";
import { logic } from '@organon/logic';

describe("@organon/logic resolution", () => {
  it('imports from "@organon/logic" and exports "model"', () => {
    expect(logic).toBe("logic");
  });
});
