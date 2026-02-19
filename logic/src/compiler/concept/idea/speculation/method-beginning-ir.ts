import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { METHOD_BEGINNING_TOPIC_MAP } from './sources/method-beginning-topic-map';

const state1: DialecticState = {
  id: 'method-beginning-1',
  title: 'Beginning as simple universal immediacy',
  concept: 'MethodicBeginning',
  phase: 'subject',
  moments: [
    {
      name: 'immediateUniversality',
      definition:
        'Beginning is abstract universality in thought, immediate self-reference equivalent to being',
      type: 'determination',
    },
    {
      name: 'supersensuousIntuiting',
      definition:
        'The beginning is not representational givenness but thought’s inner intuiting',
      type: 'immanence',
      relation: 'contains',
      relatedTo: 'immediateUniversality',
    },
  ],
  invariants: [
    {
      id: 'method-beginning-1-inv-1',
      constraint:
        'beginning requires no prior derivation as immediate universality',
      predicate: 'noPriorDerivation(immediateUniversality)',
    },
    {
      id: 'method-beginning-1-inv-2',
      constraint: 'being is the abstract form of this immediate self-reference',
      predicate: 'beingAsAbstractSelfReference(beginning)',
    },
  ],
  forces: [
    {
      id: 'method-beginning-1-force-1',
      description:
        'Immediate universality reveals deficiency and demands objective development',
      type: 'negation',
      trigger: 'beginning.isAbstractOnly = true',
      effect: 'deficiency.explicit = true',
      targetState: 'method-beginning-2',
    },
  ],
  transitions: [
    {
      id: 'method-beginning-1-trans-1',
      from: 'method-beginning-1',
      to: 'method-beginning-2',
      mechanism: 'negation',
      description:
        'From immediate beginning to deficient beginning requiring advance',
    },
  ],
  nextStates: ['method-beginning-2'],
  previousStates: ['absolute-idea-4'],
  provenance: {
    topicMapId: 'method-beginning-1-simple-universal',
    lineRange: { start: 1, end: 66 },
    section: 'The Beginning',
    order: 1,
  },
  description: METHOD_BEGINNING_TOPIC_MAP.entries[0]?.description,
  keyPoints: METHOD_BEGINNING_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'method-beginning-2',
  title: 'Deficient beginning and implicit absolute',
  concept: 'ImplicitAbsoluteInBeginning',
  phase: 'subject',
  moments: [
    {
      name: 'deficientUniversality',
      definition:
        'Beginning is only one moment of concept and thus marked by internal lack',
      type: 'contradiction',
    },
    {
      name: 'objectiveImpulse',
      definition:
        'Method’s immanent objective soul impels universality to determine itself for itself',
      type: 'mediation',
      relation: 'transitions',
      relatedTo: 'deficientUniversality',
    },
  ],
  invariants: [
    {
      id: 'method-beginning-2-inv-1',
      constraint:
        'every beginning is absolute only implicitly, not yet as posited concept',
      predicate: 'implicitNotPositedAbsolute(beginning)',
    },
    {
      id: 'method-beginning-2-inv-2',
      constraint:
        'advance is necessary for universality to become singular subjectivity',
      predicate: 'requiresAdvance(universality, singularSubject)',
    },
  ],
  forces: [
    {
      id: 'method-beginning-2-force-1',
      description:
        'Implicit totality manifests as concrete germ and purposive impulse',
      type: 'sublation',
      trigger: 'deficiency.turnedProductive = true',
      effect: 'concreteBeginning.emerges = true',
      targetState: 'method-beginning-3',
    },
  ],
  transitions: [
    {
      id: 'method-beginning-2-trans-1',
      from: 'method-beginning-2',
      to: 'method-beginning-3',
      mechanism: 'sublation',
      description:
        'From deficient universality to concrete totalized beginning',
    },
  ],
  nextStates: ['method-beginning-3'],
  previousStates: ['method-beginning-1'],
  provenance: {
    topicMapId: 'method-beginning-2-deficient-absolute',
    lineRange: { start: 68, end: 123 },
    section: 'The Beginning',
    order: 2,
  },
  description: METHOD_BEGINNING_TOPIC_MAP.entries[1]?.description,
  keyPoints: METHOD_BEGINNING_TOPIC_MAP.entries[1]?.keyPoints,
};

const state3: DialecticState = {
  id: 'method-beginning-3',
  title: 'Concrete totality as living germ of method',
  concept: 'ConcreteBeginning',
  phase: 'subject',
  moments: [
    {
      name: 'germinalTotality',
      definition:
        'Beginning is a concrete totality in itself, as germ or purposive impulse',
      type: 'process',
    },
    {
      name: 'realPossibility',
      definition:
        'Where subjectivity is not explicit, concrete concept appears as real possibility requiring external realization',
      type: 'appearance',
      relation: 'contains',
      relatedTo: 'germinalTotality',
    },
  ],
  invariants: [
    {
      id: 'method-beginning-3-inv-1',
      constraint:
        'concrete beginning contains totality before explicit positing of moments',
      predicate: 'containsUnpositedTotality(concreteBeginning)',
    },
    {
      id: 'method-beginning-3-inv-2',
      constraint:
        'cause as immediate necessity is insufficient without self-maintaining subjectivity',
      predicate: 'insufficientWithoutSubjectiveSelfMaintenance(cause)',
    },
  ],
  forces: [
    {
      id: 'method-beginning-3-force-1',
      description:
        'Concrete beginning advances by differentiating itself into analytic and synthetic moments',
      type: 'mediation',
      trigger: 'concreteTotality.selfDifferentiates = true',
      effect: 'methodAdvance.starts = true',
      targetState: 'method-advance-1',
    },
  ],
  transitions: [
    {
      id: 'method-beginning-3-trans-1',
      from: 'method-beginning-3',
      to: 'method-advance-1',
      mechanism: 'mediation',
      description: 'From concrete beginning to dialectical advance of method',
    },
  ],
  nextStates: ['method-advance-1'],
  previousStates: ['method-beginning-2'],
  provenance: {
    topicMapId: 'method-beginning-3-concrete-totality',
    lineRange: { start: 125, end: 153 },
    section: 'The Beginning',
    order: 3,
  },
  description: METHOD_BEGINNING_TOPIC_MAP.entries[2]?.description,
  keyPoints: METHOD_BEGINNING_TOPIC_MAP.entries[2]?.keyPoints,
};

export const methodBeginningIR: DialecticIR = {
  id: 'method-beginning-ir',
  title: 'Method Beginning IR: Immediate Universal, Deficiency, Concrete Germ',
  section: 'CONCEPT - IDEA - C. Speculation - B. Method - 1. The Beginning',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'method-beginning.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'method-beginning-1': 'subject',
      'method-beginning-2': 'subject',
      'method-beginning-3': 'subject',
    },
  },
};

export const methodBeginningStates = {
  'method-beginning-1': state1,
  'method-beginning-2': state2,
  'method-beginning-3': state3,
};
