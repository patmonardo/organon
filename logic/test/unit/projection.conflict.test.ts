import { describe, it, expect } from 'vitest';
import { ActiveRelationSchema, type ActiveRelation } from '../../src/schema/active';
import { computeAspectConflicts } from '../../src/absolute/form/projection.conflict';

function rel(id: string, type: string, S = 's', O = 'o'): ActiveRelation {
  return ActiveRelationSchema.parse({
    id,
    kind: 'relation',
    particularityOf: `ess:${id}`,
    source: { id: S, type: 'system.Thing' },
    target: { id: O, type: 'system.Thing' },
    type,
    active: true,
  });
}

describe('computeAspectConflicts', () => {
  it('emits conflicts_with for incompatible pairs in same respect', () => {
    const a = rel('a', 'subset_of');
    const b = rel('b', 'disjoint_from');
    const c = rel('c', 'overlaps'); // incompatible with disjoint_from but not with subset_of

    const out = computeAspectConflicts({ relations: [c, a, b] });
    // pairs: (a,b) conflict, (b,c) conflict
    expect(out.upserts.length).toBe(2);
    expect(out.conflictedIds.sort()).toEqual(['a', 'b', 'c']);

    const ids = out.upserts.map(x => x.id).sort();
    expect(ids[0]).toBe('proj:conflict:a+b');
    expect(ids[1]).toBe('proj:conflict:b+c');

    // deterministic/idempotent
    const again = computeAspectConflicts({ relations: [a, b, c] });
    expect(JSON.stringify(again)).toBe(JSON.stringify(out));
  });
});
