import { describe, it, expect } from 'vitest';
import { DialecticalBase, ActiveCarrier, NoumenalEvent } from '../../src/absolute/core/dialectic';

type Core = { id: string; label: string };
type Payload = { label: string };

class DummyDialecticalDriver extends DialecticalBase<Payload, Core> {
  constructor() { super('DummyDialecticalDriver'); }

  toActive(core: Core): ActiveCarrier<Payload> {
    return { id: core.id, kind: 'thing', payload: { label: core.label } };
  }

  fromActive(active: ActiveCarrier<Payload>): Core {
    return { id: active.id, label: active.payload.label };
  }

  // leave noumenal methods unimplemented to use DialecticalBase defaults
}

describe('DialecticalBase contract (basic)', () => {
  it('round-trips core <-> active and supports batch', async () => {
    const drv = new DummyDialecticalDriver();

    const core: Core = { id: 'c1', label: 'Alpha' };
    const active = drv.toActive(core);
    expect(active.id).toBe(core.id);
    expect(active.payload.label).toBe(core.label);

    const back = drv.fromActive(active);
    expect(back).toEqual(core);

    const batch = drv.toActiveBatch([{ id: 'c2', label: 'Beta' }, core]);
    expect(batch.length).toBe(2);
    expect(batch[0].payload.label).toBe('Beta');
  });

  it('provides sane noumenal defaults', async () => {
    const drv = new DummyDialecticalDriver();
    // interpretNoumenal default returns empty array
    const res = await drv.interpretNoumenal?.({ kind: 'essence.test' } as NoumenalEvent);
    expect(res).toEqual([]);

    // expressNoumenalFromActive default returns null
    const out = drv.expressNoumenalFromActive?.({ id: 'c1', kind: 'thing', payload: { label: 'X' } });
    expect(out).toBeNull();
  });
});
