import type { Command, Event, EventBus } from "../triad/types";
import { InMemoryEventBus } from "../triad/bus";
import { startTrace, childSpan } from "../triad/trace";

import type { Repository, Concurrency } from "../../repository/repo";
import {
  type Context,
  ContextSchema,
  createContext,
  updateContext,
} from "../../schema/context";
import { EntityRef } from "../../schema/entity";

// Typed command union for exhaustiveness + safety
export type ContextCreateCmd = {
  kind: "context.create";
  payload: Parameters<typeof createContext>[0];
  meta?: Record<string, unknown>;
};
export type ContextUpdateCmd = {
  kind: "context.update";
  payload: {
    id: string;
    patch: Parameters<typeof updateContext>[1];
    expectedRevision?: Concurrency["expectedRevision"];
  };
  meta?: Record<string, unknown>;
};
export type ContextDeleteCmd = {
  kind: "context.delete";
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type ContextGetCmd = {
  kind: "context.get";
  payload: { id: string };
  meta?: Record<string, unknown>;
};

// Membership: entities
export type ContextAddEntityCmd = {
  kind: "context.addEntity";
  payload: { id: string; ref: unknown }; // validated with EntityRef.parse
  meta?: Record<string, unknown>;
};
export type ContextAddEntitiesCmd = {
  kind: "context.addEntities";
  payload: { id: string; refs: unknown[] };
  meta?: Record<string, unknown>;
};
export type ContextRemoveEntityCmd = {
  kind: "context.removeEntity";
  payload: { id: string; ref: unknown };
  meta?: Record<string, unknown>;
};
export type ContextClearEntitiesCmd = {
  kind: "context.clearEntities";
  payload: { id: string };
  meta?: Record<string, unknown>;
};

// Membership: relations (by id string)
export type ContextAddRelationCmd = {
  kind: "context.addRelation";
  payload: { id: string; relationId: string };
  meta?: Record<string, unknown>;
};
export type ContextAddRelationsCmd = {
  kind: "context.addRelations";
  payload: { id: string; relationIds: string[] };
  meta?: Record<string, unknown>;
};
export type ContextRemoveRelationCmd = {
  kind: "context.removeRelation";
  payload: { id: string; relationId: string };
  meta?: Record<string, unknown>;
};
export type ContextClearRelationsCmd = {
  kind: "context.clearRelations";
  payload: { id: string };
  meta?: Record<string, unknown>;
};

export type ContextCommand =
  | ContextCreateCmd
  | ContextUpdateCmd
  | ContextDeleteCmd
  | ContextGetCmd
  | ContextAddEntityCmd
  | ContextAddEntitiesCmd
  | ContextRemoveEntityCmd
  | ContextClearEntitiesCmd
  | ContextAddRelationCmd
  | ContextAddRelationsCmd
  | ContextRemoveRelationCmd
  | ContextClearRelationsCmd;

/**
 * ContextEngine — repo-backed CRUD engine for Context docs.
 * Command-in, Event-out. Trace+scope meta via emit(). Idempotent membership ops.
 */
export class ContextEngine {
  constructor(
    private readonly repo: Repository<Context>,
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

  async handle(cmd: ContextCommand | Command): Promise<Event[]> {
    const base = startTrace(
      "ContextEngine",
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta
    );

    switch (cmd.kind) {
      case "context.create": {
        const doc = createContext(cmd.payload as any);
        const created = await this.repo.create(ContextSchema.parse(doc));
        const evt = this.emit(base, "context.created", {
          id: created.shape.core.id,
          type: created.shape.core.type,
        });
        return [evt];
      }

      case "context.update": {
        const { id, patch, expectedRevision } = cmd.payload;
        const current = await this.repo.get(id);
        if (!current) throw new Error(`context not found: ${id}`);
        const next = updateContext(current, patch as any);
        const saved = await this.repo.update(
          id,
          () => ContextSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined
        );
        const evt = this.emit(base, "context.updated", {
          id: saved.shape.core.id,
          revision: saved.revision,
        });
        return [evt];
      }

      case "context.delete": {
        const { id } = cmd.payload;
        const ok = await this.repo.delete(id);
        const evt = this.emit(base, "context.deleted", { id, ok: !!ok });
        return [evt];
      }

      case "context.get": {
        const { id } = cmd.payload;
        const doc = await this.repo.get(id);
        const evt = this.emit(base, "context.got", {
          id,
          context: doc ?? null,
        });
        return [evt];
      }

      // Entities
      case "context.addEntity": {
        const { id, ref } = cmd.payload;
        const current = await this.mustGet(id);
        const parsed = EntityRef.parse(ref);
        const exists = new Set(
          current.shape.entities.map((e) => `${e.type}:${e.id}`)
        );
        const next = exists.has(`${parsed.type}:${parsed.id}`)
          ? current
          : updateContext(current, {
              entities: [...current.shape.entities, parsed],
            } as any);
        const saved = await this.repo.update(id, () =>
          ContextSchema.parse(next)
        );
        const evt = this.emit(base, "context.entity.added", {
          id,
          entity: parsed,
        });
        return [evt];
      }

      case "context.addEntities": {
        const { id, refs } = cmd.payload;
        const current = await this.mustGet(id);
        const exists = new Set(
          current.shape.entities.map((e) => `${e.type}:${e.id}`)
        );
        const toAdd = (refs ?? [])
          .map((r) => EntityRef.parse(r))
          .filter((r) => !exists.has(`${r.type}:${r.id}`));
        const next =
          toAdd.length === 0
            ? current
            : updateContext(current, {
                entities: [...current.shape.entities, ...toAdd],
              } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        const evt = this.emit(base, "context.entities.added", {
          id,
          count: toAdd.length,
        });
        return [evt];
      }

      case "context.removeEntity": {
        const { id, ref } = cmd.payload;
        const current = await this.mustGet(id);
        const parsed = EntityRef.parse(ref);
        const nextList = current.shape.entities.filter(
          (e) => !(e.id === parsed.id && e.type === parsed.type)
        );
        const next =
          nextList.length === current.shape.entities.length
            ? current
            : updateContext(current, { entities: nextList } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        const evt = this.emit(base, "context.entity.removed", {
          id,
          entity: parsed,
        });
        return [evt];
      }

      case "context.clearEntities": {
        const { id } = cmd.payload;
        const current = await this.mustGet(id);
        const next =
          current.shape.entities.length === 0
            ? current
            : updateContext(current, { entities: [] } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        const evt = this.emit(base, "context.entities.cleared", { id });
        return [evt];
      }

      // Relations by id
      case "context.addRelation": {
        const { id, relationId } = cmd.payload;
        const current = await this.mustGet(id);
        const set = new Set(current.shape.relations);
        const next = set.has(relationId)
          ? current
          : updateContext(current, {
              relations: [...current.shape.relations, relationId],
            } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        const evt = this.emit(base, "context.relation.added", {
          id,
          relationId,
        });
        return [evt];
      }

      case "context.addRelations": {
        const { id, relationIds } = cmd.payload;
        const current = await this.mustGet(id);
        const set = new Set(current.shape.relations);
        const toAdd = (relationIds ?? []).filter((rid) => !set.has(rid));
        const next =
          toAdd.length === 0
            ? current
            : updateContext(current, {
                relations: [...current.shape.relations, ...toAdd],
              } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        const evt = this.emit(base, "context.relations.added", {
          id,
          count: toAdd.length,
        });
        return [evt];
      }

      case "context.removeRelation": {
        const { id, relationId } = cmd.payload;
        const current = await this.mustGet(id);
        const nextList = current.shape.relations.filter(
          (r) => r !== relationId
        );
        const next =
          nextList.length === current.shape.relations.length
            ? current
            : updateContext(current, { relations: nextList } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        const evt = this.emit(base, "context.relation.removed", {
          id,
          relationId,
        });
        return [evt];
      }

      case "context.clearRelations": {
        const { id } = cmd.payload;
        const current = await this.mustGet(id);
        const next =
          current.shape.relations.length === 0
            ? current
            : updateContext(current, { relations: [] } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        const evt = this.emit(base, "context.relations.cleared", { id });
        return [evt];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  private async mustGet(id: string): Promise<Context> {
    const doc = await this.repo.get(id);
    if (!doc) throw new Error(`context not found: ${id}`);
    return doc;
  }
}
