import { z } from 'zod';
import {
  compileFactStorePrototype,
  factStorePrototypeSchema,
} from './fact-store-prototype';
import {
  compileThingEntityShapes,
  type ThingEntityStateId,
} from './compile-thing-entity-shape';
import {
  compileWorldPropertyShapes,
  type WorldPropertyStateId,
} from './compile-world-property-shape';
import {
  compileRelationAspectShapes,
  type RelationAspectStateId,
} from './compile-relation-aspect-shape';
import {
  compileRelationStorePrototype,
  relationStorePrototypeSchema,
  type RelationStorePrototype,
} from './relation-store-prototype';

export const knowledgeStorePrototypeSchema = z.object({
  id: z.string(),
  description: z.string(),
  sdkBoundary: z.object({
    modelSdk: z.object({
      role: z.literal('pre-eval orchestration'),
      handoffTo: z.literal('logic-form-processor'),
    }),
    logicFormProcessor: z.object({
      role: z.literal('eval + synthesis'),
      emits: z.literal('knowledge-judgment'),
    }),
  }),
  stores: z.object({
    factStore: factStorePrototypeSchema,
    relationStore: relationStorePrototypeSchema,
  }),
  appearanceSide: z.object({
    entities: z.array(z.any()),
    properties: z.array(z.any()),
    aspects: z.array(z.any()),
  }),
  indices: z.object({
    entityStateIds: z.array(z.string()),
    propertyStateIds: z.array(z.string()),
    aspectStateIds: z.array(z.string()),
  }),
});

export type KnowledgeStorePrototype = z.infer<
  typeof knowledgeStorePrototypeSchema
>;

export async function compileKnowledgeStorePrototype(): Promise<KnowledgeStorePrototype> {
  const [factStore, relationStore, entities, properties, aspects] =
    await Promise.all([
      compileFactStorePrototype(),
      compileRelationStorePrototype(),
      compileThingEntityShapes(),
      compileWorldPropertyShapes(),
      compileRelationAspectShapes(),
    ]);

  const entityStateIds = entities
    .map((entity) => {
      const context =
        typeof entity.facets === 'object' && entity.facets
          ? (entity.facets as Record<string, unknown>).context
          : undefined;
      const sourceStateId =
        typeof context === 'object' && context
          ? (context as Record<string, unknown>).sourceStateId
          : undefined;
      return typeof sourceStateId === 'string' ? sourceStateId : undefined;
    })
    .filter((stateId): stateId is ThingEntityStateId => Boolean(stateId));

  const propertyStateIds = properties
    .map((property) => {
      const stateId =
        typeof property.state === 'object' && property.state
          ? (property.state as Record<string, unknown>).sourceStateId
          : undefined;
      return typeof stateId === 'string' ? stateId : undefined;
    })
    .filter((stateId): stateId is WorldPropertyStateId => Boolean(stateId));

  const aspectStateIds = aspects
    .map((aspect) => {
      const stateId =
        typeof aspect.state === 'object' && aspect.state
          ? (aspect.state as Record<string, unknown>).sourceStateId
          : undefined;
      return typeof stateId === 'string' ? stateId : undefined;
    })
    .filter((stateId): stateId is RelationAspectStateId => Boolean(stateId));

  return knowledgeStorePrototypeSchema.parse({
    id: 'knowledge-store-prototype-1',
    description:
      'EssentialRelation KnowledgeStore: FactStore + RelationStore synthesis for Agent eval.',
    sdkBoundary: {
      modelSdk: {
        role: 'pre-eval orchestration',
        handoffTo: 'logic-form-processor',
      },
      logicFormProcessor: {
        role: 'eval + synthesis',
        emits: 'knowledge-judgment',
      },
    },
    stores: {
      factStore,
      relationStore,
    },
    appearanceSide: {
      entities,
      properties,
      aspects,
    },
    indices: {
      entityStateIds,
      propertyStateIds,
      aspectStateIds,
    },
  });
}

export async function compileKnowledgeStorePrototypeBundle(): Promise<{
  factStore: z.infer<typeof factStorePrototypeSchema>;
  relationStore: RelationStorePrototype;
  knowledgeStore: KnowledgeStorePrototype;
}> {
  const [factStore, relationStore, knowledgeStore] = await Promise.all([
    compileFactStorePrototype(),
    compileRelationStorePrototype(),
    compileKnowledgeStorePrototype(),
  ]);

  return {
    factStore,
    relationStore,
    knowledgeStore,
  };
}
