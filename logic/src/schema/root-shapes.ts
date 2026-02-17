import { z } from 'zod';
import { Type, Label } from './base';

export const RootLanguageSchema = z.enum(['gdsl', 'sdsl']);
export type RootLanguage = z.infer<typeof RootLanguageSchema>;

export const RootRepositoryTargetSchema = z.enum([
  'fact-repo',
  'relation-repo',
  'knowledge-repo',
]);
export type RootRepositoryTarget = z.infer<typeof RootRepositoryTargetSchema>;

export const RootStoreTargetSchema = z.preprocess((input) => {
  if (input === 'fact-store') return 'fact-repo';
  if (input === 'relation-store') return 'relation-repo';
  if (input === 'knowledge-store') return 'knowledge-repo';
  return input;
}, RootRepositoryTargetSchema);
export type RootStoreTarget = z.infer<typeof RootStoreTargetSchema>;

export const RootPrincipleStageSchema = z.preprocess(
  (input) => {
    if (input === 'ground') return 'morph';
    return input;
  },
  z.enum(['form', 'context', 'morph']),
);

export type RootPrincipleStage = z.infer<typeof RootPrincipleStageSchema>;

export const RootLawStageSchema = z.enum(['entity', 'property', 'aspect']);
export type RootLawStage = z.infer<typeof RootLawStageSchema>;

const RootRuntimeTraceSchema = z
  .object({
    source: z.string(),
    traceId: z.string().optional(),
    recordedAt: z.number().optional(),
  })
  .optional();

const RootProvenanceSchema = RootRuntimeTraceSchema;

const UnknownRecordSchema = z.record(z.string(), z.unknown());

export const RootEntityReferenceSchema = z.object({
  id: z.string(),
  type: Type,
  role: z.string().optional(),
});
export type RootEntityReference = z.infer<typeof RootEntityReferenceSchema>;

export const RootCommonMetaSchema = z.record(z.string(), z.unknown());
export type RootCommonMeta = z.infer<typeof RootCommonMetaSchema>;

export const RootContextFacetsSchema = z
  .object({
    presuppositions: z
      .array(
        z.object({
          name: z.string(),
          definition: z.string(),
          posited: z.boolean().default(true),
        }),
      )
      .optional(),
    scope: z
      .object({
        modal: z.enum(['actual', 'possible', 'necessary', 'contingent']),
        domain: z.array(z.string()),
        phase: z.string().optional(),
      })
      .optional(),
    conditions: z
      .array(
        z.object({
          id: z.string(),
          constraint: z.string(),
          predicate: z.string().optional(),
        }),
      )
      .optional(),
    parentContext: z.string().optional(),
  })
  .catchall(z.unknown());
export type RootContextFacets = z.infer<typeof RootContextFacetsSchema>;

export const RootEntityFacetsSchema = z
  .object({
    dialecticState: z.unknown().optional(),
    phase: z.string().optional(),
    moments: z.array(z.unknown()).optional(),
    invariants: z.array(z.unknown()).optional(),
    context: z.unknown().optional(),
  })
  .catchall(z.unknown());
export type RootEntityFacets = z.infer<typeof RootEntityFacetsSchema>;

export const RootMorphCompositionSchema = z
  .object({
    kind: z.enum(['single', 'pipeline', 'composite']).default('single'),
    mode: z.enum(['sequential', 'parallel']).optional(),
    steps: z.array(z.string()).default([]),
  })
  .default({ kind: 'single', steps: [] });
export type RootMorphComposition = z.infer<typeof RootMorphCompositionSchema>;

export const RootMorphFacetsSchema = z
  .object({
    dialecticState: z.unknown().optional(),
    phase: z.string().optional(),
    container: z
      .object({
        holds: z.array(z.string()),
        activeUnity: z.array(
          z.object({
            name: z.string(),
            definition: z.string(),
            contains: z.string().optional(),
          }),
        ),
      })
      .optional(),
    transformation: z
      .object({
        principle: z.string().optional(),
        mechanism: z.string().optional(),
        transitions: z.array(
          z.object({
            from: z.string(),
            to: z.string(),
            mechanism: z.string().optional(),
            reason: z.string().optional(),
          }),
        ),
      })
      .optional(),
    context: z.unknown().optional(),
  })
  .catchall(z.unknown());
