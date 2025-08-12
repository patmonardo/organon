import type { Command, Event, EventBus } from "../triad/types";
import { InMemoryEventBus } from "../triad/bus";
import { startTrace, childSpan } from "../triad/trace";

import type { Repository, Concurrency } from "../../repository/repo";
import {
  type Property,
  PropertySchema,
  createProperty,
  updateProperty,
} from "../../schema/property";
import { EntityRef } from "../../schema/entity";
import { Id } from "../../schema/base";

// Typed command union for exhaustiveness + safety
export type PropertyCreateCmd = {
  kind: "property.create";
  payload: Parameters<typeof createProperty>[0];
  meta?: Record<string, unknown>;
};
export type PropertyUpdateCmd = {
  kind: "property.update";
  payload: {
    id: string;
    patch: Parameters<typeof updateProperty>[1];
    expectedRevision?: Concurrency["expectedRevision"];
  };
  meta?: Record<string, unknown>;
};
export type PropertyDeleteCmd = {
  kind: "property.delete";
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type PropertyGetCmd = {
  kind: "property.get";
  payload: { id: string };
  meta?: Record<string, unknown>;
};

// Binding commands
export type PropertyBindEntityCmd = {
  kind: "property.bindEntity";
  payload: { id: string; ref: unknown }; // validated with EntityRef.parse
  meta?: Record<string, unknown>;
};
export type PropertyBindRelationCmd = {
  kind: "property.bindRelation";
  payload: { id: string; relationId: unknown }; // validated with Id.parse
  meta?: Record<string, unknown>;
};
export type PropertyClearBindingCmd = {
  kind: "property.clearBinding";
  payload: { id: string };
  meta?: Record<string, unknown>;
};

// Value commands
export type PropertySetValueCmd = {
  kind: "property.setValue";
  payload: { id: string; value: unknown; valueType?: unknown }; // valueType validated by schema
  meta?: Record<string, unknown>;
};
export type PropertyClearValueCmd = {
  kind: "property.clearValue";
  payload: { id: string };
  meta?: Record<string, unknown>;
};

// Context command
export type PropertySetContextCmd = {
  kind: "property.setContext";
  payload: { id: string; contextId: unknown }; // validated with Id.parse
  meta?: Record<string, unknown>;
};

export type PropertyCommand =
  | PropertyCreateCmd
  | PropertyUpdateCmd
  | PropertyDeleteCmd
  | PropertyGetCmd
  | PropertyBindEntityCmd
  | PropertyBindRelationCmd
  | PropertyClearBindingCmd
  | PropertySetValueCmd
  | PropertyClearValueCmd
  | PropertySetContextCmd;

/**
 * PropertyEngine — repo-backed CRUD engine for Property docs.
 * Command-in, Event-out. Trace + scope meta via emit(). Binding + value ops included.
 */
export class PropertyEngine {
  constructor(
    private readonly repo: Repository<Property>,
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

  async handle(cmd: PropertyCommand | Command): Promise<Event[]> {
    const base = startTrace(
      "PropertyEngine",
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta
    );

    switch (cmd.kind) {
      case "property.create": {
        const doc = createProperty(cmd.payload as any);
        const created = await this.repo.create(PropertySchema.parse(doc));
        const evt = this.emit(base, "property.created", {
          id: created.shape.core.id,
          type: created.shape.core.type,
          key: created.shape.core.key,
        });
        return [evt];
      }

      case "property.update": {
        const { id, patch, expectedRevision } = cmd.payload;
        const current = await this.repo.get(id);
        if (!current) throw new Error(`property not found: ${id}`);
        const next = updateProperty(current, patch as any);
        const saved = await this.repo.update(
          id,
          () => PropertySchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined
        );
        const evt = this.emit(base, "property.updated", {
          id: saved.shape.core.id,
          revision: saved.revision,
        });
        return [evt];
      }

      case "property.delete": {
        const { id } = cmd.payload;
        const ok = await this.repo.delete(id);
        const evt = this.emit(base, "property.deleted", { id, ok: !!ok });
        return [evt];
      }

      case "property.get": {
        const { id } = cmd.payload;
        const doc = await this.repo.get(id);
        const evt = this.emit(base, "property.got", {
          id,
          property: doc ?? null,
        });
        return [evt];
      }

      // Bindings
      case "property.bindEntity": {
        const { id, ref } = cmd.payload;
        const current = await this.mustGet(id);
        const parsed = EntityRef.parse(ref);
        const next = updateProperty(current, {
          entity: parsed,
          relationId: null,
        });
        await this.repo.update(id, () => PropertySchema.parse(next));
        const evt = this.emit(base, "property.bound.entity", {
          id,
          entity: parsed,
        });
        return [evt];
      }

      case "property.bindRelation": {
        const { id, relationId } = cmd.payload;
        const current = await this.mustGet(id);
        const relId = Id.parse(relationId);
        const next = updateProperty(current, {
          entity: null,
          relationId: relId,
        });
        await this.repo.update(id, () => PropertySchema.parse(next));
        const evt = this.emit(base, "property.bound.relation", {
          id,
          relationId: relId,
        });
        return [evt];
      }

      case "property.clearBinding": {
        const { id } = cmd.payload;
        const current = await this.mustGet(id);
        const next = updateProperty(current, {
          entity: null,
          relationId: null,
        });
        await this.repo.update(id, () => PropertySchema.parse(next));
        const evt = this.emit(base, "property.binding.cleared", { id });
        return [evt];
      }

      // Values
      case "property.setValue": {
        const { id, value, valueType } = cmd.payload;
        const current = await this.mustGet(id);
        const patch: any = { value };
        if (valueType !== undefined) patch.valueType = valueType;
        const next = updateProperty(current, patch);
        await this.repo.update(id, () => PropertySchema.parse(next));
        const evt = this.emit(base, "property.value.set", { id });
        return [evt];
      }

      case "property.clearValue": {
        const { id } = cmd.payload;
        const current = await this.mustGet(id);
        const next = updateProperty(current, {
          value: undefined,
          valueType: undefined,
        });
        await this.repo.update(id, () => PropertySchema.parse(next));
        const evt = this.emit(base, "property.value.cleared", { id });
        return [evt];
      }

      // Context membership (rebind to a different context)
      case "property.setContext": {
        const { id, contextId } = cmd.payload;
        const current = await this.mustGet(id);
        const ctxId = Id.parse(contextId);
        const next = updateProperty(current, { contextId: ctxId });
        await this.repo.update(id, () => PropertySchema.parse(next));
        const evt = this.emit(base, "property.context.set", {
          id,
          contextId: ctxId,
        });
        return [evt];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  private async mustGet(id: string): Promise<Property> {
    const doc = await this.repo.get(id);
    if (!doc) throw new Error(`property not found: ${id}`);
    return doc;
  }
}
