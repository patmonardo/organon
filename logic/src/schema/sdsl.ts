import { z } from 'zod';

import type { EntityShapeRepo } from './entity';
import type { PropertyShapeRepo } from './property';
import type { AspectShapeRepo } from './aspect';

export const SdslClassificationSchema = z.object({
  genus: z.string().min(1),
  species: z.string().min(1),
});
export type SdslClassification = z.infer<typeof SdslClassificationSchema>;

export const SdslModelSpecSchema = z.object({
  id: z.string().min(1),
  label: z.string().min(1),
  kind: z.string().optional(),
});
export type SdslModelSpec = z.infer<typeof SdslModelSpecSchema>;

export const SdslFeatureSpecSchema = z.object({
  id: z.string().min(1),
  label: z.string().min(1),
  modelId: z.string().min(1).optional(),
  kind: z.string().optional(),
});
export type SdslFeatureSpec = z.infer<typeof SdslFeatureSpecSchema>;

export const SdslUdtSpecSchema = z.object({
  id: z.string().min(1),
  label: z.string().min(1),
  baseType: z.enum(['string', 'number', 'boolean', 'datetime', 'json']),
  modelId: z.string().min(1).optional(),
  featureId: z.string().min(1).optional(),
  constraints: z.array(z.string().min(1)).default([]),
});
export type SdslUdtSpec = z.infer<typeof SdslUdtSpecSchema>;

export const SdslUdfSpecSchema = z.object({
  id: z.string().min(1),
  label: z.string().min(1),
  inputUdtId: z.string().min(1),
  outputType: z.string().min(1),
  implementationRef: z.string().min(1),
  semantics: z
    .enum(['validate', 'normalize', 'enrich', 'project'])
    .default('validate'),
});
export type SdslUdfSpec = z.infer<typeof SdslUdfSpecSchema>;

export const SdslOntologyEntitySchema = z.object({
  id: z.string().min(1),
  iri: z.string().min(1),
  label: z.string().min(1).optional(),
  kind: z.enum(['class', 'property', 'aspect']).default('class'),
});
export type SdslOntologyEntity = z.infer<typeof SdslOntologyEntitySchema>;

export const SdslOntologyQuerySchema = z.object({
  id: z.string().min(1),
  language: z.enum(['sparql', 'cypher']).default('sparql'),
  text: z.string().min(1),
});
export type SdslOntologyQuery = z.infer<typeof SdslOntologyQuerySchema>;

export const SdslOntologyConstraintSchema = z.object({
  id: z.string().min(1),
  language: z.enum(['shacl', 'owl']).default('shacl'),
  text: z.string().min(1),
});
export type SdslOntologyConstraint = z.infer<
  typeof SdslOntologyConstraintSchema
>;

export const SdslOntologySpecSchema = z.object({
  id: z.string().min(1),
  iri: z.string().min(1),
  profile: z.enum(['owl', 'shacl', 'sparql']).default('owl'),
  entities: z.array(SdslOntologyEntitySchema).default([]),
  queries: z.array(SdslOntologyQuerySchema).default([]),
  constraints: z.array(SdslOntologyConstraintSchema).default([]),
  meta: z.record(z.string(), z.unknown()).optional(),
});
export type SdslOntologySpec = z.infer<typeof SdslOntologySpecSchema>;

export const SdslEngineIntentSchema = z.object({
  logicalForm: z.string().default('relative-form'),
  mvc: z.string().default('react-next'),
});
export type SdslEngineIntent = z.infer<typeof SdslEngineIntentSchema>;

export const SdslSpecificationSchema = z.object({
  id: z.string().min(1),
  title: z.string().min(1),
  description: z.string().optional(),
  classification: SdslClassificationSchema,
  gdslSource: z.string().optional(),
  ontologies: z.array(SdslOntologySpecSchema).default([]),
  engine: SdslEngineIntentSchema.default({
    logicalForm: 'relative-form',
    mvc: 'react-next',
  }),
  models: z.array(SdslModelSpecSchema).default([]),
  features: z.array(SdslFeatureSpecSchema).default([]),
  udts: z.array(SdslUdtSpecSchema).default([]),
  udfs: z.array(SdslUdfSpecSchema).default([]),
  tags: z.array(z.string()).default([]),
  meta: z.record(z.string(), z.unknown()).optional(),
});
export type SdslSpecification = z.infer<typeof SdslSpecificationSchema>;