export type RootMorphFacets = z.infer<typeof RootMorphFacetsSchema>;

export const RootPropertyFacetsSchema = z
  .object({
    dialecticState: z.unknown().optional(),
    phase: z.string().optional(),
    law: z
      .object({
        invariants: z.array(
          z.object({
            id: z.string(),
            constraint: z.string(),
            predicate: z.string().optional(),
            universality: z.enum(['necessary', 'conditional']),
          }),
        ),
        universality: z.enum(['necessary', 'conditional']),
      })
      .optional(),
    facticity: z
      .object({
        grounds: z.array(z.string()),
        conditions: z.array(z.string()),
        evidence: z.array(
          z.object({
            name: z.string(),
            definition: z.string(),
            type: z.string(),
          }),
        ),
      })
      .optional(),
    mediates: z
      .object({
        fromEntities: z.array(z.string()),
        toAspects: z.array(z.string()),
      })
      .optional(),
    context: z.unknown().optional(),
  })
  .catchall(z.unknown());
export type RootPropertyFacets = z.infer<typeof RootPropertyFacetsSchema>;

export const RootAspectFacetsSchema = z
  .object({
    dialecticState: z.unknown().optional(),
    phase: z.string().optional(),
    spectrum: z
      .object({
        poles: z.array(
          z.object({
            name: z.string(),
            definition: z.string(),
            oppositeTo: z.string().optional(),
          }),
        ),
        range: z.number(),
        dialectical: z.boolean(),
      })
      .optional(),
    essentialRelation: z
      .object({
        spectrum: z.unknown(),
        connections: z.array(
          z.object({
            from: z.string(),
            to: z.string().optional(),
            relation: z.string().optional(),
            type: z.string(),
          }),
        ),
        appearing: z.unknown(),
        groundedIn: z.string().optional(),
      })
      .optional(),
    relations: z
      .array(
        z.object({
          from: z.string(),
          to: z.string().optional(),
          relation: z.string().optional(),
          type: z.string(),
        }),
      )
      .optional(),
    appearing: z
      .object({
        mode: z.enum(['immanent', 'externality', 'reflection', 'passover']),
        triggers: z.array(z.string()),
        effects: z.array(z.string()),
      })
      .optional(),
    constraints: z
      .array(
        z.object({
          id: z.string(),
          constraint: z.string(),
          predicate: z.string().optional(),
        }),
      )
      .optional(),
    context: z.unknown().optional(),
  })
  .catchall(z.unknown());
export type RootAspectFacets = z.infer<typeof RootAspectFacetsSchema>;

export const RootProcessorSpecSchema = UnknownRecordSchema;
export type RootProcessorSpec = z.infer<typeof RootProcessorSpecSchema>;

export const RootProcessorDefinitionSchema = RootProcessorSpecSchema;
export type RootProcessorDefinition = RootProcessorSpec;

export const RootFormShapeSchema = z.object({
  id: z.string(),
  stage: z.literal('form').default('form'),
  type: Type,
  name: Label.optional(),
  description: z.string().optional(),
  language: z.literal('gdsl').default('gdsl'),
  form: UnknownRecordSchema.default({}),
  invariants: z.array(z.string()).default([]),
  provenance: RootProvenanceSchema,
  createdAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
  updatedAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
});
export type RootFormShape = z.infer<typeof RootFormShapeSchema>;

export const RootContextShapeSchema = z.object({
  id: z.string(),
  stage: z.literal('context').default('context'),
  type: Type,
  name: Label.optional(),
  description: z.string().optional(),
  language: z.literal('gdsl').default('gdsl'),
  formShapeId: z.string(),
  state: UnknownRecordSchema.default({}),
  entities: z.array(RootEntityReferenceSchema).default([]),
  relations: z.array(z.string()).default([]),
  signature: UnknownRecordSchema.optional(),
  facets: RootContextFacetsSchema.default({}),
  context: UnknownRecordSchema.default({}),
  invariants: z.array(z.string()).default([]),
  status: z.string().optional(),
  tags: z.array(z.string()).optional(),
  meta: RootCommonMetaSchema.optional(),
  provenance: RootProvenanceSchema,
  createdAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
  updatedAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
});
export type RootContextShape = z.infer<typeof RootContextShapeSchema>;

