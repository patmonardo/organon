import { z } from 'zod';

const UnknownRecordSchema = z.record(z.string(), z.unknown());

export const OrganonDefinitionKindSchema = z.enum([
  'entity',
  'property',
  'aspect',
  'constraint',
  'rule',
  'vocabulary',
  'provenance',
]);
export type OrganonDefinitionKind = z.infer<typeof OrganonDefinitionKindSchema>;

export const EntityDefinitionSchema = z.object({
  kind: z.literal('entity').default('entity'),
  id: z.string().min(1),
  label: z.string().min(1),
  identityKeys: z.array(z.string().min(1)).min(1),
  extends: z.array(z.string().min(1)).default([]),
  disjointWith: z.array(z.string().min(1)).default([]),
  equivalentTo: z.array(z.string().min(1)).default([]),
  facets: UnknownRecordSchema.default({}),
});
export type EntityDefinition = z.infer<typeof EntityDefinitionSchema>;

export const PropertyDefinitionSchema = z
  .object({
    kind: z.literal('property').default('property'),
    id: z.string().min(1),
    subjectTypeId: z.string().min(1),
    valueKind: z.enum(['datatype', 'object']),
    datatype: z.string().optional(),
    objectTypeId: z.string().optional(),
    cardinality: z
      .object({
        min: z.number().int().min(0).optional(),
        max: z.number().int().min(0).optional(),
        exact: z.number().int().min(0).optional(),
      })
      .optional(),
    functional: z.boolean().optional(),
    defaultValue: z.unknown().optional(),
  })
  .superRefine((value, ctx) => {
    if (value.valueKind === 'datatype' && !value.datatype) {
      ctx.addIssue({
        code: 'custom',
        message: 'PropertyDefinition with valueKind=datatype requires datatype',
        path: ['datatype'],
      });
    }
    if (value.valueKind === 'object' && !value.objectTypeId) {
      ctx.addIssue({
        code: 'custom',
        message:
          'PropertyDefinition with valueKind=object requires objectTypeId',
        path: ['objectTypeId'],
      });
    }
  });
export type PropertyDefinition = z.infer<typeof PropertyDefinitionSchema>;

export const AspectDefinitionSchema = z.object({
  kind: z.literal('aspect').default('aspect'),
  id: z.string().min(1),
  relationType: z.string().min(1),
  subjectTypeId: z.string().min(1),
  objectTypeId: z.string().optional(),
  predicate: z.string().optional(),
  polarity: z.enum(['affirmed', 'negated', 'mediated']).default('affirmed'),
  constraints: z.array(z.string().min(1)).default([]),
});
export type AspectDefinition = z.infer<typeof AspectDefinitionSchema>;

export const ConstraintDefinitionSchema = z.object({
  kind: z.literal('constraint').default('constraint'),
  id: z.string().min(1),
  target: z.string().min(1),
  severity: z.enum(['info', 'warning', 'violation']).default('violation'),
  expression: z.string().optional(),
  message: z.string().optional(),
  closed: z.boolean().optional(),
  config: UnknownRecordSchema.default({}),
});
export type ConstraintDefinition = z.infer<typeof ConstraintDefinitionSchema>;

export const RuleDefinitionSchema = z.object({
  kind: z.literal('rule').default('rule'),
  id: z.string().min(1),
  ruleKind: z.enum(['derive', 'infer', 'normalize', 'validate']),
  body: z.string().min(1),
  target: z.string().optional(),
  dependsOn: z.array(z.string().min(1)).default([]),
  priority: z.number().int().optional(),
});
export type RuleDefinition = z.infer<typeof RuleDefinitionSchema>;

export const VocabularyDefinitionSchema = z.object({
  kind: z.literal('vocabulary').default('vocabulary'),
  id: z.string().min(1),
  scheme: z.string().min(1),
  prefLabel: z.string().min(1),
  altLabels: z.array(z.string().min(1)).default([]),
  broader: z.array(z.string().min(1)).default([]),
  narrower: z.array(z.string().min(1)).default([]),
  exactMatch: z.array(z.string().min(1)).default([]),
});
export type VocabularyDefinition = z.infer<typeof VocabularyDefinitionSchema>;