export type SdslDesignSurfacePayload = {
  entity: Partial<EntityShapeRepo>;
  properties: Array<Partial<PropertyShapeRepo>>;
  aspects: Array<Partial<AspectShapeRepo>>;
};

export function specificationToDesignSurface(
  specInput: SdslSpecification,
): SdslDesignSurfacePayload {
  const spec = SdslSpecificationSchema.parse(specInput);
  const now = Date.now();
  const specTag = `sdsl.spec:${spec.id}`;

  const entity: Partial<EntityShapeRepo> = {
    id: `sdsl:spec:${spec.id}`,
    type: 'sdsl.Specification',
    formId: 'form:sdsl-specification',
    name: spec.title,
    description: spec.description,
    tags: [specTag, 'sdsl', 'design-surface', ...spec.tags],
    facets: {
      specification: spec,
      classification: spec.classification,
      engine: spec.engine,
      ontologies: spec.ontologies,
      counts: {
        models: spec.models.length,
        features: spec.features.length,
        udts: spec.udts.length,
        udfs: spec.udfs.length,
        ontologies: spec.ontologies.length,
      },
    },
    signature: {
      protocol: 'gdsl/ts-json',
      persistence: 'neo4j-entity-property-aspect',
    },
    createdAt: now,
    updatedAt: now,
  };

  const properties: Array<Partial<PropertyShapeRepo>> = [
    ...spec.models.map((model) => ({
      id: `sdsl:model:${spec.id}:${model.id}`,
      type: 'sdsl.ModelSpec',
      name: model.label,
      tags: [specTag, 'sdsl', 'model-spec'],
      facets: {
        specificationId: spec.id,
        model,
      },
      signature: {
        kind: model.kind ?? 'model',
      },
      createdAt: now,
      updatedAt: now,
    })),
    ...spec.features.map((feature) => ({
      id: `sdsl:feature:${spec.id}:${feature.id}`,
      type: 'sdsl.FeatureSpec',
      name: feature.label,
      tags: [specTag, 'sdsl', 'feature-spec'],
      facets: {
        specificationId: spec.id,
        feature,
      },
      signature: {
        kind: feature.kind ?? 'feature',
        modelId: feature.modelId,
      },
      createdAt: now,
      updatedAt: now,
    })),
  ];

  const aspects: Array<Partial<AspectShapeRepo>> = [
    {
      id: `sdsl:aspect:${spec.id}:engine-intent`,
      type: 'sdsl.EngineIntentAspect',
      name: `${spec.title} Engine Intent`,
      tags: [specTag, 'sdsl', 'engine-aspect'],
      facets: {
        specificationId: spec.id,
        logicalForm: spec.engine.logicalForm,
        mvc: spec.engine.mvc,
      },
      signature: {
        adapterFlow: 'gdsl -> ts-json -> mvc',
      },
      createdAt: now,
      updatedAt: now,
    },
    {
      id: `sdsl:aspect:${spec.id}:classification`,
      type: 'sdsl.ClassificationAspect',
      name: `${spec.title} Classification`,
      tags: [specTag, 'sdsl', 'classification-aspect'],
      facets: {
        specificationId: spec.id,
        classification: spec.classification,
      },
      signature: {
        genus: spec.classification.genus,
        species: spec.classification.species,
      },
      createdAt: now,
      updatedAt: now,
    },
    {
      id: `sdsl:aspect:${spec.id}:ontology`,
      type: 'sdsl.OntologyAspect',
      name: `${spec.title} Ontology`,
      tags: [specTag, 'sdsl', 'ontology-aspect'],
      facets: {
        specificationId: spec.id,
        ontologies: spec.ontologies,
      },
      signature: {
        profileKinds: spec.ontologies.map((ontology) => ontology.profile),
      },
      createdAt: now,
      updatedAt: now,
    },
  ];

  return {
    entity,
    properties,
    aspects,
  };
}
