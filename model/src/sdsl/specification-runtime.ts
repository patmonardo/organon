import { z } from 'zod';

import {
  schema as logicSchema,
  SdslSpecificationRepository,
  defaultConnection,
  type SdslCompilerArtifacts,
} from '@organon/logic';

import {
  type FormShape,
  type FormField,
  type PlatonicFormProjection,
  type UnifiedModelBridge,
  UnifiedModelBridgeSchema,
} from './types';
import type { ContextDocument } from './agent-view';
import {
  AgentAdapter,
  type AgentOutputFormat,
  type FunctionCallOutput,
  type GraphQLOutput,
  type JSONLDOutput,
  type PromptOutput,
  type RDFOutput,
} from './agent-adapter';

export const SdslRuntimeRequestSchema = z.object({
  specification: logicSchema.SdslSpecificationSchema,
  compilerInput: logicSchema.SdslCompilerInputSchema,
  persistToNeo4j: z.boolean().default(false),
  agent: z
    .object({
      formats: z
        .array(
          z.enum(['prompt', 'function', 'graphql', 'rdf', 'jsonld', 'context']),
        )
        .default(['context', 'prompt', 'function']),
      functionName: z.string().min(1).default('runSdslSpec'),
      operationType: z.enum(['query', 'mutation']).default('query'),
    })
    .default({
      formats: ['context', 'prompt', 'function'],
      functionName: 'runSdslSpec',
      operationType: 'query',
    }),
});
export type SdslRuntimeRequest = z.infer<typeof SdslRuntimeRequestSchema>;

export const SdslDashboardBindingSchema = z.object({
  route: z.string().min(1),
  sections: z.array(z.string().min(1)),
  chartHints: z.array(z.string().min(1)),
});
export type SdslDashboardBinding = z.infer<typeof SdslDashboardBindingSchema>;

export type SdslRuntimeOutput = {
  compiled: SdslCompilerArtifacts;
  formShape: FormShape;
  modelBridge: UnifiedModelBridge;
  contextDocument: ContextDocument;
  dashboard: SdslDashboardBinding;
  agent: SdslRuntimeAgentOutputs;
  persisted?: {
    entityId: string;
    propertyCount: number;
    aspectCount: number;
  };
};

export type SdslRuntimeAgentOutputs = {
  context: ContextDocument;
  prompt?: PromptOutput;
  function?: FunctionCallOutput;
  graphql?: GraphQLOutput;
  rdf?: RDFOutput;
  jsonld?: JSONLDOutput;
};

function toAgentOutputs(
  context: ContextDocument,
  input: {
    formats: AgentOutputFormat[];
    functionName: string;
    operationType: 'query' | 'mutation';
  },
): SdslRuntimeAgentOutputs {
  const adapter = AgentAdapter.create();
  const formats = Array.from(new Set(input.formats));
  const outputs: SdslRuntimeAgentOutputs = {
    context,
  };

  for (const format of formats) {
    if (format === 'context') {
      outputs.context = context;
      continue;
    }
    if (format === 'prompt') {
      outputs.prompt = adapter.toPrompt(context);
      continue;
    }
    if (format === 'function') {
      outputs.function = adapter.toFunctionCall(context, input.functionName);
      continue;
    }
    if (format === 'graphql') {
      outputs.graphql = adapter.toGraphQL(context, {
        operationType: input.operationType,
        operationName: input.functionName,
      });
      continue;
    }
    if (format === 'rdf') {
      outputs.rdf = adapter.toRDF(context);
      continue;
    }
    if (format === 'jsonld') {
      outputs.jsonld = adapter.toJSONLD(context);
    }
  }

  return outputs;
}

function toPlatonicProjection(
  compiled: SdslCompilerArtifacts,
): PlatonicFormProjection {
  const entity = logicSchema.EntityShapeSchema.parse(
    compiled.designSurface.entity,
  );
  const properties = compiled.designSurface.properties.map((property) =>
    logicSchema.PropertyShapeSchema.parse(property),
  );
  const aspects = compiled.designSurface.aspects.map((aspect) =>
    logicSchema.AspectShapeSchema.parse(aspect),
  );

  return {
    entity,
    properties,
    aspects,
  };
}

function toUnifiedModelBridge(
  compiled: SdslCompilerArtifacts,
  formShape: FormShape,
): UnifiedModelBridge {
  return UnifiedModelBridgeSchema.parse({
    dataset: {
      specification: compiled.specification,
      dataframe: compiled.dataframe,
      kernel: compiled.kernel,
    },
    agent: {
      formShape,
      platonic: toPlatonicProjection(compiled),
    },
  });
}

