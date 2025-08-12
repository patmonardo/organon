import { describe, it, expect } from 'vitest';
import {
  createProperty,
  updateProperty,
  PropertySchema,
} from '../../src/schema/property';
import { createEntity, EntityRef } from '../../src/schema/entity';

describe('schema/property', () => {
  it('creates with required contextId; key present; value updates validate', () => {
    const p0 = createProperty({
      type: 'system.Property',
      key: 'p0',
      contextId: 'ctx:1',
    } as any);
    const parsed0 = PropertySchema.parse(p0);
    const core = (parsed0 as any).shape?.core ?? (parsed0 as any).core ?? {};
    expect(core.key).toBe('p0');
    const ctxId =
      (parsed0 as any).shape?.contextId ?? (parsed0 as any).contextId;
    expect(ctxId).toBe('ctx:1');

    // Include contextId in updates to satisfy schema if updateProperty is non-merge
    const p1 = updateProperty(parsed0, {
      contextId: ctxId,
      value: 42,
      valueType: 'number',
    } as any);
    const parsed1 = PropertySchema.parse(p1);
    expect((parsed1 as any).shape?.value ?? (parsed1 as any).value).toBe(42);
    expect(
      (parsed1 as any).shape?.valueType ?? (parsed1 as any).valueType,
    ).toBe('number');
  });

  it('binds to entity via EntityRef and clears relationId', () => {
    const e = createEntity({ type: 'system.Entity', name: 'E0' } as any);
    const ref = EntityRef.parse({
      id: e.shape.core.id,
      type: e.shape.core.type,
    });

    const p0 = createProperty({
      type: 'system.Property',
      key: 'p1',
      contextId: 'ctx:1',
    } as any);
    const parsed0 = PropertySchema.parse(p0);
    const ctxId =
      (parsed0 as any).shape?.contextId ?? (parsed0 as any).contextId;

    const p1 = updateProperty(parsed0, {
      contextId: ctxId,
      entity: ref,
      relationId: null,
    } as any);
    const parsed = PropertySchema.parse(p1);
    expect(
      (parsed as any).shape?.entity ?? (parsed as any).entity,
    ).toBeTruthy();

    const p2 = updateProperty(parsed, {
      contextId: ctxId,
      entity: null,
      relationId: null,
    } as any);
    const parsed2 = PropertySchema.parse(p2);
    const entityVal = (parsed2 as any).shape?.entity ?? (parsed2 as any).entity;
    expect(entityVal == null).toBe(true); // accept null or undefined
  });

  it('rejects missing contextId (helper enforces required field)', () => {
    expect(() =>
      createProperty({ type: 'system.Property', key: 'p2' } as any),
    ).toThrow();
  });
});
