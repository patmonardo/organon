import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { PropertyShapeSchema, type PropertyShapeRepo } from '@schema/property';
import { loadDialecticIR } from '@relative/form/dialectic/generated/registry';

export type WorldPropertyStateId =
  | 'wld-1'
  | 'wld-4'
  | 'wld-5'
  | 'wld-7'
  | 'law-1'
  | 'law-9'
  | 'law-13'
  | 'disa-1'
  | 'disa-5'
  | 'disa-6'
  | 'disa-7';

export const WORLD_IR_KEY =
  '@relative/essence/appearance/world/world-ir#worldIR';
export const LAW_IR_KEY = '@relative/essence/appearance/world/law-ir#lawIR';
export const DISAPPEARANCE_IR_KEY =
  '@relative/essence/appearance/world/disappearance-ir#disappearanceIR';

function asWorldPropertyStateId(id: string): WorldPropertyStateId | undefined {
  if (
    id === 'wld-1' ||
    id === 'wld-4' ||
    id === 'wld-5' ||
    id === 'wld-7' ||
    id === 'law-1' ||
    id === 'law-9' ||
    id === 'law-13' ||
    id === 'disa-1' ||
    id === 'disa-5' ||
    id === 'disa-6' ||
    id === 'disa-7'
  ) {
    return id;
  }
  return undefined;
}

function toUniversality(state: DialecticState): 'necessary' | 'conditional' {
  return state.forces?.some((force) => force.type === 'contradiction')
    ? 'conditional'
    : 'necessary';
}

export function compileWorldStateToPropertyShape(
  state: DialecticState,
  irId = 'unknown-ir',
): PropertyShapeRepo | undefined {
  const stateId = asWorldPropertyStateId(state.id);
  if (!stateId) {
    return undefined;
  }

  const universality = toUniversality(state);

  return PropertyShapeSchema.parse({
    id: `propertyshape-${state.id}`,
    type: 'appearance.world.property',
    name: state.title,
    state: {
      phase: state.phase,
      sourceStateId: state.id,
    },
    signature: {
      concept: state.concept,
      irId,
      transitionTargets: state.nextStates ?? [],
    },
    facets: {
      dialecticState: state,
      phase: state.phase,
      law: {
        invariants: state.invariants.map((invariant) => ({
          id: invariant.id,
          constraint: invariant.constraint,
          predicate: invariant.predicate,
          universality,
        })),
        universality,
      },
      facticity: {
        grounds: state.moments.map((moment) => moment.name),
        conditions: (state.transitions ?? []).map(
          (transition) =>
            `${transition.from}->${transition.to}:${transition.mechanism}`,
        ),
        evidence: state.moments.map((moment) => ({
          name: moment.name,
          definition: moment.definition,
          type: moment.type,
        })),
      },
      mediates: {
        fromEntities: [state.id],
        toAspects: state.nextStates ?? [],
      },
      context: {
        sourceIrId: irId,
        sourceStateId: state.id,
        evaluateAgainstFactStore: true,
        provenance: state.provenance,
      },
    },
    status: 'compiled',
    tags: ['appearance', 'property', 'form-compiler'],
    meta: {
      compiler: 'compile-world-property-shape',
      sourceIrId: irId,
    },
  });
}

export function compileWorldIRToPropertyShapes(
  ir: DialecticIR,
): PropertyShapeRepo[] {
  return ir.states
    .map((state) => compileWorldStateToPropertyShape(state, ir.id))
    .filter((shape): shape is PropertyShapeRepo => Boolean(shape));
}

export async function compileWorldPropertyShapes(): Promise<
  PropertyShapeRepo[]
> {
  const [worldIR, lawIR, disappearanceIR] = await Promise.all([
    loadDialecticIR(WORLD_IR_KEY),
    loadDialecticIR(LAW_IR_KEY),
    loadDialecticIR(DISAPPEARANCE_IR_KEY),
  ]);

  return [
    ...compileWorldIRToPropertyShapes(worldIR),
    ...compileWorldIRToPropertyShapes(lawIR),
    ...compileWorldIRToPropertyShapes(disappearanceIR),
  ];
}

export async function compileWorldPropertyShapeRecord(): Promise<
  Record<WorldPropertyStateId, PropertyShapeRepo>
> {
  const compiled = await compileWorldPropertyShapes();
  return compiled.reduce(
    (acc, shape) => {
      const maybeStateId =
        typeof shape.state === 'object' && shape.state
          ? (shape.state as Record<string, unknown>).sourceStateId
          : undefined;
      if (typeof maybeStateId === 'string') {
        const typedStateId = asWorldPropertyStateId(maybeStateId);
        if (typedStateId) {
          acc[typedStateId] = shape;
        }
      }
      return acc;
    },
    {} as Record<WorldPropertyStateId, PropertyShapeRepo>,
  );
}
