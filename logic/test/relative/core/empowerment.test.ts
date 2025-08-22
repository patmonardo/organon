import { describe, it, expect } from 'vitest';
import FormEmpowerment from '../../../src/relative/core/FormEmpowerment';
import { combineEmpowerments } from '../../../src/relative/core/empowerment-processor';

describe('empowerment processor', () => {
  it('combines scores correctly (happy path)', () => {
    const a = FormEmpowerment.fromSchema({
      id: 't1',
      subject: 'u1',
      actions: ['create'],
      weight: 2,
      jnana: 0.5,
    });

    const b = FormEmpowerment.fromSchema({
      id: 't2',
      subject: 'u2',
      actions: ['create'],
      weight: 1,
      jnana: 1,
    });

    const res = combineEmpowerments([a, b]);
    expect(res.total).toBeCloseTo(a.score() + b.score());
    expect(res.scores).toHaveLength(2);
  });

  it('includes root fallback and tie cases', () => {
    const t = FormEmpowerment.fromSchema({
      id: 't3',
      subject: 'u3',
      actions: ['*'],
      weight: 1,
      jnana: 0.5,
    });

    const root = FormEmpowerment.createRoot();
    const res = combineEmpowerments([t], root);
    expect(res.scores.some((s) => s.id === root.id)).toBe(true);
    expect(res.total).toBeGreaterThan(0);
  });
});
