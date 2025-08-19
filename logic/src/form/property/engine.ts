import type { Command, Event, EventBus } from '../triad/message';
import { InMemoryEventBus } from '../triad/bus';
import { startTrace, childSpan } from '../triad/trace';
import type { Repository, Concurrency } from '../../repository/repo';
import {
  type Property,
  PropertySchema,
  createProperty,
  updateProperty,
} from '../../schema/property';
import { EntityRef } from '../../schema/entity';
import { Id } from '../../schema/base';
import * as active from '../../schema/active';

// Core CRUD and mutation commands
export type PropertyCreateCmd = {
  kind: 'property.create';
  payload: Parameters<typeof createProperty>[0];
  meta?: Record<string, unknown>;
};
export type PropertyDeleteCmd = {
  kind: 'property.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type PropertyDescribeCmd = {
  kind: 'property.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type PropertySetCoreCmd = {
  kind: 'property.setCore';
  payload: {
    id: string;
    name?: string;
    type?: string;
    key?: string;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type PropertySetStateCmd = {
  kind: 'property.setState';
  payload: {
    id: string;
    state: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type PropertyPatchStateCmd = {
  kind: 'property.patchState';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type PropertySetFacetsCmd = {
  kind: 'property.setFacets';
  payload: {
    id: string;
    facets: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type PropertyMergeFacetsCmd = {
  kind: 'property.mergeFacets';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type PropertySetSignatureCmd = {
  kind: 'property.setSignature';
  payload: {
    id: string;
    signature?: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type PropertyMergeSignatureCmd = {
  kind: 'property.mergeSignature';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};

// Binding commands
export type PropertyBindEntityCmd = {
  kind: 'property.bindEntity';
  payload: { id: string; ref: unknown }; // validated with EntityRef.parse
  meta?: Record<string, unknown>;
};
export type PropertyBindRelationCmd = {
  kind: 'property.bindRelation';
  payload: { id: string; relationId: unknown }; // validated with Id.parse
  meta?: Record<string, unknown>;
};
export type PropertyClearBindingCmd = {
  kind: 'property.clearBinding';
  payload: { id: string };
  meta?: Record<string, unknown>;
};

// Value commands
export type PropertySetValueCmd = {
  kind: 'property.setValue';
  payload: { id: string; value: unknown; valueType?: unknown }; // valueType validated by schema
  meta?: Record<string, unknown>;
};
export type PropertyClearValueCmd = {
  kind: 'property.clearValue';
  payload: { id: string };
  meta?: Record<string, unknown>;
};

// Context command
export type PropertySetContextCmd = {
  kind: 'property.setContext';
  payload: { id: string; contextId: unknown }; // validated with Id.parse
  meta?: Record<string, unknown>;
};

export type PropertyCommand =
  | PropertyCreateCmd
  | PropertyDeleteCmd
  | PropertyDescribeCmd
  | PropertySetCoreCmd
  | PropertySetStateCmd
  | PropertyPatchStateCmd
  | PropertySetFacetsCmd
  | PropertyMergeFacetsCmd
  | PropertySetSignatureCmd
  | PropertyMergeSignatureCmd
  | PropertyBindEntityCmd
  | PropertyBindRelationCmd
  | PropertyClearBindingCmd
  | PropertySetValueCmd
  | PropertyClearValueCmd
  | PropertySetContextCmd;

/**
 * PropertyEngine — repo-backed engine for Property docs.
 * Unified verbs + binding/value/context ops. Command-in, Event-out (Triad).
 */
export class PropertyEngine {
  constructor(
    private readonly repo: Repository<Property>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'property',
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

  async handle(cmd: PropertyCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'PropertyEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      case 'property.create': {
        const doc = createProperty((cmd as PropertyCreateCmd).payload as any);
        const created = await this.repo.create(PropertySchema.parse(doc));
        return [
          this.emit(base, 'property.created', {
            id: created.shape.core.id,
            type: created.shape.core.type,
            key: created.shape.core.key,
            name: created.shape.core.name ?? null,
          }),
        ];
      }

      case 'property.delete': {
        const { id } = (cmd as PropertyDeleteCmd).payload;
        const ok = await this.repo.delete(id);
        return [this.emit(base, 'property.deleted', { id, ok: !!ok })];
      }

      case 'property.describe': {
        const { id } = (cmd as PropertyDescribeCmd).payload;
        const doc = await this.mustGet(id);
        return [
          this.emit(base, 'property.described', {
            id,
            type: doc.shape.core.type,
            key: doc.shape.core.key,
            name: doc.shape.core.name ?? null,
            state: doc.shape.state,
            signatureKeys: Object.keys((doc.shape as any).signature ?? {}),
            facetsKeys: Object.keys((doc.shape as any).facets ?? {}),
            binding: {
              entity: (doc.shape as any).entity ?? null,
              relationId: (doc.shape as any).relationId ?? null,
              contextId: (doc.shape as any).contextId ?? null,
            },
            hasValue: (doc.shape as any).value !== undefined,
          }),
        ];
      }

      case 'property.setCore': {
        const { id, name, type, key, expectedRevision } = (
          cmd as PropertySetCoreCmd
        ).payload;
        const curr = await this.mustGet(id);
        const next = updateProperty(curr, {
          core: {
            ...(name !== undefined ? { name } : {}),
            ...(type !== undefined ? { type } : {}),
            ...(key !== undefined ? { key } : {}),
          },
        } as any);
        const saved = await this.repo.update(
          id,
          () => PropertySchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [
          this.emit(base, 'property.core.set', {
            id,
            name: saved.shape.core.name ?? null,
            type: saved.shape.core.type,
            key: saved.shape.core.key,
          }),
        ];
      }

      case 'property.setState': {
        const { id, state, expectedRevision } = (cmd as PropertySetStateCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateProperty(curr, { state } as any);
        await this.repo.update(
          id,
          () => PropertySchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'property.state.set', { id })];
      }

      case 'property.patchState': {
        const { id, patch, expectedRevision } = (cmd as PropertyPatchStateCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateProperty(curr, { state: patch } as any);
        await this.repo.update(
          id,
          () => PropertySchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'property.state.patched', { id })];
      }

      case 'property.setFacets': {
        const { id, facets, expectedRevision } = (cmd as PropertySetFacetsCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateProperty(curr, { facets } as any);
        await this.repo.update(
          id,
          () => PropertySchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'property.facets.set', { id })];
      }

      case 'property.mergeFacets': {
        const { id, patch, expectedRevision } = (cmd as PropertyMergeFacetsCmd)
          .payload;
        const curr = await this.mustGet(id);
        const merged = { ...((curr.shape as any).facets ?? {}), ...patch };
        const next = updateProperty(curr, { facets: merged } as any);
        await this.repo.update(
          id,
          () => PropertySchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'property.facets.merged', { id })];
      }

      case 'property.setSignature': {
        const { id, signature, expectedRevision } = (
          cmd as PropertySetSignatureCmd
        ).payload;
        const curr = await this.mustGet(id);
        const next = updateProperty(curr, {
          signature: signature === undefined ? (null as any) : signature,
        } as any);
        await this.repo.update(
          id,
          () => PropertySchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'property.signature.set', { id })];
      }

      case 'property.mergeSignature': {
        const { id, patch, expectedRevision } = (
          cmd as PropertyMergeSignatureCmd
        ).payload;
        const curr = await this.mustGet(id);
        const merged = { ...((curr.shape as any).signature ?? {}), ...patch };
        const next = updateProperty(curr, { signature: merged } as any);
        await this.repo.update(
          id,
          () => PropertySchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'property.signature.merged', { id })];
      }

      // Bindings
      case 'property.bindEntity': {
        const { id, ref } = (cmd as PropertyBindEntityCmd).payload;
        const current = await this.mustGet(id);
        const parsed = EntityRef.parse(ref);
        const next = updateProperty(current, {
          entity: parsed,
          relationId: null,
        });
        await this.repo.update(id, () => PropertySchema.parse(next));
        return [
          this.emit(base, 'property.bound.entity', { id, entity: parsed }),
        ];
      }

      case 'property.bindRelation': {
        const { id, relationId } = (cmd as PropertyBindRelationCmd).payload;
        const current = await this.mustGet(id);
        const relId = Id.parse(relationId);
        const next = updateProperty(current, {
          entity: null,
          relationId: relId,
        });
        await this.repo.update(id, () => PropertySchema.parse(next));
        return [
          this.emit(base, 'property.bound.relation', { id, relationId: relId }),
        ];
      }

      case 'property.clearBinding': {
        const { id } = (cmd as PropertyClearBindingCmd).payload;
        const current = await this.mustGet(id);
        const next = updateProperty(current, {
          entity: null,
          relationId: null,
        });
        await this.repo.update(id, () => PropertySchema.parse(next));
        return [this.emit(base, 'property.binding.cleared', { id })];
      }

      // Values
      case 'property.setValue': {
        const { id, value, valueType } = (cmd as PropertySetValueCmd).payload;
        const current = await this.mustGet(id);
        const patch: any = { value };
        if (valueType !== undefined) patch.valueType = valueType;
        const next = updateProperty(current, patch);
        await this.repo.update(id, () => PropertySchema.parse(next));
        return [this.emit(base, 'property.value.set', { id })];
      }

      case 'property.clearValue': {
        const { id } = (cmd as PropertyClearValueCmd).payload;
        const current = await this.mustGet(id);
        const next = updateProperty(current, {
          value: undefined,
          valueType: undefined,
        } as any);
        await this.repo.update(id, () => PropertySchema.parse(next));
        return [this.emit(base, 'property.value.cleared', { id })];
      }

      // Context membership (rebind to a different context)
      case 'property.setContext': {
        const { id, contextId } = (cmd as PropertySetContextCmd).payload;
        const current = await this.mustGet(id);
        const ctxId = Id.parse(contextId);
        const next = updateProperty(current, { contextId: ctxId });
        await this.repo.update(id, () => PropertySchema.parse(next));
        return [
          this.emit(base, 'property.context.set', { id, contextId: ctxId }),
        ];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  private async mustGet(id: string): Promise<Property> {
    const doc = await this.repo.get(id);
    if (!doc) throw new Error(`property not found: ${id}`);
    return PropertySchema.parse(doc);
  }

  // ADR-0006: PropertyEngine interface — process/commit
  async process(
  properties: Array<active.ActiveProperty>,
    _particulars: any[] = [],
    _context?: any,
  ): Promise<{ actions: any[]; snapshot: { count: number } }> {
  const list = active.parseActiveProperties(properties);
    const actions: any[] = [];
    for (const p of list) {
      if (p.revoked === true) {
        actions.push({ type: 'property.delete', id: p.id });
        continue;
      }
      actions.push({
        type: 'property.upsert',
        id: p.id,
        subjectId: p.subjectId,
        key: p.key,
        value: p.value,
        dtype: p.dtype,
      });
    }
    return { actions, snapshot: { count: list.length } };
  }

  async commit(actions: any[], _snapshot: { count: number }) {
    const events: Event[] = [];
    const defaultContextId = 'ctx:world'; // assumption: fallback when no context supplied
    for (const a of actions) {
      if (a.type === 'property.delete') {
        const [evt] = await this.handle({
          kind: 'property.delete',
          payload: { id: a.id },
        } as any);
        events.push(evt);
      } else if (a.type === 'property.upsert') {
        const id = a.id as string;
        const existing = await this.repo.get(id);
        if (!existing) {
          // Create with minimal core and context, then bind/value
          const [created] = await this.handle({
            kind: 'property.create',
            payload: {
              id,
              type: 'system.Property',
              key: a.key,
              contextId: a.contextId ?? defaultContextId,
            },
          } as any);
          events.push(created);
          if (a.subjectId) {
            const [bound] = await this.handle({
              kind: 'property.bindEntity',
              payload: { id, ref: { id: a.subjectId, type: 'system.Entity' } },
            } as any);
            events.push(bound);
          }
          if (a.value !== undefined) {
            const [valEvt] = await this.handle({
              kind: 'property.setValue',
              payload: { id, value: a.value, valueType: a.dtype },
            } as any);
            events.push(valEvt);
          }
        } else {
          // Upsert existing: ensure key matches (setCore) and value applied
          const current = PropertySchema.parse(existing);
          if (current.shape.core.key !== a.key) {
            const [coreEvt] = await this.handle({
              kind: 'property.setCore',
              payload: { id, key: a.key },
            } as any);
            events.push(coreEvt);
          }
          if (a.subjectId) {
            const [bound] = await this.handle({
              kind: 'property.bindEntity',
              payload: { id, ref: { id: a.subjectId, type: 'system.Entity' } },
            } as any);
            events.push(bound);
          }
          if (a.value !== undefined) {
            const [valEvt] = await this.handle({
              kind: 'property.setValue',
              payload: { id, value: a.value, valueType: a.dtype },
            } as any);
            events.push(valEvt);
          }
        }
      }
    }
    return { success: true, errors: [], events } as any;
  }
}
