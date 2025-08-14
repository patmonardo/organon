import type { Command, Event, EventBus } from '../triad/message';
import { InMemoryEventBus } from '../triad/bus';
import { startTrace, childSpan } from '../triad/trace';

import type { Repository, Concurrency } from '../../repository/repo';
import {
  type Relation,
  RelationSchema,
  createRelation,
  updateRelation,
  RelationDirection,
} from '../../schema/relation';
import { EntityRef } from '../../schema/entity';

// Core commands (unified verbs)
export type RelationCreateCmd = {
  kind: 'relation.create';
  payload: Parameters<typeof createRelation>[0];
  meta?: Record<string, unknown>;
};
export type RelationDeleteCmd = {
  kind: 'relation.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type RelationDescribeCmd = {
  kind: 'relation.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type RelationSetCoreCmd = {
  kind: 'relation.setCore';
  payload: {
    id: string;
    name?: string;
    type?: string;
    kind?: string;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type RelationSetStateCmd = {
  kind: 'relation.setState';
  payload: {
    id: string;
    state: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type RelationPatchStateCmd = {
  kind: 'relation.patchState';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type RelationSetFacetsCmd = {
  kind: 'relation.setFacets';
  payload: {
    id: string;
    facets: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type RelationMergeFacetsCmd = {
  kind: 'relation.mergeFacets';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type RelationSetSignatureCmd = {
  kind: 'relation.setSignature';
  payload: {
    id: string;
    signature?: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type RelationMergeSignatureCmd = {
  kind: 'relation.mergeSignature';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};

// Endpoint/direction ops
export type RelationSetEndpointsCmd = {
  kind: 'relation.setEndpoints';
  payload: { id: string; source: unknown; target: unknown }; // validated via EntityRef.parse
  meta?: Record<string, unknown>;
};
export type RelationSetDirectionCmd = {
  kind: 'relation.setDirection';
  payload: { id: string; direction: unknown }; // validated via RelationDirection.parse
  meta?: Record<string, unknown>;
};
export type RelationInvertCmd = {
  kind: 'relation.invert';
  payload: { id: string };
  meta?: Record<string, unknown>;
};

export type RelationCommand =
  | RelationCreateCmd
  | RelationDeleteCmd
  | RelationDescribeCmd
  | RelationSetCoreCmd
  | RelationSetStateCmd
  | RelationPatchStateCmd
  | RelationSetFacetsCmd
  | RelationMergeFacetsCmd
  | RelationSetSignatureCmd
  | RelationMergeSignatureCmd
  | RelationSetEndpointsCmd
  | RelationSetDirectionCmd
  | RelationInvertCmd;

/**
 * RelationEngine — repo-backed engine for Relation docs.
 * Command-in, Event-out. Unified verbs + topology ops. Triad bus + scoped tracing.
 */
export class RelationEngine {
  constructor(
    private readonly repo: Repository<Relation>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'relation',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

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

  private async mustGet(id: string): Promise<Relation> {
    const doc = await this.repo.get(id);
    if (!doc) throw new Error(`relation not found: ${id}`);
    return RelationSchema.parse(doc);
  }

  async handle(cmd: RelationCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'RelationEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      case 'relation.create': {
        const created = await this.repo.create(
          RelationSchema.parse(
            createRelation((cmd as RelationCreateCmd).payload as any),
          ),
        );
        return [
          this.emit(base, 'relation.created', {
            id: created.shape.core.id,
            type: created.shape.core.type,
            kind: created.shape.core.kind,
            direction: created.shape.direction,
            source: created.shape.source,
            target: created.shape.target,
          }),
        ];
      }

      case 'relation.delete': {
        const { id } = (cmd as RelationDeleteCmd).payload;
        const ok = await this.repo.delete(id);
        return [this.emit(base, 'relation.deleted', { id, ok: !!ok })];
      }

      case 'relation.describe': {
        const { id } = (cmd as RelationDescribeCmd).payload;
        const doc = await this.mustGet(id);
        return [
          this.emit(base, 'relation.described', {
            id,
            type: doc.shape.core.type,
            name: doc.shape.core.name ?? null,
            kind: doc.shape.core.kind,
            direction: doc.shape.direction,
            source: doc.shape.source,
            target: doc.shape.target,
            state: doc.shape.state,
            signatureKeys: Object.keys((doc.shape as any).signature ?? {}),
            facetsKeys: Object.keys((doc.shape as any).facets ?? {}),
          }),
        ];
      }

      case 'relation.setCore': {
        const { id, name, type, kind, expectedRevision } = (
          cmd as RelationSetCoreCmd
        ).payload;
        const curr = await this.mustGet(id);
        const next = updateRelation(curr, {
          core: {
            ...(name !== undefined ? { name } : {}),
            ...(type !== undefined ? { type } : {}),
            ...(kind !== undefined ? { kind } : {}),
          },
        } as any);
        const saved = await this.repo.update(
          id,
          () => RelationSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [
          this.emit(base, 'relation.core.set', {
            id,
            name: saved.shape.core.name ?? null,
            type: saved.shape.core.type,
            kind: saved.shape.core.kind,
          }),
        ];
      }

      case 'relation.setState': {
        const { id, state, expectedRevision } = (cmd as RelationSetStateCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateRelation(curr, { state } as any);
        await this.repo.update(
          id,
          () => RelationSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'relation.state.set', { id })];
      }

      case 'relation.patchState': {
        const { id, patch, expectedRevision } = (cmd as RelationPatchStateCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateRelation(curr, { state: patch } as any);
        await this.repo.update(
          id,
          () => RelationSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'relation.state.patched', { id })];
      }

      case 'relation.setFacets': {
        const { id, facets, expectedRevision } = (cmd as RelationSetFacetsCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateRelation(curr, { facets } as any);
        await this.repo.update(
          id,
          () => RelationSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'relation.facets.set', { id })];
      }

      case 'relation.mergeFacets': {
        const { id, patch, expectedRevision } = (cmd as RelationMergeFacetsCmd)
          .payload;
        const curr = await this.mustGet(id);
        const merged = { ...((curr.shape as any).facets ?? {}), ...patch };
        const next = updateRelation(curr, { facets: merged } as any);
        await this.repo.update(
          id,
          () => RelationSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'relation.facets.merged', { id })];
      }

      case 'relation.setSignature': {
        const { id, signature, expectedRevision } = (
          cmd as RelationSetSignatureCmd
        ).payload;
        const curr = await this.mustGet(id);
        const next = updateRelation(curr, {
          signature: signature === undefined ? (null as any) : signature,
        } as any);
        await this.repo.update(
          id,
          () => RelationSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'relation.signature.set', { id })];
      }

      case 'relation.mergeSignature': {
        const { id, patch, expectedRevision } = (
          cmd as RelationMergeSignatureCmd
        ).payload;
        const curr = await this.mustGet(id);
        const merged = { ...((curr.shape as any).signature ?? {}), ...patch };
        const next = updateRelation(curr, { signature: merged } as any);
        await this.repo.update(
          id,
          () => RelationSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'relation.signature.merged', { id })];
      }

      case 'relation.setEndpoints': {
        const { id, source, target } = (cmd as RelationSetEndpointsCmd).payload;
        const current = await this.mustGet(id);
        const src = EntityRef.parse(source);
        const tgt = EntityRef.parse(target);
        const next = updateRelation(current, { source: src, target: tgt });
        await this.repo.update(id, () => RelationSchema.parse(next));
        return [
          this.emit(base, 'relation.endpoints.set', {
            id,
            source: src,
            target: tgt,
          }),
        ];
      }

      case 'relation.setDirection': {
        const { id, direction } = (cmd as RelationSetDirectionCmd).payload;
        const current = await this.mustGet(id);
        const dir = RelationDirection.parse(direction);
        const next = updateRelation(current, { direction: dir });
        await this.repo.update(id, () => RelationSchema.parse(next));
        return [
          this.emit(base, 'relation.direction.set', { id, direction: dir }),
        ];
      }

      case 'relation.invert': {
        const { id } = (cmd as RelationInvertCmd).payload;
        const current = await this.mustGet(id);
        if (current.shape.direction === 'bidirectional') {
          return [
            this.emit(base, 'relation.inverted', {
              id,
              noop: true,
              direction: 'bidirectional',
            }),
          ];
        }
        const next = updateRelation(current, {
          source: current.shape.target,
          target: current.shape.source,
        });
        await this.repo.update(id, () => RelationSchema.parse(next));
        return [
          this.emit(base, 'relation.inverted', { id, direction: 'directed' }),
        ];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }
}
