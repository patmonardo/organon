import { z } from 'zod';

import {
  SdslSpecificationSchema,
  specificationToDesignSurface,
  type SdslDesignSurfacePayload,
  type SdslSpecification,
  type SdslUdfSpec,
  type SdslUdtSpec,
} from '../../../schema/sdsl';
import {
  GdsApplicationFormKindSchema,
  GdsDatabaseIdSchema,
  GdsGraphNameSchema,
  GdsUserSchema,
} from '../../../schema/common';
import { GdsFormProgramSchema } from '../../../schema/program';

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

export const SdslRootDataFrameSchema = z.object({
  id: z.string().min(1),
  source: z.literal('gdsl-specification'),
  producedBy: z.literal('specification-compiler'),
  datasetSdkExecuted: z.literal(false),
});
export type SdslRootDataFrame = z.infer<typeof SdslRootDataFrameSchema>;

export const SdslSubstrateMappingRowSchema = z.object({
  specificationId: z.string().min(1),
  modelId: z.string().min(1).optional(),
  featureId: z.string().min(1),
  featureStructId: z.string().min(1),
  udtId: z.string().min(1),
  udfIds: z.array(z.string().min(1)),
  dataframeDefinitionRef: z.string().min(1),
  datasetDefinitionRef: z.string().min(1),
  origin: z.literal('sdsl-specification'),
});
export type SdslSubstrateMappingRow = z.infer<
  typeof SdslSubstrateMappingRowSchema
>;

export const SdslSubstrateMappingSchema = z.object({
  rootDataFrame: SdslRootDataFrameSchema,
  mappings: z.array(SdslSubstrateMappingRowSchema),
});
export type SdslSubstrateMapping = z.infer<typeof SdslSubstrateMappingSchema>;

export const SdslGeneratedUdtSchema = z.object({
  id: z.string().min(1),
  label: z.string().min(1),
  baseType: z.enum(['string', 'number', 'boolean', 'datetime', 'json']),
  source: z.enum(['spec-declared', 'spec-inferred']),
});
export type SdslGeneratedUdt = z.infer<typeof SdslGeneratedUdtSchema>;

export const SdslGeneratedUdfSchema = z.object({
  id: z.string().min(1),
  label: z.string().min(1),
  inputUdtId: z.string().min(1),
  outputType: z.string().min(1),
  implementationRef: z.string().min(1),
  semantics: z.enum(['validate', 'normalize', 'enrich', 'project']),
  source: z.enum(['spec-declared', 'spec-inferred']),
});
export type SdslGeneratedUdf = z.infer<typeof SdslGeneratedUdfSchema>;

export const SdslDataFrameFieldDefinitionSchema = z.object({
  featureId: z.string().min(1),
  featureStructId: z.string().min(1),
  udtId: z.string().min(1),
  udfIds: z.array(z.string().min(1)),
  ref: z.string().min(1),
});
export type SdslDataFrameFieldDefinition = z.infer<
  typeof SdslDataFrameFieldDefinitionSchema
>;

export const SdslDatasetFieldDefinitionSchema = z.object({
  featureId: z.string().min(1),
  featureStructId: z.string().min(1),
  udtId: z.string().min(1),
  udfIds: z.array(z.string().min(1)),
  ref: z.string().min(1),
});
export type SdslDatasetFieldDefinition = z.infer<
  typeof SdslDatasetFieldDefinitionSchema
>;

export const SdslSchemaGenerationSchema = z.object({
  strategy: z.literal('model-to-dataframe-schema-generator'),
  target: z.literal('gdsl-transcendental-logic'),
  rootDataFrameProducedByDatasetSdk: z.literal(false),
  generatedUdts: z.array(SdslGeneratedUdtSchema),
  generatedUdfs: z.array(SdslGeneratedUdfSchema),
  dataframeDefinitions: z.array(SdslDataFrameFieldDefinitionSchema),
  datasetDefinitions: z.array(SdslDatasetFieldDefinitionSchema),
});
export type SdslSchemaGeneration = z.infer<typeof SdslSchemaGenerationSchema>;

