import { describe, it, expect } from 'vitest';
import { DefaultEssenceDriver } from '../../src/absolute/essence/essence';
import { DefaultThingDriver } from '../../src/absolute/essence/thing';
import { DefaultReflectDriver } from '../../src/absolute/essence/reflect';
import { DefaultWorldDriver } from '../../src/absolute/essence/world';
import { makeInMemoryRepository } from '../../src/repository/memory';
import { ShapeSchema } from '../../src/schema/shape';
import { EntitySchema } from '../../src/schema/entity';
import { ContextSchema } from '../../src/schema/context';
import { PropertySchema } from '../../src/schema/property';
import { InMemoryEventBus } from '../../src/form/triad/bus';
import { ContextEngine } from '../../src/form/context/engine';

describe('Quad driver integration — Essence → Thing → Reflect → World', () => {
  it('makes a Fact appear: entities exist, context records thought, properties appear', async () => {
    // Repos per pillar
    const shapeRepo = makeInMemoryRepository(ShapeSchema as any);
    const entityRepo = makeInMemoryRepository(EntitySchema as any);
    const contextRepo = makeInMemoryRepository(ContextSchema as any);
    const propertyRepo = makeInMemoryRepository(PropertySchema as any);
    const bus = new InMemoryEventBus();

    const seen: string[] = [];
    bus.subscribe('shape.created', (e) => seen.push(e.kind));
    bus.subscribe('entity.created', (e) => seen.push(e.kind));
    bus.subscribe('context.created', (e) => seen.push(e.kind));
    bus.subscribe('property.created', (e) => seen.push(e.kind));

    // 1) Essence: conceive two shapes (minimal)
    const shapes = [
      { id: 'shape:fact', name: 'Fact', active: true },
      { id: 'shape:actor', name: 'Actor', active: true },
    ];
    const essCommit = await DefaultEssenceDriver.commitShapes(shapes, { repo: shapeRepo, bus });
    expect(essCommit.commitResult.success).toBe(true);

    // 2) Thing: bring two entities into existence
    const entities = [
      { id: 'thing:1', entityType: 'system.Entity', active: true },
      { id: 'thing:2', entityType: 'system.Entity', active: true },
    ];
    const thingCommit = await DefaultThingDriver.commitEntities(entities, { repo: entityRepo, bus });
    expect(thingCommit.commitResult.success).toBe(true);

    // Sanity: entity persisted
    const e1 = await entityRepo.get('thing:1');
    expect(e1).toBeTruthy();

    // 3) Reflect: record a context representing a chain of thought over those entities
    const context = {
      id: 'context:chain',
      kind: 'domain',
      name: 'Chain of Thought',
      scope: { ids: ['thing:1', 'thing:2'] },
      active: true,
    };
    const reflectCommit = await DefaultReflectDriver.commitContexts([context], { repo: contextRepo, bus });
    expect(reflectCommit.commitResult.success).toBe(true);

    // Sanity: context saved, then attach memberships via ContextEngine (chain of thought)
    const ctxEngine = new ContextEngine(contextRepo as any, bus);
    await ctxEngine.handle({
      kind: 'context.addEntities',
      payload: {
        id: 'context:chain',
        refs: [
          { id: 'thing:1', type: 'system.Entity' },
          { id: 'thing:2', type: 'system.Entity' },
        ],
      },
    } as any);
    const c1 = await contextRepo.get('context:chain');
    expect(c1).toBeTruthy();
    const entityIds = (((c1 as any)?.shape?.entities ?? []) as any[]).map((e) => e.id);
    expect(entityIds.sort()).toEqual(['thing:1', 'thing:2']);

    // 4) World: make a property appear for one of the entities
    const properties = [
      { id: 'p:status:thing:1', entity: { id: 'thing:1' }, key: 'status', value: 'active' },
    ];
    const worldCommit = await DefaultWorldDriver.commitPropertiesFromWorld(properties as any, { repo: propertyRepo, bus });
    expect(worldCommit.commitResult.success).toBe(true);

    // Property persisted
    const p1 = await propertyRepo.get('p:status:thing:1');
    expect(p1).toBeTruthy();

    // Events were seen across the chain (at least one per pillar)
    expect(seen).toContain('shape.created');
    expect(seen).toContain('entity.created');
    expect(seen).toContain('context.created');
    expect(seen).toContain('property.created');
  });
});
