import type { Command, Event, EventBus } from '../triad/message';
import { InMemoryEventBus } from '../triad/bus';
import { startTrace, childSpan } from '../triad/trace';

import type { Repository, Concurrency } from '../../repository/repo';
import {
  type Morph as MorphDoc,
  MorphSchema,
  createMorph,
  updateMorph,
} from '../../schema/morph';

import { FormMorph } from './morph';
import type { MorphTransformer, MorphOptions } from './core';

// Core commands (unified verbs)
export type MorphCreateCmd = {
  kind: 'morph.create';
  payload: Parameters<typeof createMorph>[0];
  meta?: Record<string, unknown>;
};
export type MorphDeleteCmd = {
  kind: 'morph.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type MorphDescribeCmd = {
  kind: 'morph.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type MorphSetCoreCmd = {
  kind: 'morph.setCore';
  payload: {
    id: string;
    name?: string;
    type?: string;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type MorphSetStateCmd = {
  kind: 'morph.setState';
  payload: {
    id: string;
    state: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type MorphPatchStateCmd = {
  kind: 'morph.patchState';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type MorphSetFacetsCmd = {
  kind: 'morph.setFacets';
  payload: {
    id: string;
    facets: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type MorphMergeFacetsCmd = {
  kind: 'morph.mergeFacets';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type MorphSetSignatureCmd = {
  kind: 'morph.setSignature';
  payload: {
    id: string;
    signature?: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type MorphMergeSignatureCmd = {
  kind: 'morph.mergeSignature';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};

// Runtime registry commands (engine-local)
export type MorphDefineRuntimeCmd = {
  kind: 'morph.defineRuntime';
  payload: {
    name: string;
    transformer: MorphTransformer<any, any>;
    options?: Partial<MorphOptions>;
  };
  meta?: Record<string, unknown>;
};
export type MorphComposeRuntimeCmd = {
  kind: 'morph.composeRuntime';
  payload: { name: string; steps: string[]; composedName?: string };
  meta?: Record<string, unknown>;
};
export type MorphExecuteCmd = {
  kind: 'morph.execute';
  payload: { name: string; input: unknown; context?: any };
  meta?: Record<string, unknown>;
};

export type MorphCommand =
  | MorphCreateCmd
  | MorphDeleteCmd
  | MorphDescribeCmd
  | MorphSetCoreCmd
  | MorphSetStateCmd
  | MorphPatchStateCmd
  | MorphSetFacetsCmd
  | MorphMergeFacetsCmd
  | MorphSetSignatureCmd
  | MorphMergeSignatureCmd
  | MorphDefineRuntimeCmd
  | MorphComposeRuntimeCmd
  | MorphExecuteCmd;

/**
 * MorphEngine — schema-backed docs + local runtime registry.
 * Command-in, Event-out. Unified verbs, Triad bus, scoped tracing.
 */
export class MorphEngine {
  private readonly runtime = new Map<string, FormMorph<any, any>>();

  constructor(
    private readonly repo: Repository<MorphDoc>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'morph',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  // Standardized emit with trace + scope
  private emit(
    base: Record<string, unknown>,
    kind: Event['kind'],
    payload: Event['payload'],
    extraMeta?: Record<string, unknown>,
  ): Event {
    const meta = childSpan(base as any, {
      action: kind,
      scope: this.scope,
      ...(extraMeta ?? {}),
    });
    const evt: Event = { kind, payload, meta };
    this.bus.publish(evt);
    return evt;
  }

  // Runtime registry helpers
  private defineRuntime(
    name: string,
    transformer: MorphTransformer<any, any>,
    options?: Partial<MorphOptions>,
  ) {
    const fm = FormMorph.define(name, transformer, options ?? {});
    this.runtime.set(name, fm);
    return fm;
  }

  private composeRuntime(
    steps: string[],
    composedName?: string,
  ): FormMorph<any, any> {
    if (steps.length === 0) throw new Error('composeRuntime: steps required');
    const seq = steps.map((n) => {
      const m = this.runtime.get(n);
      if (!m) throw new Error(`runtime morph not found: ${n}`);
      return m;
    });
    let acc = seq[0];
    for (let i = 1; i < seq.length; i++)
      acc = FormMorph.compose(acc, seq[i], composedName);
    if (composedName) this.runtime.set(composedName, acc);
    return acc;
  }

  async handle(cmd: MorphCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'MorphEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      // Schema-backed
      case 'morph.create': {
        const doc = createMorph((cmd as MorphCreateCmd).payload as any);
        const created = await this.repo.create(MorphSchema.parse(doc));
        return [
          this.emit(base, 'morph.created', {
            id: created.shape.core.id,
            type: created.shape.core.type,
            name: created.shape.core.name ?? null,
          }),
        ];
      }

      case 'morph.delete': {
        const { id } = (cmd as MorphDeleteCmd).payload;
        const ok = await this.repo.delete(id);
        return [this.emit(base, 'morph.deleted', { id, ok: !!ok })];
      }

      case 'morph.describe': {
        const { id } = (cmd as MorphDescribeCmd).payload;
        const doc = await this.mustGet(id);
        return [
          this.emit(base, 'morph.described', {
            id,
            type: doc.shape.core.type,
            name: doc.shape.core.name ?? null,
            state: doc.shape.state,
            signatureKeys: Object.keys((doc.shape as any).signature ?? {}),
            facetsKeys: Object.keys((doc.shape as any).facets ?? {}),
          }),
        ];
      }

      case 'morph.setCore': {
        const { id, name, type, expectedRevision } = (cmd as MorphSetCoreCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateMorph(curr, {
          core: {
            ...(name !== undefined ? { name } : {}),
            ...(type !== undefined ? { type } : {}),
          },
        } as any);
        const saved = await this.repo.update(
          id,
          () => MorphSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [
          this.emit(base, 'morph.core.set', {
            id,
            name: saved.shape.core.name ?? null,
            type: saved.shape.core.type,
          }),
        ];
      }

      case 'morph.setState': {
        const { id, state, expectedRevision } = (cmd as MorphSetStateCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateMorph(curr, { state } as any);
        await this.repo.update(
          id,
          () => MorphSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'morph.state.set', { id })];
      }

      case 'morph.patchState': {
        const { id, patch, expectedRevision } = (cmd as MorphPatchStateCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateMorph(curr, { state: patch } as any);
        await this.repo.update(
          id,
          () => MorphSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'morph.state.patched', { id })];
      }

      case 'morph.setFacets': {
        const { id, facets, expectedRevision } = (cmd as MorphSetFacetsCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateMorph(curr, { facets } as any);
        await this.repo.update(
          id,
          () => MorphSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'morph.facets.set', { id })];
      }

      case 'morph.mergeFacets': {
        const { id, patch, expectedRevision } = (cmd as MorphMergeFacetsCmd)
          .payload;
        const curr = await this.mustGet(id);
        const merged = { ...((curr.shape as any).facets ?? {}), ...patch };
        const next = updateMorph(curr, { facets: merged } as any);
        await this.repo.update(
          id,
          () => MorphSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'morph.facets.merged', { id })];
      }

      case 'morph.setSignature': {
        const { id, signature, expectedRevision } = (
          cmd as MorphSetSignatureCmd
        ).payload;
        const curr = await this.mustGet(id);
        const next = updateMorph(curr, {
          signature: signature === undefined ? (null as any) : signature,
        } as any);
        await this.repo.update(
          id,
          () => MorphSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'morph.signature.set', { id })];
      }

      case 'morph.mergeSignature': {
        const { id, patch, expectedRevision } = (cmd as MorphMergeSignatureCmd)
          .payload;
        const curr = await this.mustGet(id);
        const merged = { ...((curr.shape as any).signature ?? {}), ...patch };
        const next = updateMorph(curr, { signature: merged } as any);
        await this.repo.update(
          id,
          () => MorphSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'morph.signature.merged', { id })];
      }

      // Runtime registry and execution
      case 'morph.defineRuntime': {
        const { name, transformer, options } = (cmd as MorphDefineRuntimeCmd)
          .payload;
        this.defineRuntime(name, transformer, options);
        return [this.emit(base, 'morph.runtime.defined', { name })];
      }

      case 'morph.composeRuntime': {
        const { name, steps, composedName } = (cmd as MorphComposeRuntimeCmd)
          .payload;
        const composed = this.composeRuntime(steps, composedName ?? name);
        return [
          this.emit(base, 'morph.runtime.composed', {
            name: composedName ?? name,
            steps,
            cost: composed.options.cost,
          }),
        ];
      }

      case 'morph.execute': {
        const { name, input, context } = (cmd as MorphExecuteCmd).payload;
        const fm = this.runtime.get(name);
        if (!fm) throw new Error(`runtime morph not found: ${name}`);

        const started = this.emit(base, 'morph.execution.started', { name });
        try {
          const output = fm.run(input as any, context);
          const completed = this.emit(base, 'morph.execution.completed', {
            name,
            result: output,
          });
          return [started, completed];
        } catch (err) {
          const failed = this.emit(base, 'morph.execution.failed', {
            name,
            reason: err instanceof Error ? err.message : String(err),
          });
          return [started, failed];
        }
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  // Schema helpers
  private async mustGet(id: string): Promise<MorphDoc> {
    const doc = await this.repo.get(id);
    if (!doc) throw new Error(`morph not found: ${id}`);
    return MorphSchema.parse(doc);
  }
}
