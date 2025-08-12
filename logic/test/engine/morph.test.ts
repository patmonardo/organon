import { describe, it, expect } from 'vitest';
import { MorphEngine } from '../../src/form/morph/engine';
import { InMemoryEventBus } from '../../src/form/triad/bus';
import { makeInMemoryRepository } from '../support/inMemoryRepo';
import type { Morph as MorphDoc } from '../../src/schema/morph';

describe('MorphEngine', () => {
  it('creates/updates/gets/deletes schema morphs and defines/composes/executes runtime morphs', async () => {
    const repo = makeInMemoryRepository<MorphDoc>();
    const bus = new InMemoryEventBus();

    const received: string[] = [];
    const tap = (k: string) => bus.subscribe(k, (e) => received.push(e.kind));
    [
      'morph.created',
      'morph.updated',
      'morph.got',
      'morph.deleted',
      'morph.runtime.defined',
      'morph.runtime.composed',
      'morph.execution.started',
      'morph.execution.completed',
    ].forEach(tap);

    const engine = new MorphEngine(repo as any, bus);

    // create schema morph
    const [created] = await engine.handle({
      kind: 'morph.create',
      payload: {
        type: 'system.Morph',
        name: 'Upper',
        inputType: 'string',
        outputType: 'string',
        transformFn: 'toUpperCase',
      },
    } as any);
    expect(created.kind).toBe('morph.created');
    const id = (created.payload as any).id as string;
    expect(id).toBeTruthy();

    // update schema morph (rename)
    const [updated] = await engine.handle({
      kind: 'morph.update',
      payload: { id, patch: { core: { name: 'Upper2' } } },
    } as any);
    expect(updated.kind).toBe('morph.updated');

    // get schema morph
    const [got] = await engine.handle({
      kind: 'morph.get',
      payload: { id },
    } as any);
    expect(got.kind).toBe('morph.got');
    expect((got.payload as any).morph?.shape?.core?.id).toBe(id);

    // runtime: define two simple morphs
    const [definedA] = await engine.handle({
      kind: 'morph.defineRuntime',
      payload: {
        name: 'double',
        transformer: (n: number) => n * 2,
        options: { pure: true, cost: 1 },
      },
    } as any);
    expect(definedA.kind).toBe('morph.runtime.defined');

    const [definedB] = await engine.handle({
      kind: 'morph.defineRuntime',
      payload: {
        name: 'inc',
        transformer: (n: number) => n + 1,
        options: { pure: true, cost: 1 },
      },
    } as any);
    expect(definedB.kind).toBe('morph.runtime.defined');

    // compose runtime morph: double -> inc
    const [composed] = await engine.handle({
      kind: 'morph.composeRuntime',
      payload: {
        name: 'doubleThenInc',
        steps: ['double', 'inc'],
        composedName: 'doubleThenInc',
      },
    } as any);
    expect(composed.kind).toBe('morph.runtime.composed');

    // execute composed morph
    const [started, completed] = await engine.handle({
      kind: 'morph.execute',
      payload: { name: 'doubleThenInc', input: 2 },
    } as any);
    expect(started.kind).toBe('morph.execution.started');
    expect(completed.kind).toBe('morph.execution.completed');
    expect((completed.payload as any).result).toBe(5);

    // delete schema morph
    const [deleted] = await engine.handle({
      kind: 'morph.delete',
      payload: { id },
    } as any);
    expect(deleted.kind).toBe('morph.deleted');

    expect(received).toEqual([
      'morph.created',
      'morph.updated',
      'morph.got',
      'morph.runtime.defined',
      'morph.runtime.defined',
      'morph.runtime.composed',
      'morph.execution.started',
      'morph.execution.completed',
      'morph.deleted',
    ]);
  });
});
