import { z } from 'zod';

import {
  SdslSpecificationSchema,
  specificationToDesignSurface,
  type SdslDesignSurfacePayload,
  type SdslSpecification,
} from './sdsl';
import {
  GdsApplicationFormKindSchema,
  GdsDatabaseIdSchema,
  GdsGraphNameSchema,
  GdsUserSchema,
} from './common';
import { GdsFormProgramSchema } from './program';

export const SdslCompilerInputSchema = z.object({
  user: z.object({
    username: z.string().min(1),
    isAdmin: z.boolean().optional().default(false),
  }),
  databaseId: z.string().min(1),
  graphName: z.string().min(1).optional(),
  outputGraphName: z.string().min(1).optional(),
});
export type SdslCompilerInput = z.infer<typeof SdslCompilerInputSchema>;

export const SdslDataFrameLoweringStepSchema = z.object({
  id: z.string().min(1),
  stage: z.enum(['input', 'encode', 'transform', 'decode', 'output']),
  operation: z.string().min(1),
  inputRef: z.string().min(1),
  outputRef: z.string().min(1),
  featureId: z.string().min(1).optional(),
  modelId: z.string().min(1).optional(),
});
export type SdslDataFrameLoweringStep = z.infer<
  typeof SdslDataFrameLoweringStepSchema
>;

export const SdslDataFramePlanSchema = z.object({
  engine: z.literal('polars'),
  datasetId: z.string().min(1),
  steps: z.array(SdslDataFrameLoweringStepSchema),
});
export type SdslDataFramePlan = z.infer<typeof SdslDataFramePlanSchema>;

const SdslGraphStorePutCallSchema = z.object({
  kind: GdsApplicationFormKindSchema.optional(),
  facade: z.literal('graph_store'),
  op: z.literal('put'),
  user: GdsUserSchema,
  databaseId: GdsDatabaseIdSchema,
  graphName: GdsGraphNameSchema,
  snapshot: z
    .object({
      nodes: z.array(z.number().int()).min(1),
      relationships: z
        .array(
          z.object({
            type: z.string().min(1),
            source: z.number().int(),
            target: z.number().int(),
            properties: z.record(z.string(), z.unknown()).optional(),
          }),
        )
        .optional(),
      nodeProperties: z.record(z.string(), z.array(z.unknown())).optional(),
    })
    .passthrough(),
});

const SdslFormEvalEvaluateCallSchema = z.object({
  kind: GdsApplicationFormKindSchema.optional(),
  facade: z.literal('form_eval'),
  op: z.literal('evaluate'),
  user: GdsUserSchema,
  databaseId: GdsDatabaseIdSchema,
  graphName: GdsGraphNameSchema,
  outputGraphName: GdsGraphNameSchema.optional(),
  program: GdsFormProgramSchema,
  artifacts: z.record(z.string(), z.unknown()).optional(),
});

export const SdslKernelArtifactsSchema = z.object({
  graphStorePut: SdslGraphStorePutCallSchema,
  formEvalEvaluate: SdslFormEvalEvaluateCallSchema,
});
export type SdslKernelArtifacts = z.infer<typeof SdslKernelArtifactsSchema>;

export const SdslMvcArtifactsSchema = z.object({
  adapter: z.string().min(1),
  dashboard: z.object({
    route: z.string().min(1),
    sections: z.array(z.string().min(1)),
    chartHints: z.array(z.string().min(1)),
  }),
});
export type SdslMvcArtifacts = z.infer<typeof SdslMvcArtifactsSchema>;

export const SdslCompilerArtifactsSchema = z.object({
  specification: SdslSpecificationSchema,
  designSurface: z.object({
    entity: z.record(z.string(), z.unknown()),
    properties: z.array(z.record(z.string(), z.unknown())),
    aspects: z.array(z.record(z.string(), z.unknown())),
  }),
  dataframe: SdslDataFramePlanSchema,
  kernel: SdslKernelArtifactsSchema,
  mvc: SdslMvcArtifactsSchema,
});
export type SdslCompilerArtifacts = z.infer<typeof SdslCompilerArtifactsSchema>;

function buildDataFramePlan(spec: SdslSpecification): SdslDataFramePlan {
  const steps: SdslDataFrameLoweringStep[] = [];

  for (const feature of spec.features) {
    const prefix = `feature:${feature.id}`;
    steps.push({
      id: `${prefix}:input`,
      stage: 'input',
      operation: 'text.input',
      inputRef: `${feature.modelId ?? 'global'}.source`,
      outputRef: `${prefix}.input`,
      featureId: feature.id,
      modelId: feature.modelId,
    });
    steps.push({
      id: `${prefix}:encode`,
      stage: 'encode',
      operation: 'text.encode.lowercase',
      inputRef: `${prefix}.input`,
      outputRef: `${prefix}.encoded`,
      featureId: feature.id,
      modelId: feature.modelId,
    });
    steps.push({
      id: `${prefix}:transform`,
      stage: 'transform',
      operation: 'text.transform.tokenize',
      inputRef: `${prefix}.encoded`,
      outputRef: `${prefix}.tokens`,
      featureId: feature.id,
      modelId: feature.modelId,
    });
    steps.push({
      id: `${prefix}:decode`,
      stage: 'decode',
      operation: 'text.decode.token_count',
      inputRef: `${prefix}.tokens`,
      outputRef: `${prefix}.decoded`,
      featureId: feature.id,
      modelId: feature.modelId,
    });
    steps.push({
      id: `${prefix}:output`,
      stage: 'output',
      operation: 'text.output',
      inputRef: `${prefix}.decoded`,
      outputRef: `${prefix}.output`,
      featureId: feature.id,
      modelId: feature.modelId,
    });
  }

  return SdslDataFramePlanSchema.parse({
    engine: 'polars',
    datasetId: spec.id,
    steps,
  });
}