export const RootMorphShapeSchema = z.object({
  id: z.string(),
  stage: z.literal('morph').default('morph'),
  type: Type,
  name: Label.optional(),
  description: z.string().optional(),
  language: z.literal('gdsl').default('gdsl'),
  contextShapeId: z.string(),
  inputType: z.string().default('FormShape'),
  outputType: z.string().default('FormShape'),
  transformFn: z.string().optional(),
  state: UnknownRecordSchema.default({}),
  signature: UnknownRecordSchema.optional(),
  facets: RootMorphFacetsSchema.default({}),
  composition: RootMorphCompositionSchema,
  config: UnknownRecordSchema.default({}),
  morph: RootProcessorSpecSchema.default({}),
  invariants: z.array(z.string()).default([]),
  status: z.string().optional(),
  tags: z.array(z.string()).optional(),
  meta: RootCommonMetaSchema.optional(),
  provenance: RootProvenanceSchema,
  createdAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
  updatedAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
});
export type RootMorphShape = z.infer<typeof RootMorphShapeSchema>;

export const RootGroundShapeSchema = RootMorphShapeSchema;
export type RootGroundShape = RootMorphShape;

export const RootPrincipleShapeSchema = z.discriminatedUnion('stage', [
  RootFormShapeSchema,
  RootContextShapeSchema,
  RootMorphShapeSchema,
]);
export type RootPrincipleShape = z.infer<typeof RootPrincipleShapeSchema>;

export const RootPrincipleEvolutionSchema = z
  .object({
    form: RootFormShapeSchema,
    context: RootContextShapeSchema,
    morph: RootMorphShapeSchema,
    transitions: z
      .array(
        z.object({
          from: RootPrincipleStageSchema,
          to: RootPrincipleStageSchema,
          mechanism: z.string().optional(),
        }),
      )
      .default([
        { from: 'form', to: 'context', mechanism: 'contextualization' },
        { from: 'context', to: 'morph', mechanism: 'processing' },
      ]),
  })
  .superRefine((value, ctx) => {
    if (value.context.formShapeId !== value.form.id) {
      ctx.addIssue({
        code: 'custom',
        message: 'Context must reference form.id via formShapeId',
        path: ['context', 'formShapeId'],
      });
    }
    if (value.morph.contextShapeId !== value.context.id) {
      ctx.addIssue({
        code: 'custom',
        message: 'Morph must reference context.id via contextShapeId',
        path: ['morph', 'contextShapeId'],
      });
    }
  });
export type RootPrincipleEvolution = z.infer<
  typeof RootPrincipleEvolutionSchema
>;

export const RootEntityShapeSchema = z
  .object({
    id: z.string(),
    stage: z.literal('entity').default('entity'),
    type: Type,
    name: Label.optional(),
    description: z.string().optional(),
    language: z.literal('gdsl').default('gdsl'),
    formId: z.string().optional(),
    containerBindings: z
      .object({
        formShapeId: z.string().optional(),
        contextShapeId: z.string().optional(),
        morphShapeId: z.string().optional(),
        groundShapeId: z.string().optional(),
      })
      .optional(),
    particulars: z.array(RootEntityReferenceSchema).optional(),
    values: UnknownRecordSchema.default({}),
    signature: UnknownRecordSchema.optional(),
    facets: RootEntityFacetsSchema.default({}),
    state: z.enum([
      'candidate',
      'grounded',
      'persisted',
      'deferred',
      'revised',
    ]),
    targetStore: RootStoreTargetSchema.optional(),
    contradictions: z.array(z.string()).default([]),
    status: z.string().optional(),
    tags: z.array(z.string()).optional(),
    meta: RootCommonMetaSchema.optional(),
    provenance: RootProvenanceSchema,
    createdAt: z
      .number()
      .optional()
      .default(() => Date.now())
      .optional(),
    updatedAt: z
      .number()
      .optional()
      .default(() => Date.now())
      .optional(),
  })
  .superRefine((value, ctx) => {
    if (value.state === 'persisted') {
      if (!value.targetStore) {
        ctx.addIssue({
          code: 'custom',
          message: 'Persisted entity requires targetStore',
          path: ['targetStore'],
        });
      }
    }
  });
