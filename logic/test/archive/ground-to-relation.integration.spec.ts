import { describe, it, expect } from 'vitest';
import { groundStage, toActiveFromGround } from '../../src/absolute/essence/ground';
import { RelationDriver } from '../../src/absolute/essence/relation';

describe('Ground → RelationEngine — truth of Ground', () => {
  it('commits derived relations from ground into the RelationEngine', async () => {
  const relationDriver = new RelationDriver();

    // Working graph with a trigger property that will cause a relation derivation
    const working = {
      entities: [
        { id: 'A', type: 'system.Thing' },
        { id: 'B', type: 'system.Thing' },
      ],
      properties: [
        { id: 'p:A:flag', entityId: 'A', key: 'flag', value: true },
      ],
      relations: [],
    };

    // A simple morph rule: when A has flag===true, derive relation A -> B of type supports
    const morphs = [
      {
        id: 'm:derive-supports',
        ruleSpec: {
          id: 'rule:supports',
          kind: 'deriveRelation',
          relationType: 'supports',
          idempotent: true,
          condition: { op: 'eq', key: 'flag', value: true },
          source: { byId: 'A' },
          target: { kind: 'fixed', targetEntityId: 'B' },
          setProperty: { key: 'strength', value: 1, status: 'derived' },
        },
      },
    ];

    const result = await groundStage({ morphs } as any, working as any);
    expect(result.relations.length).toBeGreaterThan(0);

    const active = toActiveFromGround(result);
    // Commit relations via the RelationDriver (properties handled by WorldDriver elsewhere)
    const { commitResult } = await relationDriver.commitActiveRelations(
      active.relations as any,
    );
    expect(commitResult.events.some((e: any) => e.kind === 'relation.created')).toBe(true);
  });
});
