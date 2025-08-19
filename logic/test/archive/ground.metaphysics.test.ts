import { describe, it, expect } from 'vitest';
import { groundStage } from '../../src/absolute/essence/ground';

describe('Ground — metaphysics provenance', () => {
  it('produces Absolute and Essential relations with metaphysics provenance and links', async () => {
    const morph = {
      id: 'm:test',
      // minimal ruleSpec for deriveRelation
      ruleSpec: {
        id: 'm:test',
        kind: 'deriveRelation',
        relationType: 'related_to',
        source: { byId: 'E1' },
        target: { kind: 'fixed', targetEntityId: 'E2' },
        idempotent: true,
      },
    } as any;

    const principles = { morphs: [morph] };
    const graph = {
      entities: [{ id: 'E1' }, { id: 'E2' }],
      properties: [
        {
          id: 'p:E1:A',
          entityId: 'E1',
          key: 'A',
          value: true,
          contextId: 'ctx1',
          contextVersion: 'v1',
          status: 'valid',
        },
      ],
    };

    const res = await groundStage(principles as any, graph as any, { fixpointMaxIters: 4 });

    const essential = res.relations.find(
      (r) => (r as any).kind === 'essential' && (r as any).sourceId === 'E1' && (r as any).targetId === 'E2',
    );
    const absolute = res.relations.find(
      (r) => (r as any).kind === 'absolute' && (r as any).sourceId === 'E1' && (r as any).targetId === 'E2',
    );

    expect(essential).toBeDefined();
    expect(absolute).toBeDefined();

    expect((essential as any).particularityOf).toBe((absolute as any).id);

    // essential provenance
    expect((essential as any).provenance).toBeDefined();
    expect((essential as any).provenance.metaphysics.role).toBe('particular');
    expect((essential as any).provenance.metaphysics.faculty).toBe('ahamkara');
    expect((essential as any).provenance.metaphysics.intuition).toBe('inner');

    // absolute provenance
    expect((absolute as any).provenance).toBeDefined();
    expect((absolute as any).provenance.metaphysics.role).toBe('absolute');
    expect((absolute as any).provenance.metaphysics.faculty).toBe('buddhi');
    expect((absolute as any).provenance.metaphysics.intuition).toBe('intellectual');
  });
});
