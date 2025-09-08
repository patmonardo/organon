import { describe, it, expect } from 'vitest';
import { gdsl } from '@organon/gdsl';
// import { model } from '@organon/model';
// import { logic } from '@organon/logic';

describe('@organon/gdsl resolution', () => {
  it('imports from "@organon/gdsl" and exports "task"', () => {
    expect(gdsl).toBe('gdsl');
  });
});
