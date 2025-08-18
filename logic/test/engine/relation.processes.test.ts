import { describe, it, expect } from 'vitest';
import { RelationEngine } from '../../src/form/relation/engine';
import { InMemoryEventBus } from '../../src/form/triad/bus';
import { makeInMemoryRepository } from '../support/inMemoryRepo';
import * as schemas from '../../src/absolute/essence/schemas';

describe('RelationEngine ADR-0007 process/commit', () => {
  it('processes upsert and delete actions and commit applies them idempotently', async () => {
  const repo = makeInMemoryRepository<any>();
  const bus = new InMemoryEventBus();
  const engine = new RelationEngine(repo as any, bus as any);

    const active: Array<schemas.ActiveRelation> = [
      schemas.ActiveRelationSchema.parse({
        id: 'r1',
        kind: 'relation',
        particularityOf: 'abs:r1',
        source: { id: 'e1', type: 'system.Entity' },
        target: { id: 'e2', type: 'system.Entity' },
        type: 'system.Relation',
        active: true,
      }),
    ];

    const { actions, snapshot } = await engine.process(active);
    expect(snapshot.count).toBe(1);
    expect(actions.length).toBe(1);
    expect(actions[0].type).toBe('relation.upsert');

    const res = await engine.commit(actions, snapshot as any);
    expect(res.success).toBe(true);
    // commit again should be idempotent
    const res2 = await engine.commit(actions, snapshot as any);
    expect(res2.success).toBe(true);

    // Now revoke relation and ensure delete action produced
    const revoked = [
      schemas.ActiveRelationSchema.parse({
        id: 'r1',
        kind: 'relation',
        particularityOf: 'abs:r1',
        source: { id: 'e1', type: 'system.Entity' },
        target: { id: 'e2', type: 'system.Entity' },
        revoked: true,
      }),
    ];
    const { actions: delActions } = await engine.process(revoked as any);
    expect(delActions[0].type).toBe('relation.delete');

    const delRes = await engine.commit(delActions, { count: 1 } as any);
    expect(delRes.success).toBe(true);
  });
});
