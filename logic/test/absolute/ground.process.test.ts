import { describe, it, expect } from 'vitest';
import { groundStage, toActiveFromGround } from '../../src/absolute/essence/ground';
import * as schemas from '../../src/absolute/essence/schemas';

describe('groundStage -> toActiveFromGround mapping', () => {
  it('produces ActiveProperty and ActiveRelation carriers', async () => {
    const morphs = [
      {
        id: 'm1',
        transform: 'derive.relation',
        params: {},
        active: true,
      },
    ];
    const entities = [
      { id: 'e1', type: 'system.Entity' },
      { id: 'e2', type: 'system.Entity' },
    ];
    const properties = [
      { id: 'p1', entityId: 'e1', key: 'relatesTo', value: 'e2' },
    ];

    const res = await groundStage({ morphs }, { entities, properties } as any, {} as any);
    const active = toActiveFromGround(res as any);

    expect(Array.isArray(active.properties)).toBe(true);
    expect(Array.isArray(active.relations)).toBe(true);
    expect(active.relations.length > 0 || active.properties.length > 0).toBe(true);

    // Validate zod parsing
    for (const p of active.properties) {
      const parsed = schemas.ActivePropertySchema.parse(p);
      expect(parsed.id).toBeTruthy();
    }
    for (const r of active.relations) {
      const parsed = schemas.ActiveRelationSchema.parse(r as any);
      expect(parsed.id).toBeTruthy();
    }
  });
});