export type RootEntityShape = z.infer<typeof RootEntityShapeSchema>;

export const RootPropertyShapeSchema = z.object({
  id: z.string(),
  stage: z.literal('property').default('property'),
  type: Type,
  name: Label.optional(),
  description: z.string().optional(),
  language: z.literal('gdsl').default('gdsl'),
  entityShapeId: z.string(),
  signature: UnknownRecordSchema.optional(),
  facets: RootPropertyFacetsSchema.default({}),
  property: z
    .object({
      key: z.string(),
      valueType: z.string(),
      value: z.unknown().optional(),
      constraints: z.array(z.string()).default([]),
    })
    .default({ key: '', valueType: 'unknown', constraints: [] }),
  state: z
    .enum(['candidate', 'bound', 'validated', 'deferred'])
    .default('candidate'),
  contradictions: z.array(z.string()).default([]),
  status: z.string().optional(),
  tags: z.array(z.string()).optional(),
  meta: RootCommonMetaSchema.optional(),
  provenance: RootProvenanceSchema,
  createdAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
  updatedAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
});
export type RootPropertyShape = z.infer<typeof RootPropertyShapeSchema>;

export const RootAspectShapeSchema = z.object({
  id: z.string(),
  stage: z.literal('aspect').default('aspect'),
  type: Type,
  name: Label.optional(),
  description: z.string().optional(),
  language: z.literal('gdsl').default('gdsl'),
  propertyShapeId: z.string(),
  signature: UnknownRecordSchema.optional(),
  facets: RootAspectFacetsSchema.default({}),
  aspect: z
    .object({
      relationType: z.string(),
      subjectEntityShapeId: z.string().optional(),
      objectEntityShapeId: z.string().optional(),
      predicate: z.string().optional(),
      polarity: z.enum(['affirmed', 'negated', 'mediated']).default('affirmed'),
    })
    .default({ relationType: 'has-aspect', polarity: 'affirmed' }),
  state: z
    .enum(['candidate', 'coherent', 'incoherent', 'sublated'])
    .default('candidate'),
  contradictions: z.array(z.string()).default([]),
  status: z.string().optional(),
  tags: z.array(z.string()).optional(),
  meta: RootCommonMetaSchema.optional(),
  provenance: RootProvenanceSchema,
  createdAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
  updatedAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
});
export type RootAspectShape = z.infer<typeof RootAspectShapeSchema>;

export const RootLawShapeSchema = z.discriminatedUnion('stage', [
  RootEntityShapeSchema,
  RootPropertyShapeSchema,
  RootAspectShapeSchema,
]);
export type RootLawShape = z.infer<typeof RootLawShapeSchema>;

export const RootLawEvolutionSchema = z
  .object({
    entity: RootEntityShapeSchema,
    property: RootPropertyShapeSchema,
    aspect: RootAspectShapeSchema,
    transitions: z
      .array(
        z.object({
          from: RootLawStageSchema,
          to: RootLawStageSchema,
          mechanism: z.string().optional(),
        }),
      )
      .default([
        { from: 'entity', to: 'property', mechanism: 'determination' },
        { from: 'property', to: 'aspect', mechanism: 'relationalization' },
      ]),
  })
  .superRefine((value, ctx) => {
    if (value.property.entityShapeId !== value.entity.id) {
      ctx.addIssue({
        code: 'custom',
        message: 'Property must reference entity.id via entityShapeId',
        path: ['property', 'entityShapeId'],
      });
    }
    if (value.aspect.propertyShapeId !== value.property.id) {
      ctx.addIssue({
        code: 'custom',
        message: 'Aspect must reference property.id via propertyShapeId',
        path: ['aspect', 'propertyShapeId'],
      });
    }
  });
export type RootLawEvolution = z.infer<typeof RootLawEvolutionSchema>;