export const SdslNlpProcessorCapabilitySchema = z.object({
  id: z.string().min(1),
  operation: z.enum([
    'tokenize',
    'embed',
    'extract',
    'classify',
    'normalize',
    'validate',
    'enrich',
    'project',
  ]),
  implementationRef: z.string().min(1),
  udfId: z.string().min(1).optional(),
});
export type SdslNlpProcessorCapability = z.infer<
  typeof SdslNlpProcessorCapabilitySchema
>;

export const SdslDatasetSdkGenerationSchema = z.object({
  role: z.literal('sdsl-sdk-generator'),
  language: z.literal('typescript'),
  packageRef: z.string().min(1),
  factStoreTarget: z.literal('sdsl-factstore'),
  schemaMappingsRef: z.literal('substrateMapping'),
  dataAccess: z.object({
    polars: z.object({
      postgres: z.literal('partial-direct'),
      filesystem: z.literal('partial-direct'),
    }),
    kernel: z.object({
      neo4j: z.literal('direct'),
      postgres: z.literal('direct'),
      filesystem: z.literal('direct'),
    }),
  }),
  dragonSeedAugmentation: z.object({
    enabled: z.literal(true),
    sdkRef: z.string().min(1),
    analyses: z.array(z.string().min(1)).min(1),
  }),
  agentMonitoring: z.object({
    enabled: z.literal(true),
    scope: z.literal('factstore-structure-observability'),
    channels: z.array(z.string().min(1)).min(1),
  }),
  nlpProcessor: z.object({
    mode: z.literal('nlp-driven'),
    capabilities: z.array(SdslNlpProcessorCapabilitySchema),
  }),
});
export type SdslDatasetSdkGeneration = z.infer<
  typeof SdslDatasetSdkGenerationSchema
>;

export const OntologyImageModelRowSchema = z.object({
  modelId: z.string().min(1),
  label: z.string().min(1),
  kind: z.string().min(1),
  ontologyIds: z.array(z.string().min(1)),
});
export type OntologyImageModelRow = z.infer<typeof OntologyImageModelRowSchema>;

export const OntologyImageFeatureRowSchema = z.object({
  featureId: z.string().min(1),
  modelId: z.string().min(1).optional(),
  label: z.string().min(1),
  kind: z.string().min(1),
  ontologyIds: z.array(z.string().min(1)),
});
export type OntologyImageFeatureRow = z.infer<
  typeof OntologyImageFeatureRowSchema
>;

export const OntologyImageConstraintRowSchema = z.object({
  ontologyId: z.string().min(1),
  constraintId: z.string().min(1),
  language: z.enum(['shacl', 'owl']),
  text: z.string().min(1),
});
export type OntologyImageConstraintRow = z.infer<
  typeof OntologyImageConstraintRowSchema
>;

export const OntologyImageQueryRowSchema = z.object({
  ontologyId: z.string().min(1),
  queryId: z.string().min(1),
  language: z.enum(['sparql', 'cypher']),
  text: z.string().min(1),
});
export type OntologyImageQueryRow = z.infer<typeof OntologyImageQueryRowSchema>;

export const OntologyImageProvenanceRowSchema = z.object({
  source: z.literal('gdsl/sdsl'),
  specificationId: z.string().min(1),
  runtimeMode: z.literal('transcendental-logic'),
  substrate: z.literal('dataframe/dataset'),
  generatedAtUnixMs: z.number().int().nonnegative(),
});
export type OntologyImageProvenanceRow = z.infer<
  typeof OntologyImageProvenanceRowSchema
>;

export const OntologyDataFrameImageSchema = z.object({
  imageId: z.string().min(1),
  engine: z.literal('polars'),
  tables: z.object({
    models: z.array(OntologyImageModelRowSchema),
    features: z.array(OntologyImageFeatureRowSchema),
    constraints: z.array(OntologyImageConstraintRowSchema),
    queries: z.array(OntologyImageQueryRowSchema),
    provenance: z.array(OntologyImageProvenanceRowSchema),
  }),
});
export type OntologyDataFrameImage = z.infer<
  typeof OntologyDataFrameImageSchema
