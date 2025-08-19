import { describe, it, expect } from 'vitest';
import { RelationDriver } from '../../src/absolute/essence/relation.driver';
import { RelationEngine } from '../../src/form/relation/engine';
import { makeInMemoryRepository } from '../../src/repository/memory';
import { RelationSchema, type Relation } from '../../src/schema/relation';

// Helper to build a real engine with an in-memory repo
function buildEngine() {
  const repo = makeInMemoryRepository(RelationSchema as any) as unknown as {
    get(id: string): Promise<Relation | null>;
    create(doc: Relation): Promise<Relation>;
    update(id: string, mut: (d: Relation) => Relation): Promise<Relation>;
    delete(id: string): Promise<boolean>;
  };
  return new RelationEngine(repo as any);
}

describe('RelationDriver — ActiveRelation -> RelationEngine truth', () => {
  it('processes and commits ActiveRelations to create relations', async () => {
    const driver = new RelationDriver();
    const engine = buildEngine();

    const actives = driver.toActiveRelationBatch([
      {
        id: 'r:ab',
        kind: 'relation',
        particularityOf: 'abs:r:ab',
        source: { id: 'a', type: 'system.Thing' },
        target: { id: 'b', type: 'system.Thing' },
        type: 'related_to',
        active: true,
      },
    ]);

    const { actions, snapshot } = await engine.process(actives as any, [], undefined);
    expect(actions.length).toBe(1);
    expect(snapshot.count).toBe(1);
    expect(actions[0].type).toBe('relation.upsert');

    const { events } = await engine.commit(actions, snapshot);
    expect(events.some((e) => e.kind === 'relation.created')).toBe(true);
  });

  it('updates endpoints and core fields on re-commit', async () => {
    const driver = new RelationDriver();
    const engine = buildEngine();

    // initial create
    const first = driver.toActiveRelation({
      id: 'r:ab',
      kind: 'relation',
      particularityOf: 'abs:r:ab',
      source: { id: 'a', type: 'system.Thing' },
      target: { id: 'b', type: 'system.Thing' },
      type: 'related_to',
      active: true,
    });
    const { actions: a1, snapshot: s1 } = await engine.process([first] as any);
    await engine.commit(a1, s1);

    // change type and target
    const second = driver.toActiveRelation({
      id: 'r:ab',
      kind: 'essential',
      particularityOf: 'abs:r:ab',
      source: { id: 'a', type: 'system.Thing' },
      target: { id: 'c', type: 'system.Thing' },
      type: 'works_with',
      active: true,
    });
    const { actions: a2, snapshot: s2 } = await engine.process([second] as any);
    const { events } = await engine.commit(a2, s2);

    expect(events.some((e) => e.kind === 'relation.core.set')).toBe(true);
    expect(events.some((e) => e.kind === 'relation.endpoints.set')).toBe(true);
  });
});