export const ProvenanceDefinitionSchema = z.object({
  kind: z.literal('provenance').default('provenance'),
  id: z.string().min(1),
  source: z.string().min(1),
  recordedAt: z.string().min(1),
  agent: z.string().optional(),
  confidence: z.number().min(0).max(1).optional(),
  method: z.string().optional(),
  lineage: z.array(z.string().min(1)).default([]),
});
export type ProvenanceDefinition = z.infer<typeof ProvenanceDefinitionSchema>;

export const OrganonDefinitionSchema = z.discriminatedUnion('kind', [
  EntityDefinitionSchema,
  PropertyDefinitionSchema,
  AspectDefinitionSchema,
  ConstraintDefinitionSchema,
  RuleDefinitionSchema,
  VocabularyDefinitionSchema,
  ProvenanceDefinitionSchema,
]);
export type OrganonDefinition = z.infer<typeof OrganonDefinitionSchema>;

export const OrganonDefinitionalPackageSchema = z
  .object({
    id: z.string().min(1),
    title: z.string().min(1),
    version: z.string().min(1).default('0.1.0'),
    definitions: z.array(OrganonDefinitionSchema),
    meta: UnknownRecordSchema.default({}),
  })
  .superRefine((value, ctx) => {
    const entityDefinitions = value.definitions.filter(
      (definition) => definition.kind === 'entity',
    );

    if (entityDefinitions.length === 0) {
      ctx.addIssue({
        code: 'custom',
        message: 'Definitional package requires at least one Entity Definition',
        path: ['definitions'],
      });
    }

    const definitionIds = new Set(
      value.definitions.map((definition) => definition.id),
    );
    const entityIds = new Set(
      entityDefinitions.map(
        (definition) =>
          (definition as z.infer<typeof EntityDefinitionSchema>).id,
      ),
    );

    value.definitions.forEach((definition, index) => {
      if (definition.kind === 'property') {
        if (!entityIds.has(definition.subjectTypeId)) {
          ctx.addIssue({
            code: 'custom',
            message: `PropertyDefinition subjectTypeId must reference an Entity Definition: ${definition.subjectTypeId}`,
            path: ['definitions', index, 'subjectTypeId'],
          });
        }
        if (
          definition.objectTypeId &&
          !entityIds.has(definition.objectTypeId)
        ) {
          ctx.addIssue({
            code: 'custom',
            message: `PropertyDefinition objectTypeId must reference an Entity Definition: ${definition.objectTypeId}`,
            path: ['definitions', index, 'objectTypeId'],
          });
        }
      }

      if (definition.kind === 'aspect') {
        if (!entityIds.has(definition.subjectTypeId)) {
          ctx.addIssue({
            code: 'custom',
            message: `AspectDefinition subjectTypeId must reference an Entity Definition: ${definition.subjectTypeId}`,
            path: ['definitions', index, 'subjectTypeId'],
          });
        }
        if (
          definition.objectTypeId &&
          !entityIds.has(definition.objectTypeId)
        ) {
          ctx.addIssue({
            code: 'custom',
            message: `AspectDefinition objectTypeId must reference an Entity Definition: ${definition.objectTypeId}`,
            path: ['definitions', index, 'objectTypeId'],
          });
        }
      }

      if (definition.kind === 'constraint' || definition.kind === 'rule') {
        if (definition.target && !definitionIds.has(definition.target)) {
          ctx.addIssue({
            code: 'custom',
            message: `Target must reference an existing definition id: ${definition.target}`,
            path: ['definitions', index, 'target'],
          });
        }
      }
    });
  });

export type OrganonDefinitionalPackage = z.infer<
  typeof OrganonDefinitionalPackageSchema
>;
