import { describe, it, expect, vi } from 'vitest';
import { DefaultReflectDriver, processContexts, commitContexts } from '../../src/absolute/essence/reflect';
import { ContextEngine } from '../../src/form/context/engine';
import { makeInMemoryRepository } from '../../src/repository/memory';
import { ContextSchema } from '../../src/schema/context';
import { InMemoryEventBus } from '../../src/absolute/core/triad/bus';

const repo = makeInMemoryRepository(ContextSchema as any);

describe('ReflectDriver → ContextEngine (ActiveContext)', () => {
  it('processes and commits ActiveContexts (create + update)', async () => {
    const contexts = [
      { id: 'context:one', kind: 'domain', name: 'One', active: true },
      { id: 'context:two', kind: 'domain', name: 'Two', active: true },
    ];

    const { actions, snapshot } = await DefaultReflectDriver.processContexts(contexts, { repo });
    expect(snapshot.count).toBe(2);
    expect(actions.some(a => a.type === 'context.upsert' && a.id === 'context:one')).toBe(true);

    const result = await DefaultReflectDriver.commitContexts(contexts, { repo });
    expect(result.commitResult.success).toBe(true);

    // engine sanity: describe via engine
    const engine = new ContextEngine(repo as any);
    const [evt] = await engine.handle({ kind: 'context.describe', payload: { id: 'context:one' } } as any);
    expect(evt.kind).toBe('context.described');
    expect((evt.payload as any).id).toBe('context:one');
  });

  it('emits events on commit and supports subscription', async () => {
    const bus = new InMemoryEventBus();
    const onCreated = vi.fn();
    bus.subscribe('context.created', onCreated);

    const input = [{ id: 'context:alpha', kind: 'domain', name: 'Alpha', active: true }];
    const { commitResult } = await commitContexts(input, { repo, bus });

    expect(commitResult.success).toBe(true);
    expect(onCreated).toHaveBeenCalledTimes(1);
    expect(onCreated.mock.calls[0][0].payload.id).toBe('context:alpha');
  });

  it('handles revoked (deletes)', async () => {
    await commitContexts([{ id: 'context:x', kind: 'domain', name: 'X', active: true }], { repo });
    const { actions } = await processContexts([{ id: 'context:x', revoked: true }], { repo });
    expect(actions.some(a => a.type === 'context.delete' && a.id === 'context:x')).toBe(true);
    const { commitResult } = await commitContexts([{ id: 'context:x', revoked: true }], { repo });
    expect(commitResult.success).toBe(true);
  });
});
