import type { Command, Event, EventBus } from "../triad/types";
import { InMemoryEventBus } from "../triad/bus";
import { startTrace, childSpan } from "../triad/trace";

import type { Repository, Concurrency } from "../../repository/repo";
import {
  type Morph as MorphDoc,
  MorphSchema,
  createMorph,
  updateMorph,
} from "../../schema/morph";

import { FormMorph } from "./morph";
import type { MorphTransformer, MorphOptions } from "./core";

// Typed command union for exhaustiveness + safety (schema-level definitions)
export type MorphCreateCmd = {
  kind: "morph.create";
  payload: Parameters<typeof createMorph>[0];
  meta?: Record<string, unknown>;
};
export type MorphUpdateCmd = {
  kind: "morph.update";
  payload: {
    id: string;
    patch: Parameters<typeof updateMorph>[1];
    expectedRevision?: Concurrency["expectedRevision"];
  };
  meta?: Record<string, unknown>;
};
export type MorphDeleteCmd = {
  kind: "morph.delete";
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type MorphGetCmd = {
  kind: "morph.get";
  payload: { id: string };
  meta?: Record<string, unknown>;
};

// Runtime registry commands (engine-local, not persisted)
export type MorphDefineRuntimeCmd = {
  kind: "morph.defineRuntime";
  payload: {
    name: string;
    transformer: MorphTransformer<any, any>;
    options?: Partial<MorphOptions>;
  };
  meta?: Record<string, unknown>;
};
export type MorphComposeRuntimeCmd = {
  kind: "morph.composeRuntime";
  payload: { name: string; steps: string[]; composedName?: string };
  meta?: Record<string, unknown>;
};
export type MorphExecuteCmd = {
  kind: "morph.execute";
  payload: { name: string; input: unknown; context?: any };
  meta?: Record<string, unknown>;
};

export type MorphCommand =
  | MorphCreateCmd
  | MorphUpdateCmd
  | MorphDeleteCmd
  | MorphGetCmd
  | MorphDefineRuntimeCmd
  | MorphComposeRuntimeCmd
  | MorphExecuteCmd;

/**
 * MorphEngine — manages schema Morph docs and a local runtime registry of FormMorphs.
 * Command-in, Event-out. Trace + scope meta via emit(). Execution uses engine-local registry.
 */
export class MorphEngine {
  private readonly runtime = new Map<string, FormMorph<any, any>>();

  constructor(
    private readonly repo: Repository<MorphDoc>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = "form"
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  // Standardized emit with trace + scope
  private emit(
    base: Record<string, unknown>,
    kind: Event["kind"],
    payload: Event["payload"],
    extraMeta?: Record<string, unknown>
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
    options?: Partial<MorphOptions>
  ) {
    const fm = FormMorph.define(name, transformer, options ?? {});
    this.runtime.set(name, fm);
    return fm;
  }

  private composeRuntime(
    steps: string[],
    composedName?: string
  ): FormMorph<any, any> {
    if (steps.length === 0) throw new Error("composeRuntime: steps required");
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
      "MorphEngine",
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta
    );

    switch (cmd.kind) {
      // Schema-backed definitions
      case "morph.create": {
        const doc = createMorph(cmd.payload as any);
        const created = await this.repo.create(MorphSchema.parse(doc));
        const evt = this.emit(base, "morph.created", {
          id: created.shape.core.id,
          type: created.shape.core.type,
          name: created.shape.core.name,
        });
        return [evt];
      }

      case "morph.update": {
        const { id, patch, expectedRevision } =
          cmd.payload as MorphUpdateCmd["payload"];
        const current = await this.repo.get(id);
        if (!current) throw new Error(`morph not found: ${id}`);
        const next = updateMorph(current, patch as any);
        const saved = await this.repo.update(
          id,
          () => MorphSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined
        );
        const evt = this.emit(base, "morph.updated", {
          id: saved.shape.core.id,
          revision: saved.revision,
        });
        return [evt];
      }

      case "morph.delete": {
        const { id } = cmd.payload as MorphDeleteCmd["payload"];
        const ok = await this.repo.delete(id);
        const evt = this.emit(base, "morph.deleted", { id, ok: !!ok });
        return [evt];
      }

      case "morph.get": {
        const { id } = cmd.payload as MorphGetCmd["payload"];
        const doc = await this.repo.get(id);
        const evt = this.emit(base, "morph.got", { id, morph: doc ?? null });
        return [evt];
      }

      // Runtime registry
      case "morph.defineRuntime": {
        const { name, transformer, options } =
          cmd.payload as MorphDefineRuntimeCmd["payload"];
        this.defineRuntime(name, transformer, options);
        const evt = this.emit(base, "morph.runtime.defined", { name });
        return [evt];
      }

      case "morph.composeRuntime": {
        const { name, steps, composedName } =
          cmd.payload as MorphComposeRuntimeCmd["payload"];
        const composed = this.composeRuntime(steps, composedName ?? name);
        const evt = this.emit(base, "morph.runtime.composed", {
          name: composedName ?? name,
          steps,
          cost: composed.options.cost,
        });
        return [evt];
      }

      case "morph.execute": {
        const { name, input, context } =
          cmd.payload as MorphExecuteCmd["payload"];
        const fm = this.runtime.get(name);
        if (!fm) throw new Error(`runtime morph not found: ${name}`);

        const started = this.emit(base, "morph.execution.started", { name });
        try {
          const output = fm.run(input as any, context);
          const completed = this.emit(base, "morph.execution.completed", {
            name,
            result: output,
          });
          return [started, completed];
        } catch (err) {
          const failed = this.emit(base, "morph.execution.failed", {
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
    return doc;
  }
}
