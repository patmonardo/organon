import { describe, it, expect } from 'vitest';
import { createMorph, updateMorph, MorphSchema } from '../../src/schema/morph';

describe('schema/morph', () => {
  it('defaults input/output types and accepts optional transformFn', () => {
    const m0 = createMorph({ type: 'system.Morph', name: 'M0' } as any);
    const p0 = MorphSchema.parse(m0);

    const core0 = (p0 as any).shape?.core ?? (p0 as any).core ?? {};
    expect(core0.inputType).toBe('FormShape');
    expect(core0.outputType).toBe('FormShape');
    expect(core0.transformFn).toBeUndefined();

    const m1 = updateMorph(p0, { core: { transformFn: 'T.fn' } } as any);
    const p1 = MorphSchema.parse(m1);
    const core1 = (p1 as any).shape?.core ?? (p1 as any).core ?? {};
    expect(core1.transformFn).toBe('T.fn');
  });

  it('preserves id and increments revision on update', () => {
    const m0 = createMorph({ type: 'system.Morph', name: 'Rev' } as any);
    const p0 = MorphSchema.parse(m0);
    const id0 = (p0 as any).shape?.core?.id ?? (p0 as any).core?.id;
    const r0 = (p0 as any).revision ?? 0;

    const m1 = updateMorph(p0, { core: { name: 'Rev2' } } as any);
    const p1 = MorphSchema.parse(m1);
    const id1 = (p1 as any).shape?.core?.id ?? (p1 as any).core?.id;
    const r1 = (p1 as any).revision ?? 0;

    expect(id1).toBe(id0);
    expect(r1).toBeGreaterThan(r0);
  });

  it('rejects invalid transformFn type when parsing raw doc', () => {
    const m0 = createMorph({ type: 'system.Morph', name: 'Bad' } as any);
    const p0 = MorphSchema.parse(m0);
    const core0 = (p0 as any).shape?.core ?? (p0 as any).core ?? {};

    // Manually inject invalid type to bypass helper validation
    const bad = {
      ...p0,
      shape: {
        ...(p0 as any).shape,
        core: { ...core0, transformFn: 123 as any },
      },
    };

    expect(() => MorphSchema.parse(bad as any)).toThrow();
  });
});
