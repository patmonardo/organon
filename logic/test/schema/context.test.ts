import { describe, it, expect } from 'vitest';
import {
  createContext,
  updateContext,
  ContextSchema,
} from '../../src/schema/context';
import { createEntity, EntityRef } from '../../src/schema/entity';

describe('schema/context', () => {
  it('createContext -> valid schema; default memberships are arrays', () => {
    const c = createContext({ type: 'system.Context', name: 'C0' } as any);
    const parsed = ContextSchema.parse(c);
    expect(Array.isArray(parsed.shape.entities)).toBe(true);
    expect(Array.isArray(parsed.shape.relations)).toBe(true);
    expect(parsed.shape.entities.length).toBe(0);
    expect(parsed.shape.relations.length).toBe(0);
  });

  it('updateContext can add and remove entity refs', () => {
    const e = createEntity({ type: 'system.Entity', name: 'E0' } as any);
    const ref = EntityRef.parse({
      id: e.shape.core.id,
      type: e.shape.core.type,
    });

    const c0 = createContext({ type: 'system.Context', name: 'C1' } as any);
    const c1 = updateContext(c0, { entities: [ref] } as any);
    const p1 = ContextSchema.parse(c1);
    expect(p1.shape.entities.length).toBe(1);

    const c2 = updateContext(p1, { entities: [] } as any);
    const p2 = ContextSchema.parse(c2);
    expect(p2.shape.entities.length).toBe(0);
  });

  it('relations accept ids; schema validates array', () => {
    const c0 = createContext({ type: 'system.Context', name: 'C2' } as any);
    const c1 = updateContext(c0, { relations: ['rel:1', 'rel:2'] } as any);
    const p1 = ContextSchema.parse(c1);
    expect(Array.isArray(p1.shape.relations)).toBe(true);
    expect(p1.shape.relations.length).toBe(2);
  });

  it('rejects invalid entity refs in entities array', () => {
    const c0 = createContext({ type: 'system.Context', name: 'C3' } as any);

    const bad = {
      ...c0,
      shape: {
        ...c0.shape,
        entities: [{ id: 1 as any, type: null as any }],
      },
    } as any;

    expect(() => ContextSchema.parse(bad)).toThrow();
  });
});
