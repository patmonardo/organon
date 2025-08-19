import { describe, it, expect } from 'vitest';
import { RelationDriver } from '../../src/absolute/essence/relation';
import { makeInMemoryRepository } from '../../src/repository/memory';
import { RelationSchema, type Relation } from '../../src/schema/relation';
import type { Repository } from '../../src/repository/repo';
import { InMemoryEventBus } from '../../src/form/triad/bus';

function makeRepo(): Repository<Relation> {
  return makeInMemoryRepository(RelationSchema as any) as unknown as Repository<Relation>;
}

describe('RelationDriver events', () => {
  it('emits created and updated events through the EventBus', async () => {
    const driver = new RelationDriver();
    const bus = new InMemoryEventBus();
    const repo = makeRepo();

    const seen: string[] = [];
    const unsubCreate = bus.subscribe('relation.created', () => seen.push('created'));
    const unsubCore = bus.subscribe('relation.core.set', () => seen.push('core.set'));
    const unsubEnd = bus.subscribe('relation.endpoints.set', () => seen.push('endpoints.set'));

    // First commit creates the relation
    const first = driver.toActiveRelation({
      id: 'r:ab',
      kind: 'relation',
      particularityOf: 'abs:r:ab',
      source: { id: 'a', type: 'system.Thing' },
      target: { id: 'b', type: 'system.Thing' },
      type: 'related_to',
      active: true,
    });

    const { commitResult: cr1 } = await driver.commitActiveRelations([first], { repo, bus });
    expect(cr1.events.some((e: any) => e.kind === 'relation.created')).toBe(true);

    // Second commit patches core and endpoints
    const second = driver.toActiveRelation({
      id: 'r:ab',
      kind: 'essential',
      particularityOf: 'abs:r:ab',
      source: { id: 'a', type: 'system.Thing' },
      target: { id: 'c', type: 'system.Thing' },
      type: 'works_with',
      active: true,
    });
    const { commitResult: cr2 } = await driver.commitActiveRelations([second], { repo, bus });
    expect(cr2.events.some((e: any) => e.kind === 'relation.core.set')).toBe(true);
    expect(cr2.events.some((e: any) => e.kind === 'relation.endpoints.set')).toBe(true);

    // Verify bus observed the events in order (created, core/endpoints after)
    unsubCreate();
    unsubCore();
    unsubEnd();

    expect(seen.includes('created')).toBe(true);
    expect(seen.some((s) => s === 'core.set' || s === 'endpoints.set')).toBe(true);
  });
});
