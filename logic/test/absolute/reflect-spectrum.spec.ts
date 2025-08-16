import { describe, it, expect } from 'vitest';
import { reflectStage } from '../../../logic/src/absolute/essence/reflect';

describe('reflectStage -> spectrum', () => {
  it('attaches a deterministic spectrum to thing facets', async () => {
    const thing = { id: 'thing:1', type: 'Example', essence: { soul: true } };
    const props = [
      { id: 'p1', entity: { id: 'thing:1' }, key: 'goal', value: 'be' },
      { id: 'p2', entity: { id: 'thing:1' }, key: 'color', value: 'red' },
    ];

    const res1 = await reflectStage([thing as any], props as any);
    const res2 = await reflectStage([thing as any], props as any);

    const f1 = res1.thingFacets?.['thing:1'];
    const f2 = res2.thingFacets?.['thing:1'];

    expect(f1).toBeDefined();
    expect(f1?.spectrum).toBeDefined();
    expect(typeof f1?.spectrum?.signature).toBe('string');
    // deterministic signature across runs with same inputs
    expect(f1?.spectrum?.signature).toEqual(f2?.spectrum?.signature);
    // intensity in range
    expect(f1?.spectrum?.intensity).toBeGreaterThanOrEqual(0);
    expect(f1?.spectrum?.intensity).toBeLessThanOrEqual(1);
  });
});
