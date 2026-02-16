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

export type FoundationStateId =
  | 'idn-3'
  | 'idn-4'
  | 'idn-6'
  | 'diff-1'
  | 'diff-5'
  | 'diff-8'
  | 'diff-11'
  | 'diff-13'
  | 'ctr-1'
  | 'ctr-4'
  | 'ctr-5'
  | 'ctr-10';

export const foundationDeterminationKindSchema = z.enum([
  'identity',
  'difference',
  'contradiction',
]);

export const foundationContextShapeSchema = z.object({
  id: z.string(),
  foundationStateId: z.string(),
  determinationKind: foundationDeterminationKindSchema,
  concept: z.string(),
  title: z.string(),
  phase: z.literal('reflection'),
  moments: z.array(MomentSchema),
  invariants: z.array(InvariantSchema),
  forces: z.array(ForceSchema),
  transitions: z.array(TransitionSchema),
  provenance: DialecticProvenanceSchema,
  handoff: z.object({
    nextState: z.string().optional(),
    toMorphCandidate: z.boolean().default(false),
    note: z.string().optional(),
  }),
});

export type FoundationContextShape = z.infer<
  typeof foundationContextShapeSchema
>;

export const IDENTITY_IR_KEY =
  '@relative/essence/reflection/foundation/identity-ir#identityIR';
export const DIFFERENCE_IR_KEY =
  '@relative/essence/reflection/foundation/difference-ir#differenceIR';
export const CONTRADICTION_IR_KEY =
  '@relative/essence/reflection/foundation/contradiction-ir#contradictionIR';

const foundationStateKindByPrefix: Record<
  string,
  z.infer<typeof foundationDeterminationKindSchema>
> = {
  idn: 'identity',
  diff: 'difference',
  ctr: 'contradiction',
};

function toDeterminationKind(
  stateId: string,
): z.infer<typeof foundationDeterminationKindSchema> | undefined {
  const prefix = stateId.split('-')[0];
  return foundationStateKindByPrefix[prefix];
}

function isMorphCandidate(stateId: string): boolean {
  return stateId === 'ctr-10';
}

export function compileFoundationStateToContextShape(
  state: DialecticState,
): FoundationContextShape | undefined {
  const determinationKind = toDeterminationKind(state.id);
  if (!determinationKind) {
    return undefined;
  }

  const nextState = state.nextStates?.[0];

  return foundationContextShapeSchema.parse({
    id: `contextshape-${state.id}`,
    foundationStateId: state.id,
    determinationKind,
    concept: state.concept,
    title: state.title,
    phase: state.phase,
    moments: state.moments,
    invariants: state.invariants,
    forces: state.forces ?? [],
    transitions: state.transitions ?? [],
    provenance: state.provenance,
    handoff: {
      nextState,
      toMorphCandidate: isMorphCandidate(state.id),
      note:
        determinationKind === 'contradiction'
          ? 'Contradiction-to-ground resolution path for Morph conditioned genesis.'
          : 'Feeds determination context before contradiction/ground realization.',
    },
  });
}

export function compileFoundationIRToContextShapes(
  ir: DialecticIR,
): FoundationContextShape[] {
  return ir.states
    .map((state) => compileFoundationStateToContextShape(state))
    .filter((shape): shape is FoundationContextShape => Boolean(shape));
}

export async function compileFoundationContextShapes(): Promise<
  FoundationContextShape[]
> {
  const [identityIR, differenceIR, contradictionIR] = await Promise.all([
    loadDialecticIR(IDENTITY_IR_KEY),
    loadDialecticIR(DIFFERENCE_IR_KEY),
    loadDialecticIR(CONTRADICTION_IR_KEY),
  ]);

  const compiled = [
    ...compileFoundationIRToContextShapes(identityIR),
    ...compileFoundationIRToContextShapes(differenceIR),
    ...compileFoundationIRToContextShapes(contradictionIR),
  ];

  return compiled;
}

export async function compileFoundationContextShapeRecord(): Promise<
  Record<string, FoundationContextShape>
> {
  const compiled = await compileFoundationContextShapes();
  return Object.fromEntries(
    compiled.map((shape) => [shape.foundationStateId, shape]),
  );
}

export async function compileFoundationContextTriad(): Promise<{
  identity: FoundationContextShape[];
  difference: FoundationContextShape[];
  contradiction: FoundationContextShape[];
}> {
  const compiled = await compileFoundationContextShapes();
  return {
    identity: compiled.filter(
      (shape) => shape.determinationKind === 'identity',
    ),
    difference: compiled.filter(
      (shape) => shape.determinationKind === 'difference',
    ),
    contradiction: compiled.filter(
      (shape) => shape.determinationKind === 'contradiction',
    ),
  };
}
