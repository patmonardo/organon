import { describe, it } from 'vitest';
import type { ActiveAny } from '../../src/absolute';
import { BaseDriver } from '../../src/absolute';

describe('absolute barrel exports', () => {
  it('type imports compile', () => {
    // compile-only test: assert types import from barrel (no runtime assertions needed)
    const _t: ActiveAny | null = null;
    void _t;
    void BaseDriver;
  });
});
