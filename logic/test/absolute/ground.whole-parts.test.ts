import { describe, it, expect } from 'vitest';
import { groundToEssentialBridge } from '../../src/absolute/essence/ground';

describe('Ground → Essential bridge (whole & parts)', () => {
  it('creates or marks an essential relation when ground aggregates parts', () => {
    const ground: any = {
      id: 'g:1',
      type: 'related_to:absolute',
      kind: 'absolute',
      sourceId: 'S',
      targetId: 'T',
      contributingConditionIds: ['c1', 'c2', 'c3'],
    };

    const candidates: any[] = [];

    const res = groundToEssentialBridge(ground as any, candidates, { wholeThreshold: 3 });
    expect(res).toBeDefined();
    expect(res?.particularityOf).toBe('g:1');
    expect(res?.provenance?.modality?.kind).toBe('actual');
    // ensure candidate was added
    expect(candidates.find((c) => c.id === res?.id)).toBeDefined();
  });
});
