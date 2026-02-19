import { z } from 'zod';
import {
  compileRelationAspectShapes,
  type RelationAspectStateId,
} from './compile-relation-aspect-shape';

export const relationStorePrototypeSchema = z.object({
  id: z.string(),
  relationAspects: z.array(z.any()),
  indexedStates: z.array(z.string()),
  evaluationMode: z.literal('agent-free-reason'),
  factStoreInterface: z.object({
    enabled: z.boolean(),
    strategy: z.literal('evaluate-against-fact-store'),
  }),
});

export type RelationStorePrototype = z.infer<
  typeof relationStorePrototypeSchema
>;

export async function compileRelationStorePrototype(): Promise<RelationStorePrototype> {
  const relationAspects = await compileRelationAspectShapes();
  const indexedStates = relationAspects
    .map((aspect) => {
      const state =
        typeof aspect.state === 'object' && aspect.state
          ? (aspect.state as Record<string, unknown>).sourceStateId
          : undefined;
      return typeof state === 'string' ? state : undefined;
    })
    .filter((state): state is RelationAspectStateId => Boolean(state));

  return relationStorePrototypeSchema.parse({
    id: 'relationstore-prototype-1',
    relationAspects,
    indexedStates,
    evaluationMode: 'agent-free-reason',
    factStoreInterface: {
      enabled: true,
      strategy: 'evaluate-against-fact-store',
    },
  });
}
