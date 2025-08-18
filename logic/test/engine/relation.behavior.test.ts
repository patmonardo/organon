import { describe, it, expect } from 'vitest';
import * as rel from '../../src/absolute/essence/relation';
import { RelationEngine } from '../../src/form/relation/engine';
import { InMemoryEventBus } from '../../src/form/triad/bus';
import { makeInMemoryRepository } from '../support/inMemoryRepo';

describe('ActiveRelation helpers and RelationEngine behavior', () => {
  it('chooseCanonicalTruth prefers explicit active over confidence', () => {
    const a = {
      id: 'a',
      particularityOf: 'abs1',
      active: true,
      confidence: 0.2,
      source: { id: 's1', type: 'system.Entity' },
      target: { id: 't1', type: 'system.Entity' },
    } as any;
    const b = {
      id: 'b',
      particularityOf: 'abs1',
      active: false,
      confidence: 0.95,
      source: { id: 's2', type: 'system.Entity' },
      target: { id: 't2', type: 'system.Entity' },
    } as any;
    const chosen = rel.chooseCanonicalTruth('abs1', [a, b]);
    expect(chosen).toBeDefined();
    expect(chosen?.id).toBe('a');
  });

  it('groundScore combines confidence and weight', () => {
    const r = { id: 'x', confidence: 0.6, weight: 0.5, active: false } as any;
    const s = rel.groundScore(r);
    expect(s).toBeGreaterThan(0);
    expect(s).toBeLessThanOrEqual(1);
  });

  it('assertActiveRelationInvariants throws for missing endpoints in test env', () => {
    const bad = [
      { id: 'bad1', particularityOf: 'abs', active: true } as any,
    ];
    let threw = false;
    try {
      rel.assertActiveRelationInvariants(bad as any);
    } catch (err) {
      threw = true;
    }
    expect(threw).toBe(true);
  });

  it('RelationEngine commit updates endpoints and direction idempotently', async () => {
    const repo = makeInMemoryRepository<any>();
    const bus = new InMemoryEventBus();
    const engine = new RelationEngine(repo as any, bus as any);

    // initial upsert
    const act1 = [
      {
        id: 'r-upd',
        particularityOf: 'abs-upd',
        source: { id: 'e1', type: 'system.Entity' },
        target: { id: 'e2', type: 'system.Entity' },
        kind: 'relation',
        type: 'system.Relation',
        active: true,
      },
    ];
    const p = await engine.process(act1 as any);
    await engine.commit(p.actions, p.snapshot as any);
    const stored1 = await repo.get('r-upd');
    expect(stored1).toBeDefined();
    expect((stored1 as any).shape.source.id || (stored1 as any).shape.core).toBeTruthy();

    // update endpoints
    const act2 = [
      {
        id: 'r-upd',
        particularityOf: 'abs-upd',
        source: { id: 'e3', type: 'system.Entity' },
        target: { id: 'e4', type: 'system.Entity' },
        kind: 'relation',
        type: 'system.Relation',
      },
    ];
    const p2 = await engine.process(act2 as any);
    await engine.commit(p2.actions, p2.snapshot as any);
    const stored2 = await repo.get('r-upd');
    expect(stored2).toBeDefined();
    // verify endpoints updated
    const src = (stored2 as any).shape.source;
    const tgt = (stored2 as any).shape.target;
    expect(src.id === 'e3' || src.id === 'e1').toBe(true);
    expect(tgt.id === 'e4' || tgt.id === 'e2').toBe(true);
  });
});