>;

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
    .catchall(z.unknown()),
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

export const FormCatalogContractSchema = z.object({
  backend: z.literal('neo4j'),
  listRef: z.string().min(1),
  finiteCatalog: z.literal(true),
  specificationIds: z.array(z.string().min(1)),
});
export type FormCatalogContract = z.infer<typeof FormCatalogContractSchema>;

export const FactStoreContractSchema = z.object({
  graphBackend: z.literal('neo4j'),
  graphRef: z.string().min(1),
  groundsBackend: z.literal('postgres'),
  groundsRef: z.string().min(1),
  groundsPurpose: z.literal('fact-grounds-support'),
  groundsDomain: z.literal('grounds-and-conditions'),
  unboundedFacticity: z.literal(true),
  prismaTemplate: z.object({
    templateRef: z.string().min(1),
    metamodelRef: z.string().min(1),
    scope: z.literal('single-factstore-metamodel'),
  }),
  postgresRole: z.literal('grounds-and-conditions-maintenance'),
  filesystemGrounds: z.object({
    enabled: z.literal(true),
    backend: z.literal('filesystem'),
    ref: z.string().min(1),
    domains: z
      .array(z.enum(['image', 'audio', 'video', 'binary', 'document']))
      .min(1),
  }),
  kernelCache: z.object({
    mode: z.literal('polyglot-dataset-cache'),
    backends: z.array(z.string().min(1)).min(1),
  }),
  datasetAugmentation: z.object({
    dragonSeedSdkRef: z.string().min(1),
    profile: z.literal('sdsl-dataset-analyses'),
  }),
  dataAccess: z.object({
    polars: z.object({
      postgres: z.literal('partial-direct'),
      filesystem: z.literal('partial-direct'),
    }),
    kernel: z.object({
      neo4j: z.literal('direct'),
      postgres: z.literal('direct'),
      filesystem: z.literal('direct'),
    }),
  }),
  agentMonitoring: z.object({
    enabled: z.literal(true),
    scope: z.literal('factstore-structure-observability'),
    channels: z.array(z.string().min(1)).min(1),
  }),
});
export type FactStoreContract = z.infer<typeof FactStoreContractSchema>;

export const KnowledgeStoreContractSchema = z.object({
  targetRef: z.string().min(1),
  compileMode: z.literal('transcendental-logic'),
  sourceRefs: z.object({
    formCatalogRef: z.string().min(1),
    factGraphRef: z.string().min(1),
    factGroundsRef: z.string().min(1),
  }),
});
export type KnowledgeStoreContract = z.infer<
  typeof KnowledgeStoreContractSchema
>;

export const ThreeStoreContractSchema = z.object({
  formCatalog: FormCatalogContractSchema,
  factStore: FactStoreContractSchema,
  knowledgeStore: KnowledgeStoreContractSchema,
});
export type ThreeStoreContract = z.infer<typeof ThreeStoreContractSchema>;

export const EpistemicProcessorContractSchema = z.object({
  runtime: z.literal('ts-agent-logic'),
  processor: z.literal('reflective-form'),
  mode: z.literal('epistemic'),
  authority: z.literal('sdsl/zod'),
});
export type EpistemicProcessorContract = z.infer<
  typeof EpistemicProcessorContractSchema
>;

export const TranscendentalKernelContractSchema = z.object({
  runtime: z.literal('gds-rust-kernel'),
  processor: z.literal('program-form-evaluate-apply-print'),
  mode: z.literal('transcendental-logic'),
  role: z.literal('cache'),
});
export type TranscendentalKernelContract = z.infer<
  typeof TranscendentalKernelContractSchema
>;

export const BoundaryHandoffContractSchema = z.object({
  substrate: z.literal('cypher-driven'),
  invariants: z.array(z.string().min(1)).min(1),
  proofObligations: z.array(z.string().min(1)).min(1),
});
export type BoundaryHandoffContract = z.infer<
  typeof BoundaryHandoffContractSchema
>;

