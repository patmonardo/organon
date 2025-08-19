import { describe, it, expect } from 'vitest';
import { DefaultThingDriver, processEntities, commitEntities } from '../../src/absolute/essence/thing';
import { EntityEngine } from '../../src/form/entity/engine';
import { makeInMemoryRepository } from '../../src/repository/memory';
import { EntitySchema } from '../../src/schema/entity';
import { InMemoryEventBus } from '../../src/form/triad/bus';

const repo = makeInMemoryRepository(EntitySchema as any);

describe('ThingDriver → EntityEngine (ActiveEntity)', () => {
  it('processes and commits ActiveEntities (create + update)', async () => {
    const entities = [
      { id: 'entity:one', entityType: 'system.Entity', active: true },
      { id: 'entity:two', entityType: 'system.Entity', active: true },
    ];

    // process via driver
    const { actions, snapshot } = await DefaultThingDriver.processEntities(entities, { repo });
    expect(snapshot.count).toBe(2);
    expect(actions.some(a => a.type === 'entity.upsert' && a.id === 'entity:one')).toBe(true);

    // commit via driver
    const result = await DefaultThingDriver.commitEntities(entities, { repo });
    expect(result.commitResult.success).toBe(true);

    // engine sanity: describe via engine
    const engine = new EntityEngine(repo as any);
    const [evt] = await engine.handle({ kind: 'entity.describe', payload: { id: 'entity:one' } } as any);
    expect(evt.kind).toBe('entity.described');
    expect((evt.payload as any).id).toBe('entity:one');
  });

  it('handles revoked (deletes)', async () => {
    // seed create
    await DefaultThingDriver.commitEntities([{ id: 'entity:x', entityType: 'system.Entity', active: true }], { repo });
    // revoke
    const { actions } = await processEntities([{ id: 'entity:x', entityType: 'system.Entity', revoked: true }], { repo });
    expect(actions.some(a => a.type === 'entity.delete' && a.id === 'entity:x')).toBe(true);
    const { commitResult } = await commitEntities([{ id: 'entity:x', entityType: 'system.Entity', revoked: true }], { repo });
    expect(commitResult.success).toBe(true);
  });

  it('publishes events on the provided bus (created + core.set in one batch)', async () => {
    const localRepo = makeInMemoryRepository(EntitySchema as any);
    const bus = new InMemoryEventBus();
    const events: string[] = [];
    bus.subscribe('entity.created', (e) => events.push(e.kind));
    bus.subscribe('entity.core.set', (e) => events.push(e.kind));

    const entities = [
      { id: 'entity:evt', entityType: 'system.Entity', active: true },
      { id: 'entity:evt', entityType: 'system.Entity', active: true },
    ];

    const { commitResult } = await commitEntities(entities, { repo: localRepo, bus });
    expect(commitResult.success).toBe(true);
    expect(events).toEqual(['entity.created', 'entity.core.set']);
  });
});
