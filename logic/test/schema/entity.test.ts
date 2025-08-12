import { describe, it, expect } from 'vitest';
import {
  createEntity,
  updateEntity,
  EntitySchema,
  EntityRef,
  isEntityRef,
} from '../../src/schema/entity';

describe('schema/entity', () => {
  it('createEntity -> valid schema; id stable across updates; revision increments', () => {
    const a = createEntity({ type: 'system.Entity', name: 'A' });
    const p1 = EntitySchema.parse(a);
    const id = p1.shape.core.id;
    expect(id).toBeTruthy();

    const next = updateEntity(p1, { core: { name: 'B' } } as any);
    const p2 = EntitySchema.parse(next);
    expect(p2.shape.core.id).toBe(id);
    expect((p2 as any).revision ?? 0).toBeGreaterThan(
      (p1 as any).revision ?? 0,
    );
  });

  it('EntityRef.parse validates and isEntityRef matches', () => {
    const e = createEntity({ type: 'system.Entity' });
    const ref = EntityRef.parse({
      id: e.shape.core.id,
      type: e.shape.core.type,
    });
    expect(isEntityRef(ref)).toBe(true);
  });

  it('EntityRef.parse rejects invalid ref', () => {
    try {
      EntityRef.parse({ id: 1, type: null } as any);
      expect(false).toBe(true);
    } catch (err: any) {
      expect(err.issues?.length).toBeGreaterThan(0);
    }
  });
});
