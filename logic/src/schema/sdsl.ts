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
  engine: SdslEngineIntentSchema.default({
    logicalForm: 'relative-form',
    mvc: 'react-next',
  }),
  models: z.array(SdslModelSpecSchema).default([]),
  features: z.array(SdslFeatureSpecSchema).default([]),
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
      counts: {
        models: spec.models.length,
        features: spec.features.length,
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
  ];

  return {
    entity,
    properties,
    aspects,
  };
}