function buildKernelArtifacts(
  spec: SdslSpecification,
  input: SdslCompilerInput,
): SdslKernelArtifacts {
  const graphName = input.graphName ?? `sdsl-spec-${spec.id}`;
  const outputGraphName =
    input.outputGraphName ?? `sdsl-spec-${spec.id}-compiled`;

  const modelNodeOffset = 1000;
  const featureNodeOffset = 2000;

  const modelNodes = spec.models.map((_, index) => modelNodeOffset + index);
  const featureNodes = spec.features.map(
    (_, index) => featureNodeOffset + index,
  );
  const nodes = [0, ...modelNodes, ...featureNodes];

  const modelNodeById = new Map(
    spec.models.map((model, index) => [model.id, modelNodeOffset + index]),
  );

  const relationships = [
    ...modelNodes.map((modelNodeId) => ({
      type: 'HAS_MODEL',
      source: 0,
      target: modelNodeId,
      properties: { weight: 1 },
    })),
    ...featureNodes.map((featureNodeId) => ({
      type: 'HAS_FEATURE',
      source: 0,
      target: featureNodeId,
      properties: { weight: 1 },
    })),
    ...spec.features
      .map((feature, index) => {
        const targetModel = feature.modelId
          ? modelNodeById.get(feature.modelId)
          : undefined;
        if (targetModel === undefined) return null;
        return {
          type: 'MODEL_FEATURE',
          source: targetModel,
          target: featureNodeOffset + index,
          properties: { weight: 1 },
        };
      })
      .filter(Boolean),
  ] as Array<{
    type: string;
    source: number;
    target: number;
    properties?: Record<string, unknown>;
  }>;

  const graphStorePut = SdslGraphStorePutCallSchema.parse({
    kind: 'ApplicationForm',
    facade: 'graph_store',
    op: 'put',
    user: input.user,
    databaseId: input.databaseId,
    graphName,
    snapshot: {
      nodes,
      relationships,
      nodeProperties: {
        specificationId: nodes.map(() => spec.id),
      },
    },
  });

  const formEvalEvaluate = SdslFormEvalEvaluateCallSchema.parse({
    kind: 'ApplicationForm',
    facade: 'form_eval',
    op: 'evaluate',
    user: input.user,
    databaseId: input.databaseId,
    graphName,
    outputGraphName,
    program: {
      morph: {
        patterns: [
          'spec.validate',
          'models.compile',
          'features.compile',
          'aspects.project',
        ],
        steps: [
          { kind: 'form', op: 'spec.validate' },
          { kind: 'form', op: 'models.compile' },
          { kind: 'form', op: 'features.compile' },
          { kind: 'judge', moment: 'reflection' },
        ],
      },
      context: {
        runtime_strategy: spec.engine.logicalForm,
        conditions: ['specification-driven', 'toolchain-first'],
      },
    },
    artifacts: {
      specificationId: spec.id,
      engine: spec.engine,
    },
  });

  return SdslKernelArtifactsSchema.parse({
    graphStorePut,
    formEvalEvaluate,
  });
}

function buildMvcArtifacts(spec: SdslSpecification): SdslMvcArtifacts {
  const sectionLabels = [
    ...spec.models.map((model) => `model:${model.label}`),
    ...spec.features.map((feature) => `feature:${feature.label}`),
  ];

  return SdslMvcArtifactsSchema.parse({
    adapter: spec.engine.mvc,
    dashboard: {
      route: `/dashboard/sdsl/${spec.id}`,
      sections: sectionLabels,
      chartHints: ['recharts:feature-coverage', 'd3:model-feature-graph'],
    },
  });
}

export function compileSdslSpecification(
  specificationInput: SdslSpecification,
  compilerInput: SdslCompilerInput,
): SdslCompilerArtifacts {
  const specification = SdslSpecificationSchema.parse(specificationInput);
  const input = SdslCompilerInputSchema.parse(compilerInput);

  const designSurface: SdslDesignSurfacePayload =
    specificationToDesignSurface(specification);

  const artifacts: SdslCompilerArtifacts = {
    specification,
    designSurface: {
      entity: designSurface.entity as Record<string, unknown>,
      properties: designSurface.properties as Array<Record<string, unknown>>,
      aspects: designSurface.aspects as Array<Record<string, unknown>>,
    },
    dataframe: buildDataFramePlan(specification),
    kernel: buildKernelArtifacts(specification, input),
    mvc: buildMvcArtifacts(specification),
  };

  return SdslCompilerArtifactsSchema.parse(artifacts);
}