function toFormField(
  feature: z.infer<typeof logicSchema.SdslFeatureSpecSchema>,
): FormField {
  return {
    id: feature.id,
    type: feature.kind ?? 'text',
    label: feature.label,
    required: false,
    disabled: false,
    placeholder: `Enter ${feature.label}`,
  };
}

function toFormShape(
  specification: z.infer<typeof logicSchema.SdslSpecificationSchema>,
): FormShape {
  return {
    id: `sdsl-form:${specification.id}`,
    name: specification.id,
    title: specification.title,
    description: specification.description,
    fields: specification.features.map(toFormField),
    meta: {
      classification: specification.classification,
      engine: specification.engine,
      modelCount: specification.models.length,
      featureCount: specification.features.length,
    },
  };
}

function toContextDocument(compiled: SdslCompilerArtifacts): ContextDocument {
  const nowIso = new Date().toISOString();

  return {
    id: `ctx-sdsl-${compiled.specification.id}`,
    timestamp: nowIso,
    schema: {
      id: compiled.specification.id,
      name: compiled.specification.title,
      description: compiled.specification.description,
      fieldCount: compiled.specification.features.length,
      requiredFields: [],
      optionalFields: compiled.specification.features.map((f) => f.id),
    },
    facts: [
      {
        id: 'classification.genus',
        label: 'Genus',
        type: 'text',
        value: compiled.specification.classification.genus,
      },
      {
        id: 'classification.species',
        label: 'Species',
        type: 'text',
        value: compiled.specification.classification.species,
      },
      {
        id: 'engine.logicalForm',
        label: 'Logical Engine',
        type: 'text',
        value: compiled.specification.engine.logicalForm,
      },
      {
        id: 'engine.mvc',
        label: 'MVC Engine',
        type: 'text',
        value: compiled.specification.engine.mvc,
      },
      {
        id: 'dataframe.stepCount',
        label: 'DataFrame Steps',
        type: 'number',
        value: compiled.dataframe.steps.length,
      },
    ],
    constraints: [
      {
        type: 'must',
        description:
          'SDSL specification is the source of truth for kernel, dataframe, and mvc artifacts.',
      },
    ],
    dependencies: compiled.specification.features
      .filter((feature) => !!feature.modelId)
      .map((feature) => ({
        from: feature.modelId as string,
        to: feature.id,
        type: 'model-feature',
      })),
    goal: {
      id: `goal-${compiled.specification.id}`,
      type: 'query',
      description: `Compile and run ${compiled.specification.title} across Kernel/DataFrame/MVC adapters`,
    },
  };
}

export async function buildSdslRuntime(
  requestInput: SdslRuntimeRequest,
): Promise<SdslRuntimeOutput> {
  const request = SdslRuntimeRequestSchema.parse(requestInput);

  if (request.persistToNeo4j) {
    const repository = new SdslSpecificationRepository(defaultConnection);
    const persistedResult = await repository.saveAndCompileSpecification(
      request.specification,
      request.compilerInput,
    );

    const dashboard = SdslDashboardBindingSchema.parse(
      persistedResult.compiled.mvc.dashboard,
    );
    const formShape = toFormShape(persistedResult.compiled.specification);
    const contextDocument = toContextDocument(persistedResult.compiled);

    const output: SdslRuntimeOutput = {
      compiled: persistedResult.compiled,
      formShape,
      modelBridge: toUnifiedModelBridge(persistedResult.compiled, formShape),
      contextDocument,
      dashboard,
      agent: toAgentOutputs(contextDocument, {
        formats: request.agent.formats,
        functionName: request.agent.functionName,
        operationType: request.agent.operationType,
      }),
      persisted: {
        entityId: persistedResult.persisted.entity.id,
        propertyCount: persistedResult.persisted.properties.length,
        aspectCount: persistedResult.persisted.aspects.length,
      },
    };

    return output;
  }

  const compiled = logicSchema.compileSdslSpecification(
    request.specification,
    request.compilerInput,
  );
  const formShape = toFormShape(compiled.specification);
  const dashboard = SdslDashboardBindingSchema.parse(compiled.mvc.dashboard);
  const contextDocument = toContextDocument(compiled);

  return {
    compiled,
    formShape,
    modelBridge: toUnifiedModelBridge(compiled, formShape),
    contextDocument,
    dashboard,
    agent: toAgentOutputs(contextDocument, {
      formats: request.agent.formats,
      functionName: request.agent.functionName,
      operationType: request.agent.operationType,
    }),
  };
}
