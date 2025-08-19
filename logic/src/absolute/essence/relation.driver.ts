import BaseDriver from '../core/driver';
import type { ProcessorInputs } from '../core/contracts';
import { createWorld, type World } from '../../schema/world';
import * as active from '../../schema/active';
import type { Repository } from '../../repository/repo';
import { makeInMemoryRepository } from '../../repository/memory';
import { RelationSchema, type Relation } from '../../schema/relation';
import type { EventBus } from '../../form/triad/bus';
import { RelationEngine } from '../../form/relation/engine';

export class RelationDriver extends BaseDriver {
  constructor() {
    super('RelationDriver');
  }

  // Minimal world assembly for stage identification
  assemble(_input: ProcessorInputs): World {
    return createWorld({ type: 'system.World', name: 'Relations', horizon: { stage: 'relations' } });
  }

  // Active conversions
  toActiveRelation(input: unknown): active.ActiveRelation {
    return active.ActiveFactory.parseRelation(input);
  }

  toActiveRelationBatch(inputs: unknown[] = []): active.ActiveRelation[] {
    return inputs.map((i) => this.toActiveRelation(i));
  }

  // Engine bridge
  private createRelationEngine(repo?: Repository<Relation>, bus?: EventBus) {
    const r: Repository<Relation> =
      (repo as Repository<Relation> | undefined) ??
      (makeInMemoryRepository(RelationSchema as any) as unknown as Repository<Relation>);
    return new RelationEngine(r, bus);
  }

  async processActiveRelations(
    relations: active.ActiveRelation[] = [],
    opts?: { repo?: Repository<Relation>; bus?: EventBus; particulars?: any[]; context?: any },
  ) {
    const engine = this.createRelationEngine(opts?.repo, opts?.bus);
    const list = active.parseActiveRelations(relations);
    return engine.process(list, opts?.particulars ?? [], opts?.context);
  }

  async commitActiveRelations(
    relations: active.ActiveRelation[] = [],
    opts?: { repo?: Repository<Relation>; bus?: EventBus; particulars?: any[]; context?: any },
  ) {
    const engine = this.createRelationEngine(opts?.repo, opts?.bus);
    const list = active.parseActiveRelations(relations);
    const { actions, snapshot } = await engine.process(list, opts?.particulars ?? [], opts?.context);
    const commitResult = await engine.commit(actions, snapshot);
    return { actions, snapshot, commitResult } as const;
  }
}

export const DefaultRelationDriver = new RelationDriver();

export function toActiveRelation(input: unknown) {
  return DefaultRelationDriver.toActiveRelation(input);
}

export function toActiveRelationBatch(inputs: unknown[] = []) {
  return DefaultRelationDriver.toActiveRelationBatch(inputs);
}

export function processActiveRelations(
  relations: active.ActiveRelation[] = [],
  opts?: { repo?: Repository<Relation>; bus?: EventBus; particulars?: any[]; context?: any },
) {
  return DefaultRelationDriver.processActiveRelations(relations, opts);
}

export function commitActiveRelations(
  relations: active.ActiveRelation[] = [],
  opts?: { repo?: Repository<Relation>; bus?: EventBus; particulars?: any[]; context?: any },
) {
  return DefaultRelationDriver.commitActiveRelations(relations, opts);
}

export default DefaultRelationDriver;
