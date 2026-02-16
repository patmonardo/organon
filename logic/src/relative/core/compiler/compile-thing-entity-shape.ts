import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { EntityShapeSchema, type EntityShapeRepo } from '@schema/entity';
import { loadDialecticIR } from '@relative/form/dialectic/generated/registry';

export type ThingEntityStateId =
  | 'thg-1'
  | 'thg-7'
  | 'thg-9'
  | 'thg-12'
  | 'thg-15'
  | 'mat-1'
  | 'mat-3'
  | 'mat-7'
  | 'dis-1'
  | 'dis-3'
  | 'dis-5'
  | 'dis-6';

export const THING_IR_KEY =
  '@relative/essence/appearance/thing/thing-ir#thingIR';
export const MATTER_IR_KEY =
  '@relative/essence/appearance/thing/matter-ir#matterIR';
export const DISSOLUTION_IR_KEY =
  '@relative/essence/appearance/thing/dissolution-ir#dissolutionIR';

function asThingEntityStateId(id: string): ThingEntityStateId | undefined {
  if (
    id === 'thg-1' ||
    id === 'thg-7' ||
    id === 'thg-9' ||
    id === 'thg-12' ||
    id === 'thg-15' ||
    id === 'mat-1' ||
    id === 'mat-3' ||
    id === 'mat-7' ||
    id === 'dis-1' ||
    id === 'dis-3' ||
    id === 'dis-5' ||
    id === 'dis-6'
  ) {
    return id;
  }
  return undefined;
}

function thingEntityKindFromStateId(
  stateId: ThingEntityStateId,
): 'thing' | 'matter' | 'dissolution' {
  if (stateId.startsWith('thg-')) {
    return 'thing';
  }
  if (stateId.startsWith('mat-')) {
    return 'matter';
  }
  return 'dissolution';
}

function toEntityShape(state: DialecticState, irId: string): EntityShapeRepo {
  const stateId = asThingEntityStateId(state.id);
  if (!stateId) {
    throw new Error(`Unsupported thing-entity state: ${state.id}`);
  }

  const kind = thingEntityKindFromStateId(stateId);

  return EntityShapeSchema.parse({
    id: `entityshape-${state.id}`,
    type: 'appearance.thing.entity',
    formId: `appearance/form/${state.id}`,
    name: state.title,
    description: state.description,
    values: {},
    signature: {
      concept: state.concept,
      phase: state.phase,
      irId,
      nextStates: state.nextStates ?? [],
      previousStates: state.previousStates ?? [],
    },
    facets: {
      dialecticState: state,
      phase: state.phase,
      moments: state.moments,
      invariants: state.invariants,
      context: {
        sourceIrId: irId,
        sourceStateId: state.id,
        entityKind: kind,
        evaluateAgainstFactStore: true,
        provenance: state.provenance,
      },
    },
    status: 'compiled',
    tags: ['appearance', 'entity', 'form-compiler', kind],
    meta: {
      compiler: 'compile-thing-entity-shape',
      sourceIrId: irId,
    },
  });
}

export function compileThingStateToEntityShape(
  state: DialecticState,
  irId = 'unknown-ir',
): EntityShapeRepo | undefined {
  if (!asThingEntityStateId(state.id)) {
    return undefined;
  }
  return toEntityShape(state, irId);
}

export function compileThingIRToEntityShapes(
  ir: DialecticIR,
): EntityShapeRepo[] {
  return ir.states
    .map((state) => compileThingStateToEntityShape(state, ir.id))
    .filter((shape): shape is EntityShapeRepo => Boolean(shape));
}

export async function compileThingEntityShapes(): Promise<EntityShapeRepo[]> {
  const [thingIR, matterIR, dissolutionIR] = await Promise.all([
    loadDialecticIR(THING_IR_KEY),
    loadDialecticIR(MATTER_IR_KEY),
    loadDialecticIR(DISSOLUTION_IR_KEY),
  ]);

  return [
    ...compileThingIRToEntityShapes(thingIR),
    ...compileThingIRToEntityShapes(matterIR),
    ...compileThingIRToEntityShapes(dissolutionIR),
  ];
}

export async function compileThingEntityShapeRecord(): Promise<
  Record<ThingEntityStateId, EntityShapeRepo>
> {
  const compiled = await compileThingEntityShapes();
  return compiled.reduce(
    (acc, shape) => {
      const maybeStateId =
        typeof shape.facets === 'object' && shape.facets
          ? (shape.facets as Record<string, unknown>).context
          : undefined;
      const stateId =
        typeof maybeStateId === 'object' && maybeStateId
          ? (maybeStateId as Record<string, unknown>).sourceStateId
          : undefined;
      if (typeof stateId === 'string') {
        const typedStateId = asThingEntityStateId(stateId);
        if (typedStateId) {
          acc[typedStateId] = shape;
        }
      }
      return acc;
    },
    {} as Record<ThingEntityStateId, EntityShapeRepo>,
  );
}

export const thingEntityShapeSeed = EntityShapeSchema.parse({
  id: 'entityshape-seed-thg-1',
  type: 'appearance.thing.entity',
  formId: 'appearance/form/thg-1',
  name: 'Concrete existence — principle',
  description: 'Seed entity shape for Thing compiler bootstrap.',
  values: {},
  signature: {
    concept: 'ConcreteExistence',
    phase: 'appearance',
    irId: 'thing-ir',
    nextStates: ['thg-2'],
    previousStates: [],
  },
  facets: {
    phase: 'appearance',
    moments: [],
    invariants: [],
    context: {
      sourceIrId: 'thing-ir',
      sourceStateId: 'thg-1',
      entityKind: 'thing',
      evaluateAgainstFactStore: true,
      seed: true,
    },
  },
  status: 'seed',
  tags: ['appearance', 'entity', 'form-compiler', 'seed'],
  meta: {
    compiler: 'compile-thing-entity-shape',
    role: 'seed',
  },
});

export function getThingEntityShapeSeed(): EntityShapeRepo {
  return { ...thingEntityShapeSeed };
}
