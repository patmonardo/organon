import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { AspectShapeSchema, type AspectShapeRepo } from '@schema/aspect';
import { loadDialecticIR } from '@relative/form/dialectic/generated/registry';

export type RelationAspectStateId =
  | 'wlp-1'
  | 'wlp-5'
  | 'wlp-8'
  | 'oin-2'
  | 'oin-3'
  | 'oin-5'
  | 'fex-1'
  | 'fex-9'
  | 'fex-12';

export const WHOLE_PARTS_IR_KEY =
  '@relative/essence/appearance/relation/whole-parts-ir#wholePartsIR';
export const OUTER_INNER_IR_KEY =
  '@relative/essence/appearance/relation/outer-inner-ir#outerInnerIR';
export const FORCE_EXPRESSION_IR_KEY =
  '@relative/essence/appearance/relation/force-expression-ir#forceExpressionIR';

function asRelationAspectStateId(
  id: string,
): RelationAspectStateId | undefined {
  if (
    id === 'wlp-1' ||
    id === 'wlp-5' ||
    id === 'wlp-8' ||
    id === 'oin-2' ||
    id === 'oin-3' ||
    id === 'oin-5' ||
    id === 'fex-1' ||
    id === 'fex-9' ||
    id === 'fex-12'
  ) {
    return id;
  }
  return undefined;
}

function toAppearingMode(
  state: DialecticState,
): 'immanent' | 'externality' | 'reflection' | 'passover' {
  const mechanism = state.transitions?.[0]?.mechanism;
  if (mechanism === 'reflection') {
    return 'reflection';
  }
  if (mechanism === 'passover') {
    return 'passover';
  }
  if (mechanism === 'appearance') {
    return 'externality';
  }
  return 'immanent';
}

export function compileRelationStateToAspectShape(
  state: DialecticState,
  irId = 'unknown-ir',
): AspectShapeRepo | undefined {
  const stateId = asRelationAspectStateId(state.id);
  if (!stateId) {
    return undefined;
  }

  const relations = state.moments
    .filter((moment) => moment.relation || moment.relatedTo)
    .map((moment) => ({
      from: moment.name,
      to: moment.relatedTo,
      relation: moment.relation,
      type: moment.type,
    }));

  return AspectShapeSchema.parse({
    id: `aspectshape-${state.id}`,
    type: 'appearance.relation.aspect',
    name: state.title,
    state: {
      phase: state.phase,
      sourceStateId: state.id,
    },
    signature: {
      concept: state.concept,
      irId,
      nextStates: state.nextStates ?? [],
    },
    facets: {
      dialecticState: state,
      phase: state.phase,
      spectrum: {
        poles: state.moments.map((moment) => ({
          name: moment.name,
          definition: moment.definition,
          oppositeTo:
            moment.relation === 'opposite' ? moment.relatedTo : undefined,
        })),
        range: state.moments.length,
        dialectical: state.moments.some(
          (moment) =>
            moment.type === 'negation' ||
            moment.type === 'contradiction' ||
            moment.relation === 'opposite',
        ),
      },
      essentialRelation: {
        spectrum: state.moments.map((moment) => moment.name),
        connections: relations,
        appearing: {
          mode: toAppearingMode(state),
          triggers: (state.forces ?? []).map((force) => force.trigger),
          effects: (state.forces ?? []).map((force) => force.effect),
        },
        groundedIn: state.provenance.topicMapId,
      },
      relations,
      appearing: {
        mode: toAppearingMode(state),
        triggers: (state.forces ?? []).map((force) => force.trigger),
        effects: (state.forces ?? []).map((force) => force.effect),
      },
      constraints: state.invariants.map((invariant) => ({
        id: invariant.id,
        constraint: invariant.constraint,
        predicate: invariant.predicate,
      })),
      context: {
        sourceIrId: irId,
        sourceStateId: state.id,
        evaluateAgainstFactStore: true,
        provenance: state.provenance,
      },
    },
    status: 'compiled',
    tags: ['appearance', 'aspect', 'relation', 'form-compiler'],
    meta: {
      compiler: 'compile-relation-aspect-shape',
      sourceIrId: irId,
    },
  });
}

export function compileRelationIRToAspectShapes(
  ir: DialecticIR,
): AspectShapeRepo[] {
  return ir.states
    .map((state) => compileRelationStateToAspectShape(state, ir.id))
    .filter((shape): shape is AspectShapeRepo => Boolean(shape));
}

export async function compileRelationAspectShapes(): Promise<
  AspectShapeRepo[]
> {
  const [wholePartsIR, outerInnerIR, forceExpressionIR] = await Promise.all([
    loadDialecticIR(WHOLE_PARTS_IR_KEY),
    loadDialecticIR(OUTER_INNER_IR_KEY),
    loadDialecticIR(FORCE_EXPRESSION_IR_KEY),
  ]);

  return [
    ...compileRelationIRToAspectShapes(wholePartsIR),
    ...compileRelationIRToAspectShapes(outerInnerIR),
    ...compileRelationIRToAspectShapes(forceExpressionIR),
  ];
}

export async function compileRelationAspectShapeRecord(): Promise<
  Record<RelationAspectStateId, AspectShapeRepo>
> {
  const compiled = await compileRelationAspectShapes();
  return compiled.reduce(
    (acc, shape) => {
      const maybeStateId =
        typeof shape.state === 'object' && shape.state
          ? (shape.state as Record<string, unknown>).sourceStateId
          : undefined;
      if (typeof maybeStateId === 'string') {
        const typedStateId = asRelationAspectStateId(maybeStateId);
        if (typedStateId) {
          acc[typedStateId] = shape;
        }
      }
      return acc;
    },
    {} as Record<RelationAspectStateId, AspectShapeRepo>,
  );
}
