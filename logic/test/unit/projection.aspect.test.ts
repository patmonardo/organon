import { describe, it, expect } from 'vitest';
import { computeAspectProjection } from '../../src/absolute/form/projection.aspect';
import { ActiveRelationSchema, type ActiveRelation } from '../../src/schema/active';

function makeRel(id: string, type = 'related_to'): ActiveRelation {
  return ActiveRelationSchema.parse({
    id,
    kind: 'relation',
    particularityOf: `ess:${id}`,
    source: { id: 'a', type: 'system.Thing' },
    target: { id: 'b', type: 'system.Thing' },
    type,
    active: true,
  });
}

describe('computeAspectProjection', () => {
  it('derives higher aspects deterministically and idempotently', () => {
    const base = [makeRel('r1', 'owns'), makeRel('r2', 'likes')];
    const { upserts, deletes } = computeAspectProjection('marks', { relations: base });

    expect(deletes.length).toBe(0);
    expect(upserts.length).toBe(2);

    expect(upserts[0].id).toBe('proj:marks:r1');
    expect(upserts[0].type).toBe('has_mark:owns');

    expect(upserts[1].id).toBe('proj:marks:r2');
    expect(upserts[1].type).toBe('has_mark:likes');

    // idempotent: re-run yields same outputs
    const again = computeAspectProjection('marks', { relations: base });
    expect(JSON.stringify(again)).toBe(JSON.stringify({ upserts, deletes }));
  });
});
