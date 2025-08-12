import type { Command, Event, EventBus } from "../triad/types";
import { InMemoryEventBus } from "../triad/bus";
import { startTrace, childSpan } from "../triad/trace";
import type { Repository, Concurrency } from "../../repository/repo";
import {
  type Entity,
  EntitySchema,
  createEntity,
  updateEntity,
} from "../../schema/entity";

// Typed command union for exhaustiveness + safety
type EntityCreateCmd = {
  kind: "entity.create";
  payload: Parameters<typeof createEntity>[0];
  meta?: Record<string, unknown>;
};
type EntityUpdateCmd = {
  kind: "entity.update";
  payload: {
    id: string;
    patch: Parameters<typeof updateEntity>[1];
    expectedRevision?: Concurrency["expectedRevision"];
  };
  meta?: Record<string, unknown>;
};
type EntityDeleteCmd = {
  kind: "entity.delete";
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type EntityGetCmd = {
  kind: "entity.get";
  payload: { id: string };
  meta?: Record<string, unknown>;
};

export type EntityCommand =
  | EntityCreateCmd
  | EntityUpdateCmd
  | EntityDeleteCmd
  | EntityGetCmd;

/**
 * EntityEngine — repo-backed CRUD engine for Entity docs.
 * Command-in, Event-out. No legacy verb/messaging, no registry.
 */
export class EntityEngine {
  constructor(
    private readonly repo: Repository<Entity>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = "form" // optional namespace for events/trace meta
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  // small helper to standardize event emission + trace meta
  private emit(
    base: Record<string, any>,
    kind: Event["kind"],
    payload: Event["payload"],
    extraMeta?: Record<string, unknown>
  ): Event {
    const meta = childSpan(base as any, { action: kind, scope: this.scope, ...(extraMeta ?? {}) });
    const evt: Event = { kind, payload, meta };
    this.bus.publish(evt);
    return evt;
  }

  async handle(cmd: EntityCommand | Command): Promise<Event[]> {
    const base = startTrace(
      "EntityEngine",
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta
    );

    switch (cmd.kind) {
      case "entity.create": {
        const doc = createEntity(cmd.payload as any);
        const created = await this.repo.create(EntitySchema.parse(doc));
        const evt = this.emit(base, "entity.created", {
          id: created.shape.core.id,
          type: created.shape.core.type,
        });
        return [evt];
      }

      case "entity.update": {
        const { id, patch, expectedRevision } = cmd.payload;
        const current = await this.repo.get(id);
        if (!current) throw new Error(`entity not found: ${id}`);
        const next = updateEntity(current, patch as any);
        const saved = await this.repo.update(
          id,
          () => EntitySchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined
        );
        const evt = this.emit(base, "entity.updated", {
          id: saved.shape.core.id,
          revision: saved.revision,
        });
        return [evt];
      }

      case "entity.delete": {
        const { id } = cmd.payload;
        const ok = await this.repo.delete(id);
        const evt = ok
          ? this.emit(base, "entity.deleted", { id, ok: true })
          : this.emit(base, "entity.deleted", { id, ok: false });
        return [evt];
      }

      case "entity.get": {
        const { id } = cmd.payload;
        const doc = await this.repo.get(id);
        const evt = this.emit(base, "entity.got", { id, entity: doc ?? null });
        return [evt];
      }

      default:
        // Keep permissive for now to allow generic Command usage;
        // tighten to EntityCommand later if desired.
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
      }
  }
}