export const ReflectiveBoundaryContractSchema = z.object({
  epistemicProcessor: EpistemicProcessorContractSchema,
  transcendentalKernel: TranscendentalKernelContractSchema,
  handoff: BoundaryHandoffContractSchema,
});
export type ReflectiveBoundaryContract = z.infer<
  typeof ReflectiveBoundaryContractSchema
>;

export const SdslCompilerArtifactsSchema = z.object({
  specification: SdslSpecificationSchema,
  designSurface: z.object({
    entity: z.record(z.string(), z.unknown()),
    properties: z.array(z.record(z.string(), z.unknown())),
    aspects: z.array(z.record(z.string(), z.unknown())),
  }),
  dataframe: SdslDataFramePlanSchema,
  substrateMapping: SdslSubstrateMappingSchema,
  schemaGeneration: SdslSchemaGenerationSchema,
  datasetSdkGeneration: SdslDatasetSdkGenerationSchema,
  ontologyImage: OntologyDataFrameImageSchema,
  storeContract: ThreeStoreContractSchema,
  kernel: SdslKernelArtifactsSchema,
  mvc: SdslMvcArtifactsSchema,
});
export type SdslCompilerArtifacts = z.infer<typeof SdslCompilerArtifactsSchema>;

function buildThreeStoreContract(
  spec: SdslSpecification,
  input: SdslCompilerInput,
): ThreeStoreContract {
  const formCatalogRef = `neo4j://formdb/specifications/${spec.id}`;
  const factGraphRef = `neo4j://factstore/${input.graphName ?? `sdsl-spec-${spec.id}`}`;
  const factGroundsRef = `postgres://fact_grounds/${spec.id}`;
  const targetRef =
    input.outputGraphName ?? `sdsl-spec-${spec.id}-knowledge-store`;
  const compileMode = 'transcendental-logic';

  return ThreeStoreContractSchema.parse({
    formCatalog: {
      backend: 'neo4j',
      listRef: formCatalogRef,
      finiteCatalog: true,
      specificationIds: [spec.id],
    },
    factStore: {
      graphBackend: 'neo4j',
      graphRef: factGraphRef,
      groundsBackend: 'postgres',
      groundsRef: factGroundsRef,
      groundsPurpose: 'fact-grounds-support',
      groundsDomain: 'grounds-and-conditions',
      unboundedFacticity: true,
      prismaTemplate: {
        templateRef: '@organon/factstore-prisma-template',
        metamodelRef: 'prisma.metamodel.factstore.v1',
        scope: 'single-factstore-metamodel',
      },
      postgresRole: 'grounds-and-conditions-maintenance',
      filesystemGrounds: {
        enabled: true,
        backend: 'filesystem',
        ref: `file://fact_grounds/${spec.id}`,
        domains: ['image', 'audio', 'video', 'binary', 'document'],
      },
      kernelCache: {
        mode: 'polyglot-dataset-cache',
        backends: ['polars', 'duckdb', 'postgres', 'arrow'],
      },
      datasetAugmentation: {
        dragonSeedSdkRef: '@organon/dragonseed-sdsl-sdk',
        profile: 'sdsl-dataset-analyses',
      },
      dataAccess: {
        polars: {
          postgres: 'partial-direct',
          filesystem: 'partial-direct',
        },
        kernel: {
          neo4j: 'direct',
          postgres: 'direct',
          filesystem: 'direct',
        },
      },
      agentMonitoring: {
        enabled: true,
        scope: 'factstore-structure-observability',
        channels: ['agent-runtime', 'dataset-health', 'cache-health'],
      },
    },
    knowledgeStore: {
      targetRef,
      compileMode,
      sourceRefs: {
        formCatalogRef,
        factGraphRef,
        factGroundsRef,
      },
    },
  });
}

