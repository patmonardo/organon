import { z } from 'zod';
import { reflectionFormShapeSchema } from './reflection-form-shape';
import { compileEssenceReflectionFormShapes } from './compile-reflection-form-shape';
import { compileFoundationContextTriad } from './compile-foundation-context-shape';
import {
  compileGroundMorphStateRecord,
  compileReflectiveConsciousnessUnitySeed,
} from './compile-ground-morph-state';

const determinationKindEnum = z.enum([
  'identity',
  'difference',
  'contradiction',
]);

export const contextDeterminationSchema = z.object({
  id: z.string(),
  kind: determinationKindEnum,
  sourceStateId: z.string(),
  predicate: z.string(),
  resolved: z.boolean().default(false),
  notes: z.string().optional(),
});

export const contextShapePrototypeSchema = z.object({
  id: z.string(),
  fromFormShapeId: z.string(),
  fromRefStateId: z.enum(['ref-14']),
  determinations: z.array(contextDeterminationSchema).min(1),
  contradictionGraph: z.object({
    root: z.string(),
    markedSubgraphs: z.array(
      z.object({
        id: z.string(),
        condition: z.string(),
        resolutionRule: z.string(),
      }),
    ),
  }),
  handoff: z.object({
    toMorphShape: z.boolean(),
    reason: z.string(),
  }),
});

export const morphShapePrototypeSchema = z.object({
  id: z.string(),
  fromContextShapeId: z.string(),
  conditionStateId: z.enum(['con-1', 'con-4', 'con-10', 'con-13', 'con-15']),
  conditionedGenesis: z.object({
    allConditionsAtHand: z.boolean(),
    concreteExistenceReady: z.boolean(),
    transitionTarget: z.string(),
  }),
  subgraphActions: z.array(
    z.object({
      action: z.enum(['attach', 'revise', 'sublate']),
      subgraphId: z.string(),
      rationale: z.string(),
    }),
  ),
});

export const factStorePrototypeSchema = z.object({
  id: z.string(),
  formShape: reflectionFormShapeSchema,
  contextShape: contextShapePrototypeSchema,
  morphShape: morphShapePrototypeSchema,
  facticity: z.object({
    state: z.enum(['potential', 'conditioned', 'concrete']),
    evidence: z.array(z.string()),
    lastTransition: z.string(),
  }),
});

function pickLast<T>(items: T[]): T | undefined {
  return items.length > 0 ? items[items.length - 1] : undefined;
}

export async function compileContextShapePrototype(): Promise<ContextShapePrototype> {
  const reflectionShapes = await compileEssenceReflectionFormShapes();
  const triad = await compileFoundationContextTriad();

  const finalReflection = reflectionShapes.find(
    (shape) => shape.refStateId === 'ref-14',
  );
  if (!finalReflection) {
    throw new Error(
      'Missing compiled reflection state ref-14 for context synthesis.',
    );
  }

  const identity = pickLast(triad.identity);
  const difference = pickLast(triad.difference);
  const contradiction = pickLast(triad.contradiction);

  if (!identity || !difference || !contradiction) {
    throw new Error(
      'Foundation triad is incomplete: identity/difference/contradiction required.',
    );
  }

  return contextShapePrototypeSchema.parse({
    id: 'contextshape-reflection-1',
    fromFormShapeId: finalReflection.id,
    fromRefStateId: 'ref-14',
    determinations: [
      {
        id: 'det-identity',
        kind: 'identity',
        sourceStateId: identity.foundationStateId,
        predicate:
          identity.invariants[0]?.predicate ?? 'equals(identity, essence)',
        resolved: true,
      },
      {
        id: 'det-difference',
        kind: 'difference',
        sourceStateId: difference.foundationStateId,
        predicate:
          difference.invariants[0]?.predicate ??
          'equals(difference, negativity(reflection))',
        resolved: true,
      },
      {
        id: 'det-contradiction',
        kind: 'contradiction',
        sourceStateId: contradiction.foundationStateId,
        predicate:
          contradiction.invariants[0]?.predicate ??
          'equals(ground, resolved(contradiction))',
        resolved: contradiction.foundationStateId === 'ctr-10',
        notes: 'Contradiction moves into ground and conditioned genesis.',
      },
    ],
    contradictionGraph: {
      root: 'universal-smoke-fire',
      markedSubgraphs: [
        {
          id: 'smoke-industrial',
          condition: 'source = machineExhaust',
          resolutionRule: 'block fire inference',
        },
        {
          id: 'smoke-organic',
          condition: 'source = combustion',
          resolutionRule: 'allow fire inference',
        },
      ],
    },
    handoff: {
      toMorphShape: true,
      reason: 'Foundation triad compiled; handoff to ground morph synthesis.',
    },
  });
}

