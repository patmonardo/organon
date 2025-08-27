import { describe, it, expect } from "vitest";
import { task } from '@organon/task';
// const task = "task";

describe("@organon/task/task resolution", () => {
  it('imports from "@organon/task/task" and exports "task"', () => {
    expect(task).toBe("task");
  });
});