function buildOntologyDataFrameImage(
  spec: SdslSpecification,
): OntologyDataFrameImage {
  const ontologyIds = spec.ontologies.map((ontology) => ontology.id);
  const runtimeMode = 'transcendental-logic';
  const generatedAtUnixMs = Date.now();

  return OntologyDataFrameImageSchema.parse({
    imageId: `ontology-image:${spec.id}`,
    engine: 'polars',
    tables: {
      models: spec.models.map((model) => ({
        modelId: model.id,
        label: model.label,
        kind: model.kind ?? 'model',
        ontologyIds,
      })),
      features: spec.features.map((feature) => ({
        featureId: feature.id,
        modelId: feature.modelId,
        label: feature.label,
        kind: feature.kind ?? 'feature',
        ontologyIds,
      })),
      constraints: spec.ontologies.flatMap((ontology) =>
        ontology.constraints.map((constraint) => ({
          ontologyId: ontology.id,
          constraintId: constraint.id,
          language: constraint.language,
          text: constraint.text,
        })),
      ),
      queries: spec.ontologies.flatMap((ontology) =>
        ontology.queries.map((query) => ({
          ontologyId: ontology.id,
          queryId: query.id,
          language: query.language,
          text: query.text,
        })),
      ),
      provenance: [
        {
          source: 'gdsl/sdsl',
          specificationId: spec.id,
          runtimeMode,
          substrate: 'dataframe/dataset',
          generatedAtUnixMs,
        },
      ],
    },
  });
}

function buildReflectiveBoundaryContract(): ReflectiveBoundaryContract {
  return ReflectiveBoundaryContractSchema.parse({
    epistemicProcessor: {
      runtime: 'ts-agent-logic',
      processor: 'reflective-form',
      mode: 'epistemic',
      authority: 'sdsl/zod',
    },
    transcendentalKernel: {
      runtime: 'gds-rust-kernel',
      processor: 'program-form-evaluate-apply-print',
      mode: 'transcendental-logic',
      role: 'cache',
    },
    handoff: {
      substrate: 'cypher-driven',
      invariants: [
        'program-features-precede-kernel-compilation',
        'specification-bindings-are-explicit',
        'graph-refs-resolve-via-store-contract',
        'entity-property-aspect-encodes-thing-world-law-essential-relations',
      ],
      proofObligations: [
        'artifact-hooks-validated',
        'program-form-print-materialized',
      ],
    },
  });
}

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

function buildSubstrateMapping(spec: SdslSpecification): SdslSubstrateMapping {
  const schemaGeneration = buildSchemaGeneration(spec);

  return SdslSubstrateMappingSchema.parse({
    rootDataFrame: {
      id: `gdsl-root-dataframe:${spec.id}`,
      source: 'gdsl-specification',
      producedBy: 'specification-compiler',
      datasetSdkExecuted: false,
    },
    mappings: schemaGeneration.dataframeDefinitions.map((definition) => {
      const datasetDefinition = schemaGeneration.datasetDefinitions.find(
        (entry) => entry.featureId === definition.featureId,
      );
      return {
        specificationId: spec.id,
        modelId: spec.features.find(
          (feature) => feature.id === definition.featureId,
        )?.modelId,
        featureId: definition.featureId,
        featureStructId: definition.featureStructId,
        udtId: definition.udtId,
        udfIds: definition.udfIds,
        dataframeDefinitionRef: definition.ref,
        datasetDefinitionRef: datasetDefinition?.ref ?? definition.ref,
        origin: 'sdsl-specification',
      };
    }),
  });
}

function isEmailFeatureLabel(value: string): boolean {
  return value.toLowerCase().includes('email');
}

function normalizeDeclaredUdt(udt: SdslUdtSpec): SdslGeneratedUdt {
  return {
    id: udt.id,
    label: udt.label,
    baseType: udt.baseType,
    source: 'spec-declared',
  };
}

function normalizeDeclaredUdf(udf: SdslUdfSpec): SdslGeneratedUdf {
  return {
    id: udf.id,
    label: udf.label,
    inputUdtId: udf.inputUdtId,
    outputType: udf.outputType,
    implementationRef: udf.implementationRef,
    semantics: udf.semantics,
    source: 'spec-declared',
  };
}

