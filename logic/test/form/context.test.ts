import { describe, it, expect } from 'vitest';
import { FormContext } from '../../src/form/context/context';
import { createEntity, createEntityRef } from '../../src/schema/entity';

describe('FormContext (principle)', () => {
  it('creates with defaults', () => {
    const ctx = FormContext.create({ type: 'system.Context', name: 'World' });
    expect(ctx.id).toBeTruthy();
    expect(ctx.type).toBe('system.Context');
    expect(ctx.name).toBe('World');
    expect(ctx.entities).toEqual([]);
    expect(ctx.relations).toEqual([]);
    expect(ctx.tags).toEqual([]);
    expect(ctx.revision).toBe(0);
  });

  it('adds/removes entities idempotently', () => {
    const ctx = FormContext.create({ type: 'system.Context', name: 'C' });
    const a = createEntity({ type: 'system.Entity', name: 'A' });
    const b = createEntity({ type: 'system.Entity', name: 'B' });
    const aRef = createEntityRef(a);
    const bRef = createEntityRef(b);

    ctx.addEntity(aRef).addEntity(bRef).addEntity(aRef); // duplicate ignored
    expect(ctx.entities).toHaveLength(2);

    const r0 = ctx.revision;
    ctx.removeEntity(aRef);
    expect(ctx.entities).toHaveLength(1);
    expect(ctx.revision).toBeGreaterThan(r0);

    ctx.clearEntities();
    expect(ctx.entities).toHaveLength(0);
  });

  it('adds/removes relation ids idempotently', () => {
    const ctx = FormContext.create({ type: 'system.Context' });
    ctx.addRelation('r1').addRelation('r2').addRelation('r2'); // duplicate ignored
    expect(ctx.relations.sort()).toEqual(['r1', 'r2']);

    ctx.removeRelation('r1');
    expect(ctx.relations).toEqual(['r2']);

    ctx.clearRelations();
    expect(ctx.relations).toEqual([]);
  });

  it('updates core/state via schema-safe mutators', () => {
    const ctx = FormContext.create({ type: 'system.Context', name: 'C' });
    const r0 = ctx.revision;

    ctx
      .setName('C2')
      .setDescription('desc')
      .setStatus('active')
      .addTag('root')
      .patchMeta({ color: 'blue' });

    expect(ctx.name).toBe('C2');
    expect(ctx.description).toBe('desc');
    expect(ctx.status).toBe('active');
    expect(ctx.tags).toContain('root');
    expect((ctx.meta as any).color).toBe('blue');
    expect(ctx.revision).toBeGreaterThan(r0);

    ctx.removeTag('root').removeTag('root'); // idempotent
    expect(ctx.tags).not.toContain('root');
  });
});
