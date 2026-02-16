import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { LIMITING_QUANTITY_TOPIC_MAP } from './sources/limiting-quantity-topic-map';

const state1: DialecticState = {
  id: 'quantity-limiting-intro',
  title: 'Discrete magnitude as quantum',
  concept: 'QuantumIntroduction',
  phase: 'quantity',
  moments: [
    {
      name: 'oneAsLimit',
      definition:
        'Discrete quantity is one magnitude whose determinateness is explicit limit',
      type: 'determination',
    },
    {
      name: 'quantumAsSomething',
      definition:
        'Quantity appears as determinate existence with one as limiting principle',
      type: 'quality',
      relation: 'contains',
      relatedTo: 'oneAsLimit',
    },
  ],
  invariants: [
    {
      id: 'quantity-limiting-intro-inv-1',
      constraint: 'quantum combines plurality with enclosed unity',
      predicate: 'combinesPluralityAndUnity(quantum)',
    },
    {
      id: 'quantity-limiting-intro-inv-2',
      constraint: 'limit is internal to quantity as one',
      predicate: 'internalLimit(quantity)',
    },
  ],
  forces: [
    {
      id: 'quantity-limiting-intro-force-1',
      description:
        'Internal limit becomes encompassing and explicit as enclosing boundary',
      type: 'mediation',
      trigger: 'limit.enclosingCharacter = true',
      effect: 'encompassingLimit.explicit = true',
      targetState: 'quantity-limiting-encompassing',
    },
  ],
  transitions: [
    {
      id: 'quantity-limiting-intro-trans-1',
      from: 'quantity-limiting-intro',
      to: 'quantity-limiting-encompassing',
      mechanism: 'mediation',
      description: 'From quantum in general to encompassing limit',
    },
  ],
  nextStates: ['quantity-limiting-encompassing'],
  previousStates: ['quantity-discrete-magnitude'],
  provenance: {
    topicMapId: 'quantity-limiting-intro',
    lineRange: { start: 3, end: 21 },
    section: 'C. THE LIMITING OF QUANTITY',
    order: 1,
  },
  description: LIMITING_QUANTITY_TOPIC_MAP.entries[0]?.description,
  keyPoints: LIMITING_QUANTITY_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'quantity-limiting-encompassing',
  title: 'Encompassing limit as negative point',
  concept: 'EncompassingLimit',
  phase: 'quantity',
  moments: [
    {
      name: 'enclosingOne',
      definition:
        'The one is self-referred limit that encloses quantity as negative point',
      type: 'negation',
    },
    {
      name: 'continuityTranscendsLimit',
      definition: 'Quantity as continuity both has and exceeds this limit',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'enclosingOne',
    },
  ],
  invariants: [
    {
      id: 'quantity-limiting-encompassing-inv-1',
      constraint: 'limit both bounds and belongs to quantity',
      predicate: 'internalBoundary(limit, quantity)',
    },
    {
      id: 'quantity-limiting-encompassing-inv-2',
      constraint: 'continuity/discreteness distinction tends to indifference',
      predicate: 'tendsToIndifference(continuity, discreteness)',
    },
  ],
  forces: [
    {
      id: 'quantity-limiting-encompassing-force-1',
      description:
        'Encompassing limit yields quantum as the explicit passage into determinable quanta',
      type: 'sublation',
      trigger: 'limitAndContinuity.interpenetrate = true',
      effect: 'quantumPassage.explicit = true',
      targetState: 'quantity-limiting-quantum',
    },
  ],
  transitions: [
    {
      id: 'quantity-limiting-encompassing-trans-1',
      from: 'quantity-limiting-encompassing',
      to: 'quantity-limiting-quantum',
      mechanism: 'sublation',
      description: 'From enclosing limit to explicit quantum transition',
    },
  ],
  nextStates: ['quantity-limiting-quantum'],
  previousStates: ['quantity-limiting-intro'],
  provenance: {
    topicMapId: 'quantity-limiting-encompassing',
    lineRange: { start: 23, end: 34 },
    section: 'C. THE LIMITING OF QUANTITY',
    order: 2,
  },
  description: LIMITING_QUANTITY_TOPIC_MAP.entries[1]?.description,
  keyPoints: LIMITING_QUANTITY_TOPIC_MAP.entries[1]?.keyPoints,
};

const state3: DialecticState = {
  id: 'quantity-limiting-quantum',
  title: 'Quantum as passage into extensive determination',
  concept: 'QuantumTransition',
  phase: 'quantity',
  moments: [
    {
      name: 'manyWithinLimit',
      definition: 'The one-limit encloses many ones as sublated moments',
      type: 'determination',
    },
    {
      name: 'passoverToQuanta',
      definition:
        'Continuous and discrete sides pass over into explicit quanta',
      type: 'passover',
      relation: 'transitions',
      relatedTo: 'manyWithinLimit',
    },
  ],
  invariants: [
    {
      id: 'quantity-limiting-quantum-inv-1',
      constraint:
        'quantum is the immediate threshold of further quantitative determination',
      predicate: 'threshold(quantum)',
    },
  ],
  forces: [
    {
      id: 'quantity-limiting-quantum-force-1',
      description:
        'Quantum transition opens the number chapter as the first explicit quantum determination',
      type: 'passover',
      trigger: 'quantum.requiresFurtherDetermination = true',
      effect: 'quantumChapter.initiated = true',
      targetState: 'number-quantum-as-number',
    },
  ],
  transitions: [
    {
      id: 'quantity-limiting-quantum-trans-1',
      from: 'quantity-limiting-quantum',
      to: 'number-quantum-as-number',
      mechanism: 'passover',
      description: 'From limiting quantity to number',
    },
  ],
  nextStates: ['number-quantum-as-number'],
  previousStates: ['quantity-limiting-encompassing'],
  provenance: {
    topicMapId: 'quantity-limiting-quantum',
    lineRange: { start: 36, end: 45 },
    section: 'C. THE LIMITING OF QUANTITY',
    order: 3,
  },
  description: LIMITING_QUANTITY_TOPIC_MAP.entries[2]?.description,
  keyPoints: LIMITING_QUANTITY_TOPIC_MAP.entries[2]?.keyPoints,
};

export const limitingQuantityIR: DialecticIR = {
  id: 'limiting-quantity-ir',
  title: 'Limiting Quantity IR: Quantum Threshold',
  section: 'BEING - QUANTITY - C. Limiting Quantity',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'limiting-quantity.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'quantity-limiting-intro': 'quantity',
      'quantity-limiting-encompassing': 'quantity',
      'quantity-limiting-quantum': 'quantity',
    },
  },
};

export const limitingQuantityStates = {
  'quantity-limiting-intro': state1,
  'quantity-limiting-encompassing': state2,
  'quantity-limiting-quantum': state3,
};
