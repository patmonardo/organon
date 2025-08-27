import { describe, it, expect } from 'vitest';
import { model } from '@organon/model';
// import { logic } from '@organon/logic';
// const logic = 'logic';

describe('@organon/model resolution', () => {
  it('imports from "@organon/model" and exports "task"', () => {
    expect(model).toBe('model');
  });
});