function buildSchemaGeneration(spec: SdslSpecification): SdslSchemaGeneration {
  const generatedUdts: SdslGeneratedUdt[] = spec.udts.map(normalizeDeclaredUdt);
  const generatedUdfs: SdslGeneratedUdf[] = spec.udfs.map(normalizeDeclaredUdf);

  if (
    spec.features.length > 0 &&
    !generatedUdts.some((udt) => udt.id === 'udt.string')
  ) {
    generatedUdts.push({
      id: 'udt.string',
      label: 'String',
      baseType: 'string',
      source: 'spec-inferred',
    });
  }

  const hasEmailFeature = spec.features.some(
    (feature) =>
      isEmailFeatureLabel(feature.id) || isEmailFeatureLabel(feature.label),
  );

  const emailUdtId = 'udt.email';
  if (hasEmailFeature && !generatedUdts.some((udt) => udt.id === emailUdtId)) {
    generatedUdts.push({
      id: emailUdtId,
      label: 'Email',
      baseType: 'string',
      source: 'spec-inferred',
    });
  }

  if (
    hasEmailFeature &&
    !generatedUdfs.some((udf) => udf.id === 'udf.email.normalize')
  ) {
    generatedUdfs.push({
      id: 'udf.email.normalize',
      label: 'Normalize Email',
      inputUdtId: emailUdtId,
      outputType: 'string',
      implementationRef: 'udf://email/normalize',
      semantics: 'normalize',
      source: 'spec-inferred',
    });
  }

  if (
    hasEmailFeature &&
    !generatedUdfs.some((udf) => udf.id === 'udf.email.validate')
  ) {
    generatedUdfs.push({
      id: 'udf.email.validate',
      label: 'Validate Email',
      inputUdtId: emailUdtId,
      outputType: 'boolean',
      implementationRef: 'udf://email/validate',
      semantics: 'validate',
      source: 'spec-inferred',
    });
  }

  const dataframeDefinitions: SdslDataFrameFieldDefinition[] =
    spec.features.map((feature) => {
      const modelScope = feature.modelId ?? 'global';
      const fallbackUdtId =
        isEmailFeatureLabel(feature.id) || isEmailFeatureLabel(feature.label)
          ? emailUdtId
          : 'udt.string';
      const explicitUdt = spec.udts.find((udt) => udt.featureId === feature.id);
      const udtId = explicitUdt?.id ?? fallbackUdtId;
      const udfIds = generatedUdfs
        .filter((udf) => udf.inputUdtId === udtId)
        .map((udf) => udf.id);
      return {
        featureId: feature.id,
        featureStructId: `feature-struct:${feature.id}`,
        udtId,
        udfIds,
        ref: `dataframe:def:${spec.id}:${modelScope}:${feature.id}`,
      };
    });

  const datasetDefinitions: SdslDatasetFieldDefinition[] =
    dataframeDefinitions.map((definition) => ({
      featureId: definition.featureId,
      featureStructId: definition.featureStructId,
      udtId: definition.udtId,
      udfIds: definition.udfIds,
      ref: definition.ref.replace('dataframe:def:', 'dataset:def:'),
    }));

  return SdslSchemaGenerationSchema.parse({
    strategy: 'model-to-dataframe-schema-generator',
    target: 'gdsl-transcendental-logic',
    rootDataFrameProducedByDatasetSdk: false,
    generatedUdts,
    generatedUdfs,
    dataframeDefinitions,
    datasetDefinitions,
  });
}

