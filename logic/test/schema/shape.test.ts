import { describe, it, expect } from 'vitest';
import { FormShape } from '../../src/form/shape/shape';

describe('schema/shape (via FormShape wrapper)', () => {
  it('creates with defaults and has an id', () => {
    const s = FormShape.create({
      definitionId: 'def:shape:1',
      definitionName: 'Demo',
    });
    expect(s.id).toBeTruthy();
    expect(s.definitionId).toBe('def:shape:1');
    expect(s.definitionName).toBe('Demo');
  });

  it('id is stable across data/state mutations', () => {
    const s = FormShape.create({
      definitionId: 'def:shape:stable',
      definitionName: 'Stable',
    });
    const id0 = s.id;

    s.setData({ a: 1 } as any);
    s.patchState({ status: 'active' } as any);
    s.mergeData({ b: { x: 1 } } as any);
    s.patchState({ status: 'success' } as any);

    expect(s.id).toBe(id0);
  });

  it('mergeData is shallow: replaces provided branches only', () => {
    const s = FormShape.create({
      definitionId: 'def:shape:merge',
      definitionName: 'Merge',
    });
    s.setData({
      meta: { version: '1', theme: 'light' },
      access: { read: { path: 'name' } },
    } as any);

    s.mergeData({ meta: { version: '2' } } as any);

    expect((s.data as any).meta.version).toBe('2');
    expect((s.data as any).meta.theme).toBeUndefined();
    expect((s.data as any).access.read.path).toBe('name');
  });

  it('patchState merges partial state without dropping unspecified fields', () => {
    const s = FormShape.create({
      definitionId: 'def:shape:state',
      definitionName: 'State',
      state: { status: 'idle', message: 'hi' } as any,
    });
    s.patchState({ status: 'error' } as any);
    expect(s.state.status).toBe('error');
    expect((s.state as any).message).toBe('hi');
  });
});
