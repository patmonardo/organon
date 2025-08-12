import { describe, it, expect } from 'vitest';
import {
  createRelation,
  updateRelation,
  RelationSchema,
  RelationDirection,
} from '../../src/schema/relation';
import { createEntity, EntityRef } from '../../src/schema/entity';

describe('schema/relation', () => {
  it('creates with endpoints and direction; schema validates', () => {
    const a = createEntity({ type: 'system.Entity', name: 'A' } as any);
    const b = createEntity({ type: 'system.Entity', name: 'B' } as any);
    const source = EntityRef.parse({
      id: a.shape.core.id,
      type: a.shape.core.type,
    });
    const target = EntityRef.parse({
      id: b.shape.core.id,
      type: b.shape.core.type,
    });

    const r0 = createRelation({
      type: 'system.Relation',
      kind: 'related_to',
      direction: 'directed',
      source,
      target,
    } as any);

    const p0 = RelationSchema.parse(r0);
    expect(p0.shape.core.id).toBeTruthy();
    expect(p0.shape.direction).toBe('directed');
    expect(p0.shape.source.id).toBe(source.id);
    expect(p0.shape.target.id).toBe(target.id);
  });

  it('updates direction and swaps endpoints via updateRelation', () => {
    const a = createEntity({ type: 'system.Entity', name: 'A' } as any);
    const b = createEntity({ type: 'system.Entity', name: 'B' } as any);
    const source = EntityRef.parse({
      id: a.shape.core.id,
      type: a.shape.core.type,
    });
    const target = EntityRef.parse({
      id: b.shape.core.id,
      type: b.shape.core.type,
    });

    const base = RelationSchema.parse(
      createRelation({
        type: 'system.Relation',
        kind: 'related_to',
        direction: 'directed',
        source,
        target,
      } as any),
    );

    // set direction
    const r1 = RelationSchema.parse(
      updateRelation(base, { direction: 'bidirectional' } as any),
    );
    expect(r1.shape.direction).toBe('bidirectional');

    // swap endpoints
    const r2 = RelationSchema.parse(
      updateRelation(r1, { source: target, target: source } as any),
    );
    expect(r2.shape.source.id).toBe(target.id);
    expect(r2.shape.target.id).toBe(source.id);
  });

  it('RelationDirection rejects invalid values', () => {
    expect(() => RelationDirection.parse('sideways')).toThrow();
    expect(RelationDirection.parse('directed')).toBe('directed');
    expect(RelationDirection.parse('bidirectional')).toBe('bidirectional');
  });
});