export const RootPrincipleLawBridgeSchema = z.object({
  principle: RootPrincipleEvolutionSchema,
  law: RootLawEvolutionSchema,
  correspondences: z
    .array(
      z.object({
        principleStage: RootPrincipleStageSchema,
        lawStage: RootLawStageSchema,
        principle: z.string(),
      }),
    )
    .default([
      {
        principleStage: 'form',
        lawStage: 'entity',
        principle: 'admissible form determines entity identity space',
      },
      {
        principleStage: 'context',
        lawStage: 'property',
        principle: 'context determines property validity and scope',
      },
      {
        principleStage: 'morph',
        lawStage: 'aspect',
        principle: 'morphic processing determines coherent aspect relation',
      },
    ]),
});
export type RootPrincipleLawBridge = z.infer<
  typeof RootPrincipleLawBridgeSchema
>;

export const RootCanonicalShapeSchema = z.union([
  RootPrincipleShapeSchema,
  RootLawShapeSchema,
]);
export type RootCanonicalShape = z.infer<typeof RootCanonicalShapeSchema>;

export const RootShapeReferenceSchema = z.object({
  id: z.string(),
  stage: z.union([RootPrincipleStageSchema, RootLawStageSchema]),
});
export type RootShapeReference = z.infer<typeof RootShapeReferenceSchema>;

export const SdslShapeKindSchema = z.enum(['container', 'contained']);
export type SdslShapeKind = z.infer<typeof SdslShapeKindSchema>;

export const SdslModelShapeSchema = z.object({
  id: z.string(),
  language: z.literal('sdsl').default('sdsl'),
  dialect: z.string(),
  kind: SdslShapeKindSchema,
  payload: UnknownRecordSchema.default({}),
  mapsTo: RootShapeReferenceSchema,
  provenance: RootProvenanceSchema,
});
export type SdslModelShape = z.infer<typeof SdslModelShapeSchema>;

export const SdslMorphicRestrictionSchema = z.object({
  requiredKeys: z.array(z.string()).default([]),
  forbiddenKeys: z.array(z.string()).default([]),
  stageCompatibility: z.array(
    z.object({
      sdslKind: SdslShapeKindSchema,
      gdslStage: z.union([RootPrincipleStageSchema, RootLawStageSchema]),
    }),
  ),
});
export type SdslMorphicRestriction = z.infer<
  typeof SdslMorphicRestrictionSchema
>;

export const SdslGdslIsomorphicClaimSchema = z
  .object({
    id: z.string(),
    sdslShape: SdslModelShapeSchema,
    gdslShapeRef: RootShapeReferenceSchema,
    restriction: SdslMorphicRestrictionSchema,
    correspondence: z.record(z.string(), z.string()).default({}),
    notes: z.string().optional(),
  })
  .superRefine((value, ctx) => {
    if (value.sdslShape.mapsTo.id !== value.gdslShapeRef.id) {
      ctx.addIssue({
        code: 'custom',
        message: 'sdslShape.mapsTo.id must equal gdslShapeRef.id',
        path: ['sdslShape', 'mapsTo', 'id'],
      });
    }

    if (value.sdslShape.mapsTo.stage !== value.gdslShapeRef.stage) {
      ctx.addIssue({
        code: 'custom',
        message: 'sdslShape.mapsTo.stage must equal gdslShapeRef.stage',
        path: ['sdslShape', 'mapsTo', 'stage'],
      });
    }

    const payloadKeys = Object.keys(value.sdslShape.payload);

    for (const key of value.restriction.requiredKeys) {
      if (!payloadKeys.includes(key)) {
        ctx.addIssue({
          code: 'custom',
          message: `Missing required SDSL key: ${key}`,
          path: ['sdslShape', 'payload', key],
        });
      }
    }

    for (const forbidden of value.restriction.forbiddenKeys) {
      if (payloadKeys.includes(forbidden)) {
        ctx.addIssue({
          code: 'custom',
          message: `Forbidden SDSL key present: ${forbidden}`,
          path: ['sdslShape', 'payload', forbidden],
        });
      }
    }

    const stageAllowed = value.restriction.stageCompatibility.some(
      (rule) =>
        rule.sdslKind === value.sdslShape.kind &&
        rule.gdslStage === value.gdslShapeRef.stage,
    );

    if (!stageAllowed) {
      ctx.addIssue({
        code: 'custom',
        message: 'SDSL kind is not compatible with referenced GDSL stage',
        path: ['restriction', 'stageCompatibility'],
      });
    }
  });
export type SdslGdslIsomorphicClaim = z.infer<
  typeof SdslGdslIsomorphicClaimSchema
>;
