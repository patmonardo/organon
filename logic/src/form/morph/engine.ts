import { InMemoryEventBus, type EventBus } from '../triad/bus';
import type { Morph as MorphDoc } from '../../schema/morph';
import { MorphSchema, createMorph, updateMorph } from '../../schema/morph';

type RuntimeFn = (input: any) => any;

export class MorphEngine {
  private readonly bus: EventBus;
  private readonly mem = new Map<string, MorphDoc>();
  private readonly runtime = new Map<string, RuntimeFn>();

  constructor(private readonly repo?: any, bus?: EventBus) {
    this.bus = bus ?? new InMemoryEventBus();
  }

  // Core handle API (returns emitted events)
  async handle(msg: any) {
    switch (msg.kind) {
      case 'morph.create': {
        const doc = MorphSchema.parse(createMorph(msg.payload as any));
        await this.persist(doc);
        const id = doc.shape.core.id;
        const evt = { kind: 'morph.created', payload: { id } };
        this.bus.publish(evt);
        return [evt];
      }

      case 'morph.update': {
        const { id, patch } = msg.payload as { id: string; patch: any };
        const curr = await this.mustGet(id);
        const next = MorphSchema.parse(updateMorph(curr, patch));
        await this.persist(next);
        const evt = { kind: 'morph.updated', payload: { id } };
        this.bus.publish(evt);
        return [evt];
      }

      case 'morph.get': {
        const { id } = msg.payload as { id: string };
        const morph = await this.get(id);
        const evt = { kind: 'morph.got', payload: { id, morph } };
        this.bus.publish(evt);
        return [evt];
      }

      case 'morph.delete': {
        const { id } = msg.payload as { id: string };
        const ok = await this.remove(id);
        const evt = { kind: 'morph.deleted', payload: { id, ok } };
        this.bus.publish(evt);
        return [evt];
      }

      // Runtime morphs
      case 'morph.defineRuntime': {
        const { name, transformer } = msg.payload as {
          name: string;
          transformer: RuntimeFn;
          options?: Record<string, unknown>;
        };
        if (typeof transformer !== 'function') {
          throw new Error('transformer must be a function');
        }
        this.runtime.set(name, transformer);
        const evt = { kind: 'morph.runtime.defined', payload: { name } };
        this.bus.publish(evt);
        return [evt];
      }

      case 'morph.composeRuntime': {
        const { name, steps, composedName } = msg.payload as {
          name: string;
          steps: string[];
          composedName?: string;
        };
        const fns = steps.map((s) => {
          const f = this.runtime.get(s);
          if (!f) throw new Error(`runtime morph not found: ${s}`);
          return f;
        });
        const composed = (input: any) =>
          fns.reduce((acc, fn) => fn(acc), input);
        const outName = composedName ?? name;
        this.runtime.set(outName, composed);
        const evt = {
          kind: 'morph.runtime.composed',
          payload: { name: outName, steps },
        };
        this.bus.publish(evt);
        return [evt];
      }

      case 'morph.execute': {
        const { name, input } = msg.payload as { name: string; input: any };
        const fn = this.runtime.get(name);
        if (!fn) throw new Error(`runtime morph not found: ${name}`);
        const started = { kind: 'morph.execution.started', payload: { name } };
        this.bus.publish(started);
        const result = await Promise.resolve(fn(input));
        const completed = {
          kind: 'morph.execution.completed',
          payload: { name, result },
        };
        this.bus.publish(completed);
        return [started, completed];
      }

      default:
        throw new Error(`Unknown command: ${msg.kind}`);
    }
  }

  // Internals

  private async get(id: string): Promise<MorphDoc | undefined> {
    if (this.repo) {
      const doc = await this.repo.get(id);
      return doc ? MorphSchema.parse(doc) : undefined;
    }
    const doc = this.mem.get(id);
    return doc ? MorphSchema.parse(doc) : undefined;
  }

  private async mustGet(id: string): Promise<MorphDoc> {
    const doc = await this.get(id);
    if (!doc) throw new Error(`Morph not found: ${id}`);
    return doc;
  }

  private async persist(doc: MorphDoc): Promise<void> {
    const id = doc.shape.core.id;
    if (this.repo) {
      const existing = await this.repo.get(id);
      if (existing) {
        await (this.repo as any).update(id, () => doc); // use mutateFn form
      } else {
        await this.repo.create(doc);
      }
    } else {
      this.mem.set(id, doc);
    }
  }

  private async remove(id: string): Promise<boolean> {
    if (this.repo) {
      const existing = await this.repo.get(id);
      if (!existing) return false;
      await this.repo.delete(id);
      return true;
    }
    return this.mem.delete(id);
  }
}
