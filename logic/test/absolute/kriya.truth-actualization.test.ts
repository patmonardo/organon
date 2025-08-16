import { describe, it, expect, vi } from 'vitest';
import { runKriya } from '../../src/absolute/core';
import type { Morph } from '../../src/schema/morph';

describe('Kriya — truth actualization', () => {
  it('actualizes essential relations and emits knowledge events', async () => {
    const morph: Morph = {
      id: 'm:copula-A-to-R',
      // @ts-ignore
      ruleSpec: {
        id: 'm:copula-A-to-R',
        kind: 'deriveRelation',
        condition: { op: 'exists', key: 'A' },
        source: { byId: 'E1' },
        target: { kind: 'fixed', targetEntityId: 'E2' },
        relationType: 'related_to',
        setProperty: { key: 'B', value: 1 },
        idempotent: true,
      },
    } as any;

    const bus = { publish: vi.fn() };
    const triad = {
      relation: { get: vi.fn().mockResolvedValue(undefined), create: vi.fn(), update: vi.fn() },
      property: { get: vi.fn().mockResolvedValue(undefined), create: vi.fn(), update: vi.fn() },
      bus,
    };

    const res = await runKriya(
      {
        shapes: [],
        entities: [{ id: 'E1' } as any, { id: 'E2' } as any],
        properties: [{ id: 'p:E1:A', entityId: 'E1', key: 'A', value: true } as any],
        contexts: [], morphs: [morph], relations: [], content: [], concepts: [], judgments: [], syllogisms: [],
      },
      { commitGround: true, triad }
    );

    const r = res.ground?.relations.find((x) => (x as any).kind === 'essential');
    expect(r?.provenance?.modality?.kind).toBe('actual');
    expect(bus.publish).toHaveBeenCalled();
    expect((bus.publish as any).mock.calls.some(([evt]: any[]) => evt?.kind === 'knowledge.relation.actualized')).toBe(true);
  });
});
