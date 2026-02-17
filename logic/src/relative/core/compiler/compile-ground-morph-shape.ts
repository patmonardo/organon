import { z } from 'zod';
import type { DialecticIR, DialecticState } from '@schema/dialectic';
import {
  MomentSchema,
  InvariantSchema,
  ForceSchema,
  TransitionSchema,
  DialecticProvenanceSchema,
} from '@schema/dialectic';
import { loadDialecticIR } from '@relative/form/dialectic/generated/registry';

export type GroundStateId =
  | 'abs-1'
  | 'abs-5'
  | 'abs-7'
  | 'abs-9'
  | 'abs-13'
  | 'det-1'
  | 'det-5'
  | 'det-9'
  | 'det-10'
  | 'det-14'
  | 'con-1'
  | 'con-4'
  | 'con-10'
  | 'con-13'
  | 'con-15';

export const groundMorphKindSchema = z.enum([
  'absolute-ground',
  'determinate-ground',
  'condition-ground',
]);

export const groundMorphShapeSchema = z.object({
  id: z.string(),
  groundStateId: z.string(),
  groundKind: groundMorphKindSchema,
  concept: z.string(),
  title: z.string(),
  phase: z.literal('reflection'),
  moments: z.array(MomentSchema),
  invariants: z.array(InvariantSchema),
  forces: z.array(ForceSchema),
  transitions: z.array(TransitionSchema),
  provenance: DialecticProvenanceSchema,
  morphState: z.object({
    conditionedGenesis: z.boolean(),
    allConditionsAtHand: z.boolean().default(false),
    concreteExistenceReady: z.boolean().default(false),
    transitionTarget: z.string().optional(),
  }),
  handoff: z.object({
    nextState: z.string().optional(),
    factStoreCandidate: z.boolean().default(false),
    reflectiveUnityCandidate: z.boolean().default(false),
    note: z.string().optional(),
  }),
});

export type GroundMorphShape = z.infer<typeof groundMorphShapeSchema>;

export const ABSOLUTE_GROUND_IR_KEY =
  '@relative/essence/reflection/ground/absolute-ir#absoluteIR';
export const DETERMINATE_GROUND_IR_KEY =
  '@relative/essence/reflection/ground/determinate-ir#determinateIR';
export const CONDITION_GROUND_IR_KEY =
  '@relative/essence/reflection/ground/condition-ir#conditionIR';

const groundStateKindByPrefix: Record<
  string,
  z.infer<typeof groundMorphKindSchema>
> = {
  abs: 'absolute-ground',
  det: 'determinate-ground',
  con: 'condition-ground',
};

function toGroundKind(
  stateId: string,
): z.infer<typeof groundMorphKindSchema> | undefined {
  const prefix = stateId.split('-')[0];
  return groundStateKindByPrefix[prefix];
}

function isFactStoreCandidate(stateId: string): boolean {
  return stateId === 'con-15';
}

function isReflectiveUnityCandidate(stateId: string): boolean {
  return stateId === 'con-15';
}

export function compileGroundStateToMorphShape(
  state: DialecticState,
): GroundMorphShape | undefined {
  const groundKind = toGroundKind(state.id);
  if (!groundKind) {
    return undefined;
  }

  const nextState = state.nextStates?.[0];

  return groundMorphShapeSchema.parse({
    id: `morphstate-${state.id}`,
    groundStateId: state.id,
    groundKind,
    concept: state.concept,
    title: state.title,
    phase: state.phase,
    moments: state.moments,
    invariants: state.invariants,
    forces: state.forces ?? [],
    transitions: state.transitions ?? [],
    provenance: state.provenance,
    morphState: {
      conditionedGenesis:
        groundKind === 'condition-ground' || state.id.startsWith('det-'),
      allConditionsAtHand: state.id === 'con-15',
      concreteExistenceReady: state.id === 'con-15',
      transitionTarget: nextState,
    },
    handoff: {
      nextState,
      factStoreCandidate: isFactStoreCandidate(state.id),
      reflectiveUnityCandidate: isReflectiveUnityCandidate(state.id),
      note: isFactStoreCandidate(state.id)
        ? 'Concrete existence reached; seed FactStore and ReflectiveConsciousness unity.'
        : 'Ground mediation continues toward concrete existence.',
    },
  });
}

export function compileGroundIRToMorphShapes(
  ir: DialecticIR,
): GroundMorphShape[] {
  return ir.states
    .map((state) => compileGroundStateToMorphShape(state))
    .filter((shape): shape is GroundMorphShape => Boolean(shape));
}

export async function compileGroundMorphShapes(): Promise<GroundMorphShape[]> {
  const [absoluteIR, determinateIR, conditionIR] = await Promise.all([
    loadDialecticIR(ABSOLUTE_GROUND_IR_KEY),
    loadDialecticIR(DETERMINATE_GROUND_IR_KEY),
    loadDialecticIR(CONDITION_GROUND_IR_KEY),
  ]);

  return [
    ...compileGroundIRToMorphShapes(absoluteIR),
    ...compileGroundIRToMorphShapes(determinateIR),
    ...compileGroundIRToMorphShapes(conditionIR),
  ];
}

export async function compileGroundMorphShapeRecord(): Promise<
  Record<string, GroundMorphShape>
> {
  const compiled = await compileGroundMorphShapes();
  return Object.fromEntries(
    compiled.map((state) => [state.groundStateId, state]),
  );
}

export async function compileReflectiveConsciousnessUnitySeed(): Promise<
  GroundMorphShape | undefined
> {
  const compiled = await compileGroundMorphShapes();
  return compiled.find((state) => state.handoff.reflectiveUnityCandidate);
}
