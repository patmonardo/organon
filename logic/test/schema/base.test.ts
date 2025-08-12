import { describe, it, expect } from 'vitest';
import {
  Id,
  Label,
  Type,
  IsoDateTime,
  Timestamps,
  stamp,
  touch,
  BaseCore,
  BaseState,
  BaseShape,
  BaseSchema,
} from '../../src/schema/base';

describe('schema/base primitives', () => {
  it('Id/Label/Type validate non-empty strings', () => {
    expect(Id.parse('x')).toBe('x');
    expect(Label.parse('name')).toBe('name');
    expect(Type.parse('system.Type')).toBe('system.Type');
    expect(() => Id.parse('')).toThrow();
    expect(() => Label.parse('')).toThrow();
    expect(() => Type.parse('')).toThrow();
  });

  it('IsoDateTime validates ISO strings', () => {
    const iso = new Date().toISOString();
    expect(IsoDateTime.parse(iso)).toBe(iso);
    expect(() => IsoDateTime.parse('not-a-date')).toThrow();
  });
});

describe('schema/base timestamps helpers', () => {
  it('Timestamps supplies defaults', () => {
    const t = Timestamps.parse({});
    expect(typeof t.createdAt).toBe('string');
    expect(typeof t.updatedAt).toBe('string');
    // round-trip via schema
    expect(() => IsoDateTime.parse(t.createdAt)).not.toThrow();
    expect(() => IsoDateTime.parse(t.updatedAt)).not.toThrow();
  });

  it('stamp sets createdAt/updatedAt when missing and preserves provided values', () => {
    const a = stamp({});
    expect(a.createdAt).toBeDefined();
    expect(a.updatedAt).toBeDefined();

    const given = new Date(0).toISOString();
    const b = stamp({ createdAt: given, updatedAt: given });
    expect(b.createdAt).toBe(given);
    expect(b.updatedAt).toBe(given);
  });

  it('touch updates only updatedAt', async () => {
    const a = stamp({});
    await new Promise((r) => setTimeout(r, 5));
    const b = touch(a);
    expect(b.createdAt).toBe(a.createdAt);
    expect(b.updatedAt).not.toBe(a.updatedAt);
    expect(() => IsoDateTime.parse(b.updatedAt!)).not.toThrow();
  });
});

describe('schema/base models', () => {
  it('BaseCore defaults timestamps', () => {
    const core = BaseCore.parse({ id: 'id:1', type: 'system.Type' });
    expect(core.id).toBe('id:1');
    expect(core.type).toBe('system.Type');
    expect(() => IsoDateTime.parse(core.createdAt)).not.toThrow();
    expect(() => IsoDateTime.parse(core.updatedAt)).not.toThrow();
  });

  it('BaseState defaults status/tags/meta', () => {
    const state = BaseState.parse({});
    expect(state.status).toBe('active');
    expect(Array.isArray(state.tags)).toBe(true);
    expect(state.tags.length).toBe(0);
    expect(state.meta && typeof state.meta).toBe('object');
  });

  it('BaseShape composes Core + State', () => {
    const shape = BaseShape.parse({
      core: { id: 'id:s', type: 'system.Type' },
      state: {},
    });
    expect(shape.core.id).toBe('id:s');
    expect(shape.state.status).toBe('active');
  });

  it('BaseSchema wraps shape and sets defaults (revision/ext)', () => {
    const doc = BaseSchema.parse({
      shape: {
        core: { id: 'id:b', type: 'system.Type' },
        state: {},
      },
    });
    expect(doc.shape.core.id).toBe('id:b');
    expect(doc.revision).toBe(0);
    expect(doc.ext && typeof doc.ext).toBe('object');
  });

  it('BaseSchema accepts provided revision and version', () => {
    const doc = BaseSchema.parse({
      shape: {
        core: { id: 'id:c', type: 'system.Type' },
        state: {},
      },
      revision: 3,
      version: '1.2.3',
    });
    expect(doc.revision).toBe(3);
    expect(doc.version).toBe('1.2.3');
  });
});
