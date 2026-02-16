import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { METHOD_ADVANCE_TOPIC_MAP } from './sources/method-advance-topic-map';

const state1: DialecticState = {
  id: 'method-advance-1',
  title: 'Advance as analytic-synthetic dialectical movement',
  concept: 'DialecticalAdvance',
  phase: 'subject',
  moments: [
    {
      name: 'analyticExtraction',
      definition:
        'Method analytically finds determinations immanent in the universal subject matter itself',
      type: 'reflection',
    },
    {
      name: 'syntheticOthering',
      definition:
        'The same universal synthetically proves itself as other and thereby generates dialectical judgment',
      type: 'mediation',
      relation: 'transitions',
      relatedTo: 'analyticExtraction',
    },
  ],
  invariants: [
    {
      id: 'method-advance-1-inv-1',
      constraint:
        'absolute method derives determinacy from immanent principle not external imposition',
      predicate: 'immanentDerivation(method, determinacy)',
    },
    {
      id: 'method-advance-1-inv-2',
      constraint: 'dialectical moment is necessary to reason',
      predicate: 'necessaryToReason(dialecticalMoment)',
    },
  ],
  forces: [
    {
      id: 'method-advance-1-force-1',
      description:
        'Immanent differentiation turns into explicit negativity as method’s turning point',
      type: 'negation',
      trigger: 'universal.determinesItselfAsOther = true',
      effect: 'negativity.explicit = true',
      targetState: 'method-advance-3',
    },
  ],
  transitions: [
    {
      id: 'method-advance-1-trans-1',
      from: 'method-advance-1',
      to: 'method-advance-3',
      mechanism: 'negation',
      description:
        'From analytic-synthetic split to explicit dialectical negativity',
    },
  ],
  nextStates: ['method-advance-3'],
  previousStates: ['method-beginning-3'],
  provenance: {
    topicMapId: 'method-advance-1-analytic-synthetic',
    lineRange: { start: 1, end: 99 },
    section: 'The Advance',
    order: 1,
  },
  description: METHOD_ADVANCE_TOPIC_MAP.entries[0]?.description,
  keyPoints: METHOD_ADVANCE_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'method-advance-3',
  title: 'Negativity and the third as truth',
  concept: 'TriplicitTruth',
  phase: 'subject',
  moments: [
    {
      name: 'negativeSelfReference',
      definition:
        'Negativity is method’s inner source and dialectical soul, the turning point of movement',
      type: 'negation',
    },
    {
      name: 'thirdAsConcreteSubject',
      definition:
        'The third sublates mediation and realizes concept as singular concrete subject',
      type: 'sublation',
      relation: 'contains',
      relatedTo: 'negativeSelfReference',
    },
  ],
  invariants: [
    {
      id: 'method-advance-3-inv-1',
      constraint:
        'the second preserves the first within negation and not as empty nullity',
      predicate: 'preservesInNegation(second, first)',
    },
    {
      id: 'method-advance-3-inv-2',
      constraint:
        'triplicity expresses universal rational form as syllogistic movement',
      predicate: 'syllogisticUniversalForm(triplicity)',
    },
  ],
  forces: [
    {
      id: 'method-advance-3-force-1',
      description:
        'The realized third re-forms itself as a new beginning in system expansion',
      type: 'passover',
      trigger: 'third.realizedAsTruth = true',
      effect: 'newBeginning.form = immediacy',
      targetState: 'method-advance-6',
    },
  ],
  transitions: [
    {
      id: 'method-advance-3-trans-1',
      from: 'method-advance-3',
      to: 'method-advance-6',
      mechanism: 'passover',
      description: 'From triplicit truth to circular progression of method',
    },
  ],
  nextStates: ['method-advance-6'],
  previousStates: ['method-advance-1'],
  provenance: {
    topicMapId: 'method-advance-3-negativity-triplicity',
    lineRange: { start: 226, end: 410 },
    section: 'The Advance',
    order: 2,
  },
  description: METHOD_ADVANCE_TOPIC_MAP.entries[2]?.description,
  keyPoints: METHOD_ADVANCE_TOPIC_MAP.entries[2]?.keyPoints,
};

const state3: DialecticState = {
  id: 'method-advance-6',
  title: 'Circle of circles and return to beginning',
  concept: 'MethodicCircle',
  phase: 'subject',
  moments: [
    {
      name: 'forwardRetrogressiveUnity',
      definition:
        'Method advances into richer determinacy while simultaneously returning toward the grounding beginning',
      type: 'reflection',
    },
    {
      name: 'circleOfCircles',
      definition:
        'Logic coils into a circle of circles where fulfilled concept is concrete intensive totality',
      type: 'sublation',
      relation: 'passesOver',
      relatedTo: 'forwardRetrogressiveUnity',
    },
  ],
  invariants: [
    {
      id: 'method-advance-6-inv-1',
      constraint:
        'universal remains self-identical across particularization and enrichment',
      predicate: 'selfIdenticalAcrossParticularization(universal)',
    },
    {
      id: 'method-advance-6-inv-2',
      constraint:
        'result as new beginning preserves accumulated determination in compressed form',
      predicate: 'compressedPreservation(result, determinations)',
    },
  ],
  forces: [
    {
      id: 'method-advance-6-force-1',
      description:
        'Completed speculative circle returns to the logical beginning of being',
      type: 'passover',
      trigger: 'method.circleCompleted = true',
      effect: 'logicalBeginning.reopened = true',
      targetState: 'being-1',
    },
  ],
  transitions: [
    {
      id: 'method-advance-6-trans-1',
      from: 'method-advance-6',
      to: 'being-1',
      mechanism: 'passover',
      description: 'From circle of circles back to being as renewed beginning',
    },
  ],
  nextStates: ['being-1'],
  previousStates: ['method-advance-3'],
  provenance: {
    topicMapId: 'method-advance-6-circle-circles',
    lineRange: { start: 711, end: 735 },
    section: 'The Advance',
    order: 3,
  },
  description: METHOD_ADVANCE_TOPIC_MAP.entries[5]?.description,
  keyPoints: METHOD_ADVANCE_TOPIC_MAP.entries[5]?.keyPoints,
};

export const methodAdvanceIR: DialecticIR = {
  id: 'method-advance-ir',
  title: 'Method Advance IR: Dialectical Movement, Third, Circle',
  section: 'CONCEPT - IDEA - C. Speculation - B. Method - 2. The Advance',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'method-advance.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'method-advance-1': 'subject',
      'method-advance-3': 'subject',
      'method-advance-6': 'subject',
    },
  },
};

export const methodAdvanceStates = {
  'method-advance-1': state1,
  'method-advance-3': state2,
  'method-advance-6': state3,
};
