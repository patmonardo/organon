import type { Command, Event, EventBus } from '../triad/types';
import { InMemoryEventBus } from '../triad/bus';
import { startTrace, childSpan } from '../triad/trace';
import { FormShape } from './shape';
import type { FormData, FormState } from '../../schema/shape';
import type { Repository } from '../../repository/repo';

type ShapeDoc = {
  shape: {
    core: { id: string; type: string };
    definition?: { id: string; name: string };
    data?: unknown;
    state?: unknown;
  };
  revision?: number;
};
type SerializeShape = (s: FormShape) => ShapeDoc;
type DeserializeShape = (d: ShapeDoc) => FormShape;

// Typed command union for exhaustiveness + safety
type ShapeInstantiateCmd = {
  kind: 'shape.instantiate';
  payload: {
    id?: string;
    definitionId: string;
    definitionName: string;
    data?: FormData;
    state?: FormState;
  };
  meta?: Record<string, unknown>;
};
type ShapeDestroyCmd = {
  kind: 'shape.destroy';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type ShapeSetDataCmd = {
  kind: 'shape.setData';
  payload: { id: string; data?: FormData };
  meta?: Record<string, unknown>;
};
type ShapeMergeDataCmd = {
  kind: 'shape.mergeData';
  payload: { id: string; patch: Partial<FormData> };
  meta?: Record<string, unknown>;
};
type ShapeSetStateCmd = {
  kind: 'shape.setState';
  payload: { id: string; state: FormState };
  meta?: Record<string, unknown>;
};
type ShapePatchStateCmd = {
  kind: 'shape.patchState';
  payload: { id: string; patch: Partial<FormState> };
  meta?: Record<string, unknown>;
};
type ShapeDescribeCmd = {
  kind: 'shape.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};

export type ShapeCommand =
  | ShapeInstantiateCmd
  | ShapeDestroyCmd
  | ShapeSetDataCmd
  | ShapeMergeDataCmd
  | ShapeSetStateCmd
  | ShapePatchStateCmd
  | ShapeDescribeCmd;

/**
 * ShapeEngine — runtime manager for FormShape instances (Principle of Form).
 * Command-in, Event-out. In-memory store, optional repo persistence.
 */
export class ShapeEngine {
  private readonly shapes = new Map<string, FormShape>();
  private readonly serialize: SerializeShape;
  private readonly deserialize: DeserializeShape;

  constructor(
    private readonly repo?: Repository<any>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'form',
    options?: {
      serialize?: SerializeShape;
      deserialize?: DeserializeShape;
    },
  ) {
    this.serialize =
      options?.serialize ??
      ((s) =>
        ({
          shape: {
            core: { id: s.id, type: 'form.Shape' },
            definition: { id: s.definitionId, name: s.definitionName },
            data: s.data,
            state: s.state,
          },
        } as ShapeDoc));
    this.deserialize =
      options?.deserialize ??
      ((d) => {
        const shape = (d as any).shape ?? d;
        const core = shape.core ?? {};
        const def = shape.definition ?? {};
        return FormShape.create({
          id: core.id,
          definitionId: def.id,
          definitionName: def.name,
          data: shape.data,
          state: shape.state,
        });
      });
  }

  get eventBus(): EventBus {
    return this.bus;
  }

  // Optional: hydrate from repo into memory
  async load(id: string): Promise<FormShape | undefined> {
    if (!this.repo) return this.shapes.get(id);
    const doc = await this.repo.get(id);
    if (!doc) return undefined;
    const s = this.deserialize(doc as any);
    this.shapes.set(id, s);
    return s;
  }

  getShape(id: string): FormShape | undefined {
    return this.shapes.get(id);
  }

  // Standardized emit with trace + scope
  private emit(
    base: Record<string, any>,
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

  async handle(cmd: ShapeCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'ShapeEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      case 'shape.instantiate': {
        const { id, definitionId, definitionName, data, state } = cmd.payload;
        const shape = FormShape.create({
          id,
          definitionId,
          definitionName,
          data,
          state,
        });
        this.shapes.set(shape.id, shape);
        await this.persist(shape);
        const evt = this.emit(base, 'shape.instantiated', {
          id: shape.id,
          definitionId: shape.definitionId,
          definitionName: shape.definitionName,
        });
        return [evt];
      }

      case 'shape.destroy': {
        const { id } = cmd.payload;
        const existed = this.shapes.delete(id);
        if (this.repo) await this.repo.delete(id);
        const evt = this.emit(base, 'shape.destroyed', { id, ok: existed });
        return [evt];
      }

      case 'shape.setData': {
        const { id, data } = cmd.payload;
        const s = this.mustGet(id);
        s.setData(data);
        await this.persist(s);
        const evt = this.emit(base, 'shape.data.set', { id });
        return [evt];
      }

      case 'shape.mergeData': {
        const { id, patch } = cmd.payload;
        const s = this.mustGet(id);
        s.mergeData(patch);
        await this.persist(s);
        const evt = this.emit(base, 'shape.data.merged', { id });
        return [evt];
      }

      case 'shape.setState': {
        const { id, state } = cmd.payload;
        const s = this.mustGet(id);
        s.setState(state);
        await this.persist(s);
        const evt = this.emit(base, 'shape.state.set', {
          id,
          status: s.state.status,
        });
        return [evt];
      }

      case 'shape.patchState': {
        const { id, patch } = cmd.payload;
        const s = this.mustGet(id);
        s.patchState(patch);
        await this.persist(s);
        const evt = this.emit(base, 'shape.state.patched', {
          id,
          status: s.state.status,
        });
        return [evt];
      }

      case 'shape.describe': {
        const { id } = cmd.payload;
        const s = this.mustGet(id);
        const evt = this.emit(base, 'shape.described', s.getInfo());
        return [evt];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  private mustGet(id: string): FormShape {
    const s = this.shapes.get(id);
    if (!s) throw new Error(`shape not found: ${id}`);
    return s;
  }

  private async persist(s: FormShape): Promise<void> {
    if (!this.repo) return;
    const id = s.id;
    const current = await this.repo.get(id);
    const doc = this.serialize(s);
    if (current) {
      await this.repo.update(id, () => doc);
    } else {
      await this.repo.create(doc as any);
    }
  }
}
