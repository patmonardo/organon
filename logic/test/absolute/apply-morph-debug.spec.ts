import { describe, it, expect } from 'vitest';
import { applyMorphRule } from '../../../logic/src/absolute/essence/ground';

describe('applyMorphRule debug', () => {
  it('deriveRelation morph produces relations', async () => {
    const morph: any = {
      id: 'm-debug',
      ruleSpec: {
        id: 'rs-debug-1',
        kind: 'deriveRelation',
        relationType: 'linked_to',
        source: { byId: 'thing:1' },
        target: { kind: 'fixed', targetEntityId: 'thing:2' },
        idempotent: true,
      }
    };

    const working = {
      entities: [{ id: 'thing:1' }, { id: 'thing:2' }],
      properties: [{ id: 'prop:1', entityId: 'thing:1', key: 'k', value: 'v' }],
      relations: []
    } as any;

    const res = await applyMorphRule(morph as any, working as any, {} as any);
    // console.log for debugging
    // eslint-disable-next-line no-console
    console.log('applyMorphRule result:', JSON.stringify(res, null, 2));
    expect(res.derivedRelations.length).toBeGreaterThanOrEqual(1);
  });
});
