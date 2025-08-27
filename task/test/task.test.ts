import { describe, it, expect } from 'vitest';
import { task } from '@organon/logic';

describe('@organon/logic/logic resolution', () => {
  it('imports from "@organon/logic/logic" and exports "logic"', () => {
    expect(task).toBe('task');
  });
});
