import { describe, it, expect, vi } from 'vitest';
import { runKriya } from '../../src/absolute/core/orchestrator';
import type { Morph } from '../../src/schema/morph';

describe('Knowledge delta', () => {
  it('computes a positive delta when new relations/properties and actualization occur', async () => {
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

    expect(res.knowledge?.score ?? 0).toBeGreaterThan(0);
    expect(bus.publish).toHaveBeenCalled();
    expect((bus.publish as any).mock.calls.some(([evt]: any[]) => evt?.kind === 'knowledge.delta')).toBe(true);
  });
});
