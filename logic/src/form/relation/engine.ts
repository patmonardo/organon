import type { Command, Event, EventBus } from "../triad/types";
import { InMemoryEventBus } from "../triad/bus";
import { startTrace, childSpan } from "../triad/trace";

import type { Repository, Concurrency } from "../../repository/repo";
import {
  type Relation,
  RelationSchema,
  createRelation,
  updateRelation,
  RelationDirection,
} from "../../schema/relation";
import { EntityRef } from "../../schema/entity";

// Typed command union for exhaustiveness + safety
export type RelationCreateCmd = {
  kind: "relation.create";
  payload: Parameters<typeof createRelation>[0];
  meta?: Record<string, unknown>;
};
export type RelationUpdateCmd = {
  kind: "relation.update";
  payload: {
    id: string;
    patch: Parameters<typeof updateRelation>[1];
    expectedRevision?: Concurrency["expectedRevision"];
  };
  meta?: Record<string, unknown>;
};
export type RelationDeleteCmd = {
  kind: "relation.delete";
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type RelationGetCmd = {
  kind: "relation.get";
  payload: { id: string };
  meta?: Record<string, unknown>;
};

// Endpoint/direction ops
export type RelationSetEndpointsCmd = {
  kind: "relation.setEndpoints";
  payload: { id: string; source: unknown; target: unknown }; // validated via EntityRef.parse
  meta?: Record<string, unknown>;
};
export type RelationSetDirectionCmd = {
  kind: "relation.setDirection";
  payload: { id: string; direction: unknown }; // validated via RelationDirection.parse
  meta?: Record<string, unknown>;
};
export type RelationInvertCmd = {
  kind: "relation.invert";
  payload: { id: string };
  meta?: Record<string, unknown>;
};

export type RelationCommand =
  | RelationCreateCmd
  | RelationUpdateCmd
  | RelationDeleteCmd
  | RelationGetCmd
  | RelationSetEndpointsCmd
  | RelationSetDirectionCmd
  | RelationInvertCmd;

/**
 * RelationEngine — repo-backed CRUD + topology ops for Relation docs.
 * Command-in, Event-out. Trace + scope meta via emit().
 */
export class RelationEngine {
  constructor(
    private readonly repo: Repository<Relation>,
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

  async handle(cmd: RelationCommand | Command): Promise<Event[]> {
    const base = startTrace(
      "RelationEngine",
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta
    );

    switch (cmd.kind) {
      case "relation.create": {
        const doc = createRelation(cmd.payload as any);
        const created = await this.repo.create(RelationSchema.parse(doc));
        const evt = this.emit(base, "relation.created", {
          id: created.shape.core.id,
          type: created.shape.core.type,
          kind: created.shape.core.kind,
          direction: created.shape.direction,
        });
        return [evt];
      }

      case "relation.update": {
        const { id, patch, expectedRevision } = cmd.payload;
        const current = await this.mustGet(id);
        const next = updateRelation(current, patch as any);
        const saved = await this.repo.update(
          id,
          () => RelationSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined
        );
        const evt = this.emit(base, "relation.updated", {
          id: saved.shape.core.id,
          revision: saved.revision,
        });
        return [evt];
      }

      case "relation.delete": {
        const { id } = cmd.payload;
        const ok = await this.repo.delete(id);
        const evt = this.emit(base, "relation.deleted", { id, ok: !!ok });
        return [evt];
      }

      case "relation.get": {
        const { id } = cmd.payload;
        const doc = await this.repo.get(id);
        const evt = this.emit(base, "relation.got", {
          id,
          relation: doc ?? null,
        });
        return [evt];
      }

      case "relation.setEndpoints": {
        const { id, source, target } = cmd.payload;
        const current = await this.mustGet(id);
        const src = EntityRef.parse(source);
        const tgt = EntityRef.parse(target);
        const next = updateRelation(current, { source: src, target: tgt });
        await this.repo.update(id, () => RelationSchema.parse(next));
        const evt = this.emit(base, "relation.endpoints.set", {
          id,
          source: src,
          target: tgt,
        });
        return [evt];
      }

      case "relation.setDirection": {
        const { id, direction } = cmd.payload;
        const current = await this.mustGet(id);
        const dir = RelationDirection.parse(direction);
        const next = updateRelation(current, { direction: dir });
        await this.repo.update(id, () => RelationSchema.parse(next));
        const evt = this.emit(base, "relation.direction.set", {
          id,
          direction: dir,
        });
        return [evt];
      }

      case "relation.invert": {
        const { id } = cmd.payload;
        const current = await this.mustGet(id);
        if (current.shape.direction === "bidirectional") {
          const evt = this.emit(base, "relation.inverted", {
            id,
            noop: true,
            direction: "bidirectional",
          });
          return [evt];
        }
        const next = updateRelation(current, {
          source: current.shape.target,
          target: current.shape.source,
        });
        await this.repo.update(id, () => RelationSchema.parse(next));
        const evt = this.emit(base, "relation.inverted", {
          id,
          direction: "directed",
        });
        return [evt];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  private async mustGet(id: string): Promise<Relation> {
    const doc = await this.repo.get(id);
    if (!doc) throw new Error(`relation not found: ${id}`);
    return doc;
  }
}
