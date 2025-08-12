import { describe, it, expect } from 'vitest';
import { randomUUID } from 'node:crypto';
import {
  createForm,
  updateForm,
  getFormShape,
  setFormShape,
  FormSchema,
} from '../../src/schema/form';

describe('schema/form', () => {
  const makeFormShape = (over: Partial<any> = {}) => ({
    core: { id: randomUUID(), type: 'form.Shape' },
    definition: { id: 'def:1', name: 'Demo' },
    state: {},
    data: { a: 1 },
    ...over,
  });

  it('createForm -> valid schema: core/state defaults; embeds FormShape', () => {
    const formShape = makeFormShape();
    const doc = createForm({
      type: 'system.Form',
      name: 'F0',
      form: formShape,
    } as any);

    const parsed = FormSchema.parse(doc);
    expect(parsed.shape.core.id).toBeTruthy();
    expect(parsed.shape.core.type).toBe('system.Form');
    expect(parsed.shape.state.status).toBe('active');
    expect(getFormShape(parsed)).toBeDefined();
  });

  it('updateForm preserves id and increments revision; set/get form shape round-trips', () => {
    const formShape = makeFormShape();
    const base = FormSchema.parse(
      createForm({ type: 'system.Form', name: 'F1', form: formShape } as any),
    );

    const id0 = base.shape.core.id;
    const rev0 = base.revision;

    // mutate embedded FormShape data
    const nextShape = { ...getFormShape(base), data: { a: 2 } } as any;
    const doc1 = updateForm(base, { form: nextShape });
    const p1 = FormSchema.parse(doc1);

    expect(p1.shape.core.id).toBe(id0);
    expect(p1.revision).toBeGreaterThan(rev0);
    expect((getFormShape(p1) as any).data).toEqual({ a: 2 });

    // use ergonomics helpers
    const p2 = setFormShape(p1, { ...nextShape, data: { a: 3 } } as any);
    const parsed2 = FormSchema.parse(p2);
    expect((getFormShape(parsed2) as any).data).toEqual({ a: 3 });
  });

  it('accepts state patch via updateForm and keeps other fields', () => {
    const doc0 = createForm({
      type: 'system.Form',
      name: 'F2',
      form: makeFormShape(),
      state: { status: 'idle', tags: ['x'] },
    } as any);
    const p0 = FormSchema.parse(doc0);

    const p1 = FormSchema.parse(
      updateForm(p0, { state: { status: 'active' } }),
    );
    expect(p1.shape.state.status).toBe('active');
    expect(Array.isArray(p1.shape.state.tags)).toBe(true);
  });
});
