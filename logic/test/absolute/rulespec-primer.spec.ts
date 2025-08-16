import { describe, it, expect } from 'vitest';
import { runCycle, StageFns } from '../../../logic/src/absolute/core/kriya';
import { reflectStage } from '../../../logic/src/absolute/essence/reflect';
import { groundStage, applyMorphRule } from '../../../logic/src/absolute/essence/ground';

describe.skip('RuleSpec primer examples', () => {
  it('deriveRelation RuleSpec produces essential + absolute relations', async () => {
    const principles = { shapes: [], contexts: [], morphs: [
      {
        id: 'm-derive-rel',
        ruleSpec: {
          id: 'rs-rel-1',
          kind: 'deriveRelation',
          relationType: 'friend_of',
          source: { byId: 'thing:1' },
          target: { kind: 'fixed', targetEntityId: 'thing:2' },
          idempotent: true,
          setProperty: { key: 'derived', value: true }
        }
      }
    ] } as any;

  const entities = [{ id: 'thing:1', type: 'Human', essence: { name: 'Alice' } }, { id: 'thing:2', type: 'Human', essence: { name: 'Bob' } }];
  const properties = [{ id: 'prop:1', entityId: 'thing:1', key: 'likes', value: 'chess' }];
  // call applyMorphRule directly for the example morph
  const res = await applyMorphRule(principles.morphs[0] as any, { entities: entities as any, properties: properties as any, relations: [] as any });
  const rels = res.derivedRelations;
  expect(rels.length).toBeGreaterThanOrEqual(1);
  // expect at least one derived relation with kind 'essential' or 'absolute'
  const essential = rels.find((r) => (r as any).kind === 'essential');
  const absolute = rels.find((r) => (r as any).kind === 'absolute');
  expect(essential || absolute).toBeDefined();
  });

  it('deriveProperty RuleSpec produces a derived property for targets', async () => {
    const principles = { shapes: [], contexts: [], morphs: [
      {
        id: 'm-derive-prop',
        ruleSpec: {
          id: 'rs-prop-1',
          kind: 'deriveProperty',
          setProperty: { key: 'score', value: 7, status: 'derived' },
          source: { all: true }
        }
      }
    ] } as any;

  const entities = [{ id: 'thing:A', type: 'Node' }, { id: 'thing:B', type: 'Node' }];
  const properties: any[] = [];
  const res2 = await applyMorphRule(principles.morphs[0] as any, { entities: entities as any, properties: properties as any, relations: [] as any });
  const props = res2.derivedProperties;
  // Expect at least one derived property with key 'score'
  const derived = props.filter((p) => (p as any).key === 'score');
  expect(derived.length).toBeGreaterThanOrEqual(1);
  expect((derived[0] as any).value).toEqual(7);
  });
});
