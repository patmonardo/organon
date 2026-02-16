import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { PURE_QUANTITY_TOPIC_MAP } from './sources/pure-quantity-topic-map';

const state1: DialecticState = {
  id: 'quantity-1',
  title: 'Pure quantity as sublated being-for-itself',
  concept: 'PureQuantityImmediacy',
  phase: 'quantity',
  moments: [
    {
      name: 'sublatedBeingForItself',
      definition:
        'Repelling one loses rigid self-exclusion and passes into attraction',
      type: 'sublation',
    },
    {
      name: 'continuityEmergence',
      definition:
        'Attraction appears as continuity, the self-same reference of quantity',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'sublatedBeingForItself',
    },
  ],
  invariants: [
    {
      id: 'quantity-1-inv-1',
      constraint: 'quantity begins as unity of repulsion and attraction',
      predicate: 'unity(repulsion, attraction)',
    },
    {
      id: 'quantity-1-inv-2',
      constraint: 'being-for-itself is preserved only in sublated form',
      predicate: 'preservedAsSublated(beingForItself)',
    },
  ],
  forces: [
    {
      id: 'quantity-1-force-1',
      description:
        'Continuity must articulate itself as internally discrete without losing self-equality',
      type: 'mediation',
      trigger: 'continuity.containsDifference = true',
      effect: 'discreteMoment.explicit = true',
      targetState: 'quantity-4',
    },
  ],
  transitions: [
    {
      id: 'quantity-1-trans-1',
      from: 'quantity-1',
      to: 'quantity-4',
      mechanism: 'mediation',
      description:
        'From pure quantity immediacy to continuity/discreteness articulation',
    },
  ],
  nextStates: ['quantity-4'],
  previousStates: ['attraction-c'],
  provenance: {
    topicMapId: 'quantity-pure-intro',
    lineRange: { start: 3, end: 14 },
    section: 'A. PURE QUANTITY',
    order: 1,
  },
  description: PURE_QUANTITY_TOPIC_MAP.entries[0]?.description,
  keyPoints: PURE_QUANTITY_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'quantity-4',
  title: 'Continuity and discreteness as one quantity',
  concept: 'ContinuousDiscreteUnity',
  phase: 'quantity',
  moments: [
    {
      name: 'continuity',
      definition:
        'Unbroken self-sameness that contains plurality as internally equal',
      type: 'determination',
    },
    {
      name: 'discreteness',
      definition: 'Repulsion persists as many ones, yet only within continuity',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'continuity',
    },
  ],
  invariants: [
    {
      id: 'quantity-4-inv-1',
      constraint:
        'continuity and discreteness are inseparable moments of quantity',
      predicate: 'inseparable(continuity, discreteness)',
    },
    {
      id: 'quantity-4-inv-2',
      constraint: 'plurality does not interrupt quantitative self-equality',
      predicate: 'notInterruptingEquality(plurality)',
    },
  ],
  forces: [
    {
      id: 'quantity-4-force-1',
      description:
        'Unified moments require explicit positing of quantity as whole magnitude',
      type: 'sublation',
      trigger: 'moments.requireConcreteWhole = true',
      effect: 'magnitudeWhole.posited = true',
      targetState: 'quantity-pure-unity',
    },
  ],
  transitions: [
    {
      id: 'quantity-4-trans-1',
      from: 'quantity-4',
      to: 'quantity-pure-unity',
      mechanism: 'sublation',
      description: 'From moment-duality to pure quantity as concrete whole',
    },
  ],
  nextStates: ['quantity-pure-unity'],
  previousStates: ['quantity-1'],
  provenance: {
    topicMapId: 'quantity-pure-discreteness',
    lineRange: { start: 32, end: 40 },
    section: 'A. PURE QUANTITY',
    order: 2,
  },
  description: PURE_QUANTITY_TOPIC_MAP.entries[2]?.description,
  keyPoints: PURE_QUANTITY_TOPIC_MAP.entries[2]?.keyPoints,
};

const state3: DialecticState = {
  id: 'quantity-pure-unity',
  title: 'Pure quantity as self-equal whole',
  concept: 'PureQuantityWhole',
  phase: 'quantity',
  moments: [
    {
      name: 'selfSublatingReference',
      definition:
        'Quantity is self-reference that sublates itself into plurality while remaining equal to itself',
      type: 'sublation',
    },
    {
      name: 'creativeFlow',
      definition:
        'Repulsion and attraction appear as one creative flow of quantitative determination',
      type: 'process',
      relation: 'transitions',
      relatedTo: 'selfSublatingReference',
    },
  ],
  invariants: [
    {
      id: 'quantity-pure-unity-inv-1',
      constraint:
        'pure quantity is concrete unity of its own differentiated moments',
      predicate: 'concreteUnity(pureQuantity)',
    },
  ],
  forces: [
    {
      id: 'quantity-pure-unity-force-1',
      description:
        'Pure quantity as whole must differentiate into continuous and discrete magnitude',
      type: 'passover',
      trigger: 'whole.requiresDifferentiatedForm = true',
      effect: 'magnitudeChapter.initiated = true',
      targetState: 'magnitude-1',
    },
  ],
  transitions: [
    {
      id: 'quantity-pure-unity-trans-1',
      from: 'quantity-pure-unity',
      to: 'magnitude-1',
      mechanism: 'passover',
      description: 'From pure quantity to magnitude',
    },
  ],
  nextStates: ['magnitude-1'],
  previousStates: ['quantity-4'],
  provenance: {
    topicMapId: 'quantity-pure-unity',
    lineRange: { start: 42, end: 65 },
    section: 'A. PURE QUANTITY',
    order: 3,
  },
  description: PURE_QUANTITY_TOPIC_MAP.entries[3]?.description,
  keyPoints: PURE_QUANTITY_TOPIC_MAP.entries[3]?.keyPoints,
};

export const pureQuantityIR: DialecticIR = {
  id: 'pure-quantity-ir',
  title: 'Pure Quantity IR: Continuity and Discreteness',
  section: 'BEING - QUANTITY - A. Pure Quantity',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'pure-quantity.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'quantity-1': 'quantity',
      'quantity-4': 'quantity',
      'quantity-pure-unity': 'quantity',
    },
  },
};

export const pureQuantityStates = {
  'quantity-1': state1,
  'quantity-4': state2,
  'quantity-pure-unity': state3,
};
