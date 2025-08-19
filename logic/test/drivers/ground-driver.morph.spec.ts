import { describe, it, expect } from 'vitest';
import { GroundDriver } from '../../src/absolute/essence/ground';
import { MorphEngine } from '../../src/form/morph/engine';

describe('GroundDriver — ActiveMorph orchestration', () => {
  it('upserts ActiveMorphs and returns engine events', async () => {
  const engine = new MorphEngine();
    const driver = new GroundDriver();

    const morphs = driver.toActiveMorphBatch([
      { id: 'm:inc', kind: 'inc', transform: 'math.inc', params: { by: 1 } },
    ]);

    const events1 = await driver.upsertMorphs(morphs, { engine });
    expect(events1.some((e: any) => e.kind === 'morph.created')).toBe(true);

    const updated = driver.toActiveMorph({ id: 'm:inc', kind: 'inc', transform: 'math.inc', params: { by: 2 } });
    const events2 = await driver.upsertMorphs([updated], { engine });
    expect(events2.some((e: any) => e.kind === 'morph.updated')).toBe(true);
  });

  it('defines, composes, and executes runtime morphs', async () => {
  const engine = new MorphEngine();
    const driver = new GroundDriver();

    // define two simple steps
    await driver.defineRuntime('add2', (x) => x + 2, { engine });
    await driver.defineRuntime('mul3', (x) => x * 3, { engine });

    // compose and execute
    const composeEvents = await driver.composeRuntime('pipeline', ['add2', 'mul3'], 'add2_then_mul3', { engine });
    expect(composeEvents[0].kind).toBe('morph.runtime.composed');

    const runEvents = await driver.executeRuntime('add2_then_mul3', 5, { engine });
    expect(runEvents[0].kind).toBe('morph.execution.started');
    expect(runEvents[1].kind).toBe('morph.execution.completed');
    expect(runEvents[1].payload.result).toBe(21);
  });
});
