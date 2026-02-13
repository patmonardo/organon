import type {
  AgentOmniscientContext,
  KnowledgeUnit,
  SpecificationRef,
} from '@organon/task';

import { AgentModel, type AgentGoal } from './agent-model';
import {
  buildSdslRuntime,
  type SdslRuntimeRequest,
  type SdslRuntimeOutput,
} from './specification-runtime';
import type { FormShape } from './types';

export type AgentLogicModelClientRequest = {
  runtime: SdslRuntimeRequest;
  graphId?: string;
  goal?: AgentGoal;
  knowledgeUnits?: KnowledgeUnit[];
  specifications?: SpecificationRef[];
  metadata?: Record<string, unknown>;
};

export type AgentLogicModelClientResult = {
  runtime: SdslRuntimeOutput;
  model: AgentModel<FormShape>;
  omniscientContext: AgentOmniscientContext;
};

function toModelValues(runtime: SdslRuntimeOutput): Record<string, unknown> {
  const values: Record<string, unknown> = {};

  for (const field of runtime.formShape.fields) {
    values[field.id] = undefined;
  }

  for (const fact of runtime.contextDocument.facts) {
    if (fact.id in values) {
      values[fact.id] = fact.value;
    }
  }

  return values;
}

function toOmniscientContext(
  runtime: SdslRuntimeOutput,
  input: AgentLogicModelClientRequest,
): AgentOmniscientContext {
  const nowIso = new Date().toISOString();
  const specId = runtime.compiled.specification.id;
  const graphId =
    input.graphId ?? runtime.compiled.kernel.graphStorePut.graphName;

  const propertyIds = runtime.compiled.designSurface.properties
    .map((property) => property.id)
    .filter((id): id is string => typeof id === 'string' && id.length > 0);

  const aspectIds = runtime.compiled.designSurface.aspects
    .map((aspect) => aspect.id)
    .filter((id): id is string => typeof id === 'string' && id.length > 0);

  const entityId = runtime.compiled.designSurface.entity.id;
  const entityIds =
    typeof entityId === 'string' && entityId.length > 0 ? [entityId] : [];

  const knowledgeUnits = input.knowledgeUnits ?? [];
  const focusTopics = Array.from(
    new Set(knowledgeUnits.flatMap((unit) => unit.tags ?? [])),
  );

  const specifications = input.specifications ?? [
    {
      id: specId,
      kind: 'sdsl',
      title: runtime.compiled.specification.title,
      version: 'v1',
    } satisfies SpecificationRef,
  ];

  return {
    id: `omniscient:${specId}`,
    graphId,
    timestamp: nowIso,
    factProjection: {
      source: 'fact-store',
      context: runtime.contextDocument,
      entityIds,
      propertyIds,
      aspectIds,
      metadata: {
        compiler: 'sdsl',
      },
    },
    knowledgeProjection: {
      source: 'knowledge-store',
      units: knowledgeUnits,
      focusTopics,
    },
    specifications,
    metadata: input.metadata,
  };
}

/**
 * Agent-first SDK surface for SDSL runtime orchestration.
 *
 * Agent calls this model client, model client calls logical specification runtime,
 * and returns both AgentModel and omniscient graph projections.
 */
export class AgentLogicModelClient {
  async synthesize(
    input: AgentLogicModelClientRequest,
  ): Promise<AgentLogicModelClientResult> {
    const runtime = await buildSdslRuntime(input.runtime);

    const model = AgentModel.fromLogicalProjection(
      runtime.modelBridge.agent.platonic,
      {
        id: runtime.modelBridge.agent.formShape.id,
        name: runtime.modelBridge.agent.formShape.name,
        title: runtime.modelBridge.agent.formShape.title,
        description: runtime.modelBridge.agent.formShape.description,
        values: toModelValues(runtime),
      },
    );

    const goal =
      input.goal ??
      (runtime.contextDocument.goal
        ? {
            id: runtime.contextDocument.goal.id,
            type: runtime.contextDocument.goal.type as AgentGoal['type'],
            description: runtime.contextDocument.goal.description,
            priority: 1,
          }
        : undefined);

    if (goal) {
      model.setGoal(goal);
    }

    const omniscientContext = toOmniscientContext(runtime, input);

    return {
      runtime,
      model,
      omniscientContext,
    };
  }
}
