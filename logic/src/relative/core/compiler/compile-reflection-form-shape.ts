import type { DialecticIR, DialecticState } from '@schema/dialectic';
import {
  reflectionFormShapeSchema,
  type ReflectionFormShape,
  reflectionFormShapeFixtureList,
} from './reflection-form-shape';
import { loadDialecticIR } from '@relative/form/dialectic/generated/registry';

export type ReflectionRefStateId = 'ref-2' | 'ref-7' | 'ref-10' | 'ref-14';

type ReflectionCompileSpec = {
  reflectionMode: ReflectionFormShape['reflectionMode'];
  cittaReflection: ReflectionFormShape['cittaReflection'];
  bhasaPhase: ReflectionFormShape['bhasa']['phase'];
  dhyanaAbiding: boolean;
  samadhiReady: boolean;
};

const reflectionCompileSpec: Record<
  ReflectionRefStateId,
  ReflectionCompileSpec
> = {
  'ref-2': {
    reflectionMode: 'movement',
    cittaReflection: 'movement-prelude',
    bhasaPhase: 'pratibhasa',
    dhyanaAbiding: false,
    samadhiReady: false,
  },
  'ref-7': {
    reflectionMode: 'positing',
    cittaReflection: 'positing',
    bhasaPhase: 'dharana',
    dhyanaAbiding: false,
    samadhiReady: false,
  },
  'ref-10': {
    reflectionMode: 'external',
    cittaReflection: 'external',
    bhasaPhase: 'dhyana',
    dhyanaAbiding: true,
    samadhiReady: false,
  },
  'ref-14': {
    reflectionMode: 'determining',
    cittaReflection: 'determining',
    bhasaPhase: 'dhyana',
    dhyanaAbiding: true,
    samadhiReady: true,
  },
};

export const ESSENCE_REFLECTION_IR_KEY =
  '@relative/essence/reflection/essence/reflection-ir#reflectionIR';

function asReflectionRefStateId(id: string): ReflectionRefStateId | undefined {
  if (id === 'ref-2' || id === 'ref-7' || id === 'ref-10' || id === 'ref-14') {
    return id;
  }
  return undefined;
}

export function compileReflectionStateToFormShape(
  state: DialecticState,
): ReflectionFormShape | undefined {
  const refStateId = asReflectionRefStateId(state.id);
  if (!refStateId) {
    return undefined;
  }

  const spec = reflectionCompileSpec[refStateId];
  const firstTransition = state.transitions?.[0];
  const firstForce = state.forces?.[0];

  return reflectionFormShapeSchema.parse({
    id: `formshape-${state.id}`,
    refStateId,
    reflectionMode: spec.reflectionMode,
    cittaReflection: spec.cittaReflection,
    bhasa: {
      phase: spec.bhasaPhase,
      pratyaharaTrace: state.moments.map((moment) => moment.name),
      dharanaTrigger: firstForce?.trigger,
      dhyanaAbiding: spec.dhyanaAbiding,
      samadhiSignal: {
        ready: spec.samadhiReady,
        reference: firstTransition?.to ?? 'ref-19',
      },
    },
    moments: state.moments,
    invariants: state.invariants.map((invariant) => ({
      id: invariant.id,
      constraint: invariant.constraint,
      predicate: invariant.predicate ?? '',
    })),
    forces: (state.forces ?? []).map((force) => ({
      id: force.id,
      description: force.description,
      type: force.type,
      trigger: force.trigger,
      effect: force.effect,
      targetState: force.targetState,
    })),
    transitions: (state.transitions ?? []).map((transition) => ({
      id: transition.id,
      from: transition.from,
      to: transition.to,
      mechanism: transition.mechanism,
      description: transition.description,
    })),
    provenance: {
      topicMapId: state.provenance.topicMapId,
      lineRange: state.provenance.lineRange,
      section: state.provenance.section ?? '2. Reflection',
      order: state.provenance.order ?? 0,
    },
  });
}

export function compileReflectionIRToFormShapes(
  ir: DialecticIR,
): ReflectionFormShape[] {
  return ir.states
    .map((state) => compileReflectionStateToFormShape(state))
    .filter((state): state is ReflectionFormShape => Boolean(state));
}

export function compileReflectionIRToFormShapeRecord(
  ir: DialecticIR,
): Record<ReflectionRefStateId, ReflectionFormShape> {
  const compiled = compileReflectionIRToFormShapes(ir);
  return compiled.reduce(
    (acc, formShape) => {
      acc[formShape.refStateId] = formShape;
      return acc;
    },
    {} as Record<ReflectionRefStateId, ReflectionFormShape>,
  );
}

export async function compileEssenceReflectionFormShapes(): Promise<
  ReflectionFormShape[]
> {
  const ir = await loadDialecticIR(ESSENCE_REFLECTION_IR_KEY);
  return compileReflectionIRToFormShapes(ir);
}

export async function compileEssenceReflectionFormShapeRecord(): Promise<
  Record<ReflectionRefStateId, ReflectionFormShape>
> {
  const ir = await loadDialecticIR(ESSENCE_REFLECTION_IR_KEY);
  return compileReflectionIRToFormShapeRecord(ir);
}

export function getReflectionFormShapeCompilerPrototype(): ReflectionFormShape[] {
  return [...reflectionFormShapeFixtureList];
}
