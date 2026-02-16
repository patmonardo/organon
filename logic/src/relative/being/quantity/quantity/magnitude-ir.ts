import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { CONTINUOUS_DISCRETE_MAGNITUDE_TOPIC_MAP } from './sources/magnitude-topic-map';

const state1: DialecticState = {
  id: 'magnitude-1',
  title: 'Magnitude as differentiated quantity',
  concept: 'MagnitudeIntro',
  phase: 'quantity',
  moments: [
    {
      name: 'continuousMagnitude',
      definition: 'Quantity first appears as continuity, compact unity of many',
      type: 'determination',
    },
    {
      name: 'discreteMomentImplicit',
      definition:
        'This continuity already contains discreteness as its own moment',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'continuousMagnitude',
    },
  ],
  invariants: [
    {
      id: 'magnitude-1-inv-1',
      constraint: 'each moment remains the whole quantity in its form',
      predicate: 'momentAsWhole(continuity, discreteness)',
    },
    {
      id: 'magnitude-1-inv-2',
      constraint: 'continuity does not abolish discreteness, but holds it',
      predicate: 'holdsWithin(continuity, discreteness)',
    },
  ],
  forces: [
    {
      id: 'magnitude-1-force-1',
      description:
        'Immediacy of continuity must be negated into explicit discreteness',
      type: 'negation',
      trigger: 'immediacy.sublated = true',
      effect: 'discreteMagnitude.explicit = true',
      targetState: 'magnitude-4',
    },
  ],
  transitions: [
    {
      id: 'magnitude-1-trans-1',
      from: 'magnitude-1',
      to: 'magnitude-4',
      mechanism: 'negation',
      description: 'From continuous magnitude to discrete magnitude',
    },
  ],
  nextStates: ['magnitude-4'],
  previousStates: ['quantity-pure-unity'],
  provenance: {
    topicMapId: 'quantity-continuous-discrete-intro',
    lineRange: { start: 3, end: 23 },
    section: 'B. CONTINUOUS AND DISCRETE MAGNITUDE',
    order: 1,
  },
  description: CONTINUOUS_DISCRETE_MAGNITUDE_TOPIC_MAP.entries[0]?.description,
  keyPoints: CONTINUOUS_DISCRETE_MAGNITUDE_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'magnitude-4',
  title: 'Discrete magnitude as continuity of ones',
  concept: 'DiscreteMagnitude',
  phase: 'quantity',
  moments: [
    {
      name: 'discontinuousOutsideness',
      definition:
        'Discrete magnitude appears as broken outsideness-of-one-another',
      type: 'determination',
    },
    {
      name: 'implicitContinuityOfOnes',
      definition:
        'Yet these ones share the same unity, so discreteness is itself continuous',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'discontinuousOutsideness',
    },
  ],
  invariants: [
    {
      id: 'magnitude-4-inv-1',
      constraint: 'discrete magnitude remains quantitative unity of many ones',
      predicate: 'unityOfManyOnes(discreteMagnitude)',
    },
    {
      id: 'magnitude-4-inv-2',
      constraint: 'outsideness remains within common quantitative form',
      predicate: 'withinSameForm(outsideness)',
    },
  ],
  forces: [
    {
      id: 'magnitude-4-force-1',
      description: 'Discrete magnitude requires explicit limiting as quantum',
      type: 'sublation',
      trigger: 'discreteness.requiresLimit = true',
      effect: 'quantumForm.posited = true',
      targetState: 'quantity-discrete-magnitude',
    },
  ],
  transitions: [
    {
      id: 'magnitude-4-trans-1',
      from: 'magnitude-4',
      to: 'quantity-discrete-magnitude',
      mechanism: 'sublation',
      description: 'From discrete magnitude to its limiting form',
    },
  ],
  nextStates: ['quantity-discrete-magnitude'],
  previousStates: ['magnitude-1'],
  provenance: {
    topicMapId: 'quantity-discrete-magnitude',
    lineRange: { start: 34, end: 57 },
    section: 'B. CONTINUOUS AND DISCRETE MAGNITUDE',
    order: 2,
  },
  description: CONTINUOUS_DISCRETE_MAGNITUDE_TOPIC_MAP.entries[2]?.description,
  keyPoints: CONTINUOUS_DISCRETE_MAGNITUDE_TOPIC_MAP.entries[2]?.keyPoints,
};

const state3: DialecticState = {
  id: 'quantity-discrete-magnitude',
  title: 'Magnitude transitions to limiting quantity',
  concept: 'MagnitudeToQuantumThreshold',
  phase: 'quantity',
  moments: [
    {
      name: 'limitEmergence',
      definition:
        'Magnitude now requires one as explicit limit within quantity',
      type: 'determination',
    },
    {
      name: 'quantumThreshold',
      definition:
        'Continuous and discrete difference becomes indifferent in the emerging quantum form',
      type: 'process',
      relation: 'transitions',
      relatedTo: 'limitEmergence',
    },
  ],
  invariants: [
    {
      id: 'quantity-discrete-magnitude-inv-1',
      constraint: 'magnitude tends toward explicit boundedness',
      predicate: 'tendsTowardBoundedness(magnitude)',
    },
  ],
  forces: [
    {
      id: 'quantity-discrete-magnitude-force-1',
      description: 'Magnitude passes over to the chapter of limiting quantity',
      type: 'passover',
      trigger: 'limit.formallyPosited = true',
      effect: 'limitingQuantityChapter.initiated = true',
      targetState: 'quantity-limiting-intro',
    },
  ],
  transitions: [
    {
      id: 'quantity-discrete-magnitude-trans-1',
      from: 'quantity-discrete-magnitude',
      to: 'quantity-limiting-intro',
      mechanism: 'passover',
      description: 'From magnitude to limiting quantity',
    },
  ],
  nextStates: ['quantity-limiting-intro'],
  previousStates: ['magnitude-4'],
  provenance: {
    topicMapId: 'quantity-discrete-magnitude',
    lineRange: { start: 34, end: 57 },
    section: 'B. CONTINUOUS AND DISCRETE MAGNITUDE',
    order: 3,
  },
  description: CONTINUOUS_DISCRETE_MAGNITUDE_TOPIC_MAP.entries[2]?.description,
  keyPoints: CONTINUOUS_DISCRETE_MAGNITUDE_TOPIC_MAP.entries[2]?.keyPoints,
};

export const magnitudeIR: DialecticIR = {
  id: 'magnitude-ir',
  title: 'Magnitude IR: Continuous and Discrete Magnitude',
  section: 'BEING - QUANTITY - B. Continuous and Discrete Magnitude',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'continuous-discrete-magnitude.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'magnitude-1': 'quantity',
      'magnitude-4': 'quantity',
      'quantity-discrete-magnitude': 'quantity',
    },
  },
};

export const magnitudeStates = {
  'magnitude-1': state1,
  'magnitude-4': state2,
  'quantity-discrete-magnitude': state3,
};
