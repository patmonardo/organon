import { describe, it, expect } from 'vitest';
import { ProcessorEngine, type PropertyAdapter } from './engine.js';
import type { PropertySpec } from './types.js';
import { PropertyAdapterRegistry } from './property-adapter.js';

// In-memory property adapter stub
const makeAdapter = (props: PropertySpec[]): PropertyAdapter => {
  const map = new Map(props.map(p => [p.id, p] as const));
  return {
    getProperty: (propId) => map.get(propId),
    listByWorld: (_worldId) => Array.from(map.values()),
  };
};

describe('ProcessorEngine', () => {
  it('creates world/form/entity and enforces closed form allowance', () => {
    const adapter = makeAdapter([
      { id: 'p:name', name: 'name', domain: ['Form:Person'], range: { kind: 'value', valueType: 'string' }, cardinality: { functional: true }, required: true },
      { id: 'p:friend', name: 'friend', domain: ['Form:Person'], range: { kind: 'entity', shapeIds: ['Form:Person'] }, cardinality: { max: 5 } },
      { id: 'p:age', name: 'age', domain: ['Form:Person'], range: { kind: 'value', valueType: 'number' } },
    ]);

    const engine = new ProcessorEngine(adapter);
    const world = engine.registerWorld('World:default', 'Default');
    const form = engine.createForm('Form:Person', 'Person', world.id, { open: false, allowed: new Set(['p:name', 'p:friend']) });
    const e1 = engine.createEntity('E:1', form.immediate.id);

    // Allowed property works
    engine.setField(e1.id, 'p:name', 'Alice');

    // Not allowed (p:age not in closed form allowed set)
    expect(() => engine.setField(e1.id, 'p:age', 42)).toThrowError(/not allowed/i);
  });

  it('enforces required and functional cardinality', () => {
    const adapter = makeAdapter([
      { id: 'p:name', name: 'name', domain: ['Form:Person'], range: { kind: 'value', valueType: 'string' }, cardinality: { functional: true }, required: true },
    ]);

    const engine = new ProcessorEngine(adapter);
    const world = engine.registerWorld('World:default', 'Default');
    const form = engine.createForm('Form:Person', 'Person', world.id, { open: false, allowed: new Set(['p:name']) });
    const e1 = engine.createEntity('E:1', form.immediate.id);

    // Missing required
    let res = engine.validate(e1.id);
    expect(res.ok).toBe(false);
    expect(res.errors.some(e => /required property/i.test(e))).toBe(true);

    // Set once
    engine.setField(e1.id, 'p:name', 'Alice');
    res = engine.validate(e1.id);
    expect(res.ok).toBe(true);

    // Functional replaces
    engine.setField(e1.id, 'p:name', 'Bob');
    res = engine.validate(e1.id);
    expect(res.ok).toBe(true);
  });

  it('open forms allow any world property present in the adapter', () => {
    const adapter = makeAdapter([
      { id: 'p:label', name: 'label', domain: ['Form:Thing'], range: { kind: 'value', valueType: 'string' } },
      { id: 'p:ref', name: 'ref', domain: ['Form:Thing'], range: { kind: 'entity', shapeIds: ['Form:Thing'] } },
    ]);

    const engine = new ProcessorEngine(adapter);
    const world = engine.registerWorld('World:default', 'Default');
    const form = engine.createForm('Form:Thing', 'Thing', world.id, { open: true });
    const a = engine.createEntity('E:A', form.immediate.id);

    engine.setField(a.id, 'p:label', 'A');
    engine.setField(a.id, 'p:ref', { entityId: 'E:B' });

    const res = engine.validate(a.id);
    expect(res.ok).toBe(true);
  });

  it('integrates with PropertyAdapterRegistry and respects domain + entity range', () => {
    const registry = new PropertyAdapterRegistry();
    registry.addMany([
      {
        worldId: 'World:default',
        id: 'p:label',
        name: 'label',
        domain: ['Form:Thing'],
        range: { kind: 'value', valueType: 'string' },
        required: true,
      },
      {
        worldId: 'World:default',
        id: 'p:link',
        name: 'link',
        domain: ['Form:Thing'],
        range: { kind: 'entity', shapeIds: ['Form:Thing'] },
      },
    ]);

    const engine = new ProcessorEngine(registry);
    const world = engine.registerWorld('World:default', 'Default');
    const form = engine.createForm('Form:Thing', 'Thing', world.id, { open: false, allowed: new Set(['p:label', 'p:link']) });
    const a = engine.createEntity('E:A', form.immediate.id);

    engine.setField(a.id, 'p:label', 'A');
    engine.setField(a.id, 'p:link', { entityId: 'E:B' });

    const res = engine.validate(a.id);
    expect(res.ok).toBe(true);
  });

  it('rejects linking to an entity whose form is not in the property range', () => {
    const registry = new PropertyAdapterRegistry();
    registry.addMany([
      { worldId: 'World:default', id: 'p:rel', name: 'rel', domain: ['Form:A'], range: { kind: 'entity', shapeIds: ['Form:B'] } },
    ]);

    const engine = new ProcessorEngine(registry);
    const world = engine.registerWorld('World:default', 'Default');
    const formA = engine.createForm('Form:A', 'A', world.id, { open: false, allowed: new Set(['p:rel']) });
    const formB = engine.createForm('Form:B', 'B', world.id, { open: false, allowed: new Set() });
    const a = engine.createEntity('E:A', formA.immediate.id);
    const b = engine.createEntity('E:B', formB.immediate.id);

    // Now attempt to link A.rel -> E:A (wrong form)
    expect(() => engine.setField(a.id, 'p:rel', { entityId: a.id })).toThrowError(/not allowed/);

    // Linking to B is allowed
    engine.setField(a.id, 'p:rel', { entityId: b.id });
    const res = engine.validate(a.id);
    expect(res.ok).toBe(true);
  });
});