export async function compileMorphShapePrototype(
  contextShape: ContextShapePrototype,
): Promise<MorphShapePrototype> {
  const groundRecord = await compileGroundMorphStateRecord();
  const unitySeed = await compileReflectiveConsciousnessUnitySeed();

  const concreteGround = groundRecord['con-15'] ?? unitySeed;
  if (!concreteGround) {
    throw new Error(
      'Missing con-15 ground morph state for facticity synthesis.',
    );
  }

  return morphShapePrototypeSchema.parse({
    id: 'morphshape-conditioned-genesis-1',
    fromContextShapeId: contextShape.id,
    conditionStateId: concreteGround.groundStateId,
    conditionedGenesis: {
      allConditionsAtHand: concreteGround.morphState.allConditionsAtHand,
      concreteExistenceReady: concreteGround.morphState.concreteExistenceReady,
      transitionTarget:
        concreteGround.morphState.transitionTarget ?? 'existence-1',
    },
    subgraphActions: [
      {
        action: 'revise',
        subgraphId: 'smoke-industrial',
        rationale: 'Resolve contradiction by restricting inference context',
      },
      {
        action: 'attach',
        subgraphId: 'smoke-organic',
        rationale: 'Preserve validated combustion inference path',
      },
    ],
  });
}

export async function compileFactStorePrototype(): Promise<FactStorePrototype> {
  const reflectionShapes = await compileEssenceReflectionFormShapes();
  const formShape = reflectionShapes.find(
    (shape) => shape.refStateId === 'ref-14',
  );
  if (!formShape) {
    throw new Error(
      'Missing compiled reflection state ref-14 for FactStore synthesis.',
    );
  }

  const contextShape = await compileContextShapePrototype();
  const morphShape = await compileMorphShapePrototype(contextShape);

  return factStorePrototypeSchema.parse({
    id: 'factstore-prototype-1',
    formShape,
    contextShape,
    morphShape,
    facticity: {
      state: morphShape.conditionedGenesis.concreteExistenceReady
        ? 'concrete'
        : morphShape.conditionedGenesis.allConditionsAtHand
          ? 'conditioned'
          : 'potential',
      evidence: [
        morphShape.conditionStateId,
        `allConditions.atHand=${String(morphShape.conditionedGenesis.allConditionsAtHand)}`,
        morphShape.conditionedGenesis.transitionTarget,
      ],
      lastTransition: `${morphShape.conditionStateId} -> ${morphShape.conditionedGenesis.transitionTarget}`,
    },
  });
}

export async function compileFactStorePrototypeBundle(): Promise<{
  contextShape: ContextShapePrototype;
  morphShape: MorphShapePrototype;
  factStore: FactStorePrototype;
}> {
  const contextShape = await compileContextShapePrototype();
  const morphShape = await compileMorphShapePrototype(contextShape);
  const factStore = await compileFactStorePrototype();
  return { contextShape, morphShape, factStore };
}

export type ContextDetermination = z.infer<typeof contextDeterminationSchema>;
export type ContextShapePrototype = z.infer<typeof contextShapePrototypeSchema>;
export type MorphShapePrototype = z.infer<typeof morphShapePrototypeSchema>;
export type FactStorePrototype = z.infer<typeof factStorePrototypeSchema>;
