import { describe, it, expect } from 'vitest';
import { reflectStage } from '../../src/absolute/essence/reflect';

describe('absolute/reflect — basic facets and signatures', () => {
  it('computes stable signatures and evidence for things and properties', async () => {
    const things = [{ id: 'T1', type: 'system.Thing', essence: { a: 1 } }];
    const properties = [
      { id: 'P1', entity: { id: 'T1', type: 'system.Thing' }, key: 'color', value: 'red' },
    ];

    const res = await reflectStage(things as any, properties as any, { contextId: 'ctx1' });
    expect(res.signatures).toBeDefined();
    expect(res.signatures?.thing?.T1).toBeDefined();
    expect(res.signatures?.property?.P1).toBeDefined();
    expect(res.evidence?.length).toBeGreaterThan(0);

    // Running again with same inputs returns same signature
    const res2 = await reflectStage(things as any, properties as any, { contextId: 'ctx1' });
    expect(res2.signatures?.thing?.T1).toEqual(res.signatures?.thing?.T1);
    expect(res2.signatures?.property?.P1).toEqual(res.signatures?.property?.P1);
  });
});