function buildDatasetSdkGeneration(
  spec: SdslSpecification,
  schemaGeneration: SdslSchemaGeneration,
): SdslDatasetSdkGeneration {
  const udfCapabilities = schemaGeneration.generatedUdfs.map((udf) => ({
    id: `nlp.capability:${udf.id}`,
    operation: udf.semantics,
    implementationRef: udf.implementationRef,
    udfId: udf.id,
  }));

  const baselineCapabilities: SdslNlpProcessorCapability[] = [
    {
      id: 'nlp.capability:tokenize',
      operation: 'tokenize',
      implementationRef: 'nlp://processor/tokenize',
    },
    {
      id: 'nlp.capability:embed',
      operation: 'embed',
      implementationRef: 'nlp://processor/embed',
    },
    {
      id: 'nlp.capability:extract',
      operation: 'extract',
      implementationRef: 'nlp://processor/extract',
    },
    {
      id: 'nlp.capability:classify',
      operation: 'classify',
      implementationRef: 'nlp://processor/classify',
    },
  ];

  const capabilitiesById = new Map<string, SdslNlpProcessorCapability>();
  for (const capability of [...baselineCapabilities, ...udfCapabilities]) {
    capabilitiesById.set(capability.id, capability);
  }

  return SdslDatasetSdkGenerationSchema.parse({
    role: 'sdsl-sdk-generator',
    language: 'typescript',
    packageRef: `@organon/sdsl-sdk-${spec.id}`,
    factStoreTarget: 'sdsl-factstore',
    schemaMappingsRef: 'substrateMapping',
    dataAccess: {
      polars: {
        postgres: 'partial-direct',
        filesystem: 'partial-direct',
      },
      kernel: {
        neo4j: 'direct',
        postgres: 'direct',
        filesystem: 'direct',
      },
    },
    dragonSeedAugmentation: {
      enabled: true,
      sdkRef: '@organon/dragonseed-sdsl-sdk',
      analyses: [
        'entity-extraction',
        'relation-extraction',
        'semantic-classification',
        'feature-enrichment',
      ],
    },
    agentMonitoring: {
      enabled: true,
      scope: 'factstore-structure-observability',
      channels: ['agent-runtime', 'dataset-health', 'cache-health'],
    },
    nlpProcessor: {
      mode: 'nlp-driven',
      capabilities: Array.from(capabilitiesById.values()),
    },
  });
}

function buildKernelArtifacts(
  spec: SdslSpecification,
  input: SdslCompilerInput,
): SdslKernelArtifacts {
  const storeContract = buildThreeStoreContract(spec, input);
  const schemaGeneration = buildSchemaGeneration(spec);
  const datasetSdkGeneration = buildDatasetSdkGeneration(
    spec,
    schemaGeneration,
  );
  const ontologyImage = buildOntologyDataFrameImage(spec);
  const substrateMapping = buildSubstrateMapping(spec);
  const boundaryContract = buildReflectiveBoundaryContract();
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
        conditions: [
          'specification-driven',
          'toolchain-first',
          'given-form-recognition',
          'ontological-program-feature',
        ],
      },
      applicationForms: spec.models.map((model) => {
        const modelFeatures = spec.features.filter(
          (feature) => feature.modelId === model.id,
        );
        return {
          name: model.id,
          domain: 'ontology-runtime',
          features: modelFeatures.map((feature) => feature.id),
          patterns: ['spec.validate', 'features.compile', 'aspects.project'],
          specifications: {
            modelKind: model.kind ?? 'model',
            ontologyCount: String(spec.ontologies.length),
          },
        };
      }),
      selectedForms: spec.models.map((model) => model.id),
    },
    artifacts: {
      specificationId: spec.id,
      engine: spec.engine,
      doctrine: {
        authority: 'ts-first',
        substrate: 'cypher-driven',
        kernelRole: 'cache',
        discipline: 'specification-driven',
      },
      boundaryContract,
      storeContract,
      substrateMapping,
      schemaGeneration,
      datasetSdkGeneration,
      ontologyImage,
      ontology: {
        mode: 'transcendental-logic',
        total: spec.ontologies.length,
        profiles: spec.ontologies.map((ontology) => ontology.profile),
        ids: spec.ontologies.map((ontology) => ontology.id),
      },
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
  const schemaGeneration = buildSchemaGeneration(specification);

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
    substrateMapping: buildSubstrateMapping(specification),
    schemaGeneration,
    datasetSdkGeneration: buildDatasetSdkGeneration(
      specification,
      schemaGeneration,
    ),
    ontologyImage: buildOntologyDataFrameImage(specification),
    storeContract: buildThreeStoreContract(specification, input),
    kernel: buildKernelArtifacts(specification, input),
    mvc: buildMvcArtifacts(specification),
  };

  return SdslCompilerArtifactsSchema.parse(artifacts);
}
