import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { constitutionTopicMap } from './sources/constitution-topic-map';

const state1: DialecticState = {
  id: 'constitution-1',
  title: 'In-itself mediated through being-for-other',
  concept: 'MediatedInItself',
  phase: 'quality',
  moments: [
    {
      name: 'mediatedInItself',
      definition:
        'In-itself is no longer abstract; it returns to itself through being-for-other',
      type: 'mediation',
    },
    {
      name: 'determinatenessInItself',
      definition:
        'Determinateness is posited as immanently reflected in the something',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'mediatedInItself',
    },
  ],
  invariants: [
    {
      id: 'constitution-1-inv-1',
      constraint: 'in-itself includes and sublates being-for-other',
      predicate: 'sublatesWithin(inItself, beingForOther)',
    },
    {
      id: 'constitution-1-inv-2',
      constraint: 'determinateness is now immanent, not external only',
      predicate: 'immanentDeterminateness(something)',
    },
  ],
  forces: [
    {
      id: 'constitution-1-force-1',
      description:
        'Immanent determinateness differentiates into determination, constitution, and their transition',
      type: 'mediation',
      trigger: 'determinateness.immanent = true',
      effect: 'doubleMoment.explicit = true',
      targetState: 'constitution-10',
    },
  ],
  transitions: [
    {
      id: 'constitution-1-trans-1',
      from: 'constitution-1',
      to: 'constitution-10',
      mechanism: 'mediation',
      description:
        'From mediated in-itself to dynamic determination/constitution relation',
    },
  ],
  nextStates: ['constitution-10'],
  previousStates: ['something-and-other-17'],
  provenance: {
    topicMapId: 'constitution-1',
    lineRange: { start: 4, end: 25 },
    section: 'b. Determination, constitution, and limit',
    order: 1,
  },
  description: constitutionTopicMap[0]?.description,
  keyPoints: constitutionTopicMap[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'constitution-10',
  title: 'Limit as joining and separating determinateness',
  concept: 'LimitMediation',
  phase: 'quality',
  moments: [
    {
      name: 'negationOfOtherAsQuality',
      definition:
        'Something preserves itself through negating its other; this negation is its qualitative determinateness',
      type: 'negation',
    },
    {
      name: 'limitAsCommonDeterminateness',
      definition:
        'Limit both joins and separates two somethings as one immanent determination',
      type: 'mediation',
      relation: 'transitions',
      relatedTo: 'negationOfOtherAsQuality',
    },
  ],
  invariants: [
    {
      id: 'constitution-10-inv-1',
      constraint: 'limit is non-being of each and mediation of both',
      predicate: 'mediatingNonBeing(limit, something, other)',
    },
    {
      id: 'constitution-10-inv-2',
      constraint: 'determination and constitution pass over into each other',
      predicate: 'mutualPassover(determination, constitution)',
    },
  ],
  forces: [
    {
      id: 'constitution-10-force-1',
      description:
        'Immanent limit turns into contradiction that constitutes finitude',
      type: 'contradiction',
      trigger: 'limit.immanentAndNegative = true',
      effect: 'finitude.posited = true',
      targetState: 'constitution-16',
    },
  ],
  transitions: [
    {
      id: 'constitution-10-trans-1',
      from: 'constitution-10',
      to: 'constitution-16',
      mechanism: 'contradiction',
      description: 'From limit mediation to finite contradiction',
    },
  ],
  nextStates: ['constitution-16'],
  previousStates: ['constitution-1'],
  provenance: {
    topicMapId: 'constitution-10',
    lineRange: { start: 181, end: 227 },
    section: 'Transition',
    order: 2,
  },
  description: constitutionTopicMap[9]?.description,
  keyPoints: constitutionTopicMap[9]?.keyPoints,
};

const state3: DialecticState = {
  id: 'constitution-16',
  title: 'Finite as something driven beyond itself',
  concept: 'FiniteByImmanentLimit',
  phase: 'quality',
  moments: [
    {
      name: 'immanentContradiction',
      definition:
        'Something with immanent limit is contradiction and therefore self-transcending',
      type: 'contradiction',
    },
    {
      name: 'directedBeyondItself',
      definition:
        'The finite is constituted by being driven beyond its own standing',
      type: 'process',
      relation: 'transitions',
      relatedTo: 'immanentContradiction',
    },
  ],
  invariants: [
    {
      id: 'constitution-16-inv-1',
      constraint: 'finitude follows from limit as internal contradiction',
      predicate: 'followsFromInternalLimit(finite)',
    },
  ],
  forces: [
    {
      id: 'constitution-16-force-1',
      description:
        'Finite contradiction requires explicit treatment of finitude, restriction, and ought',
      type: 'passover',
      trigger: 'finite.contradiction.explicit = true',
      effect: 'finitudeChapter.initiated = true',
      targetState: 'finitude-1',
    },
  ],
  transitions: [
    {
      id: 'constitution-16-trans-1',
      from: 'constitution-16',
      to: 'finitude-1',
      mechanism: 'passover',
      description: 'From limited something to finitude proper',
    },
  ],
  nextStates: ['finitude-1'],
  previousStates: ['constitution-10'],
  provenance: {
    topicMapId: 'constitution-16',
    lineRange: { start: 407, end: 410 },
    section: 'Conclusion',
    order: 3,
  },
  description: constitutionTopicMap[15]?.description,
  keyPoints: constitutionTopicMap[15]?.keyPoints,
};

export const constitutionIR: DialecticIR = {
  id: 'constitution-ir',
  title: 'Constitution IR: Determination, Constitution, and Limit',
  section:
    'BEING - QUALITY - B. Finitude - b. Determination, Constitution, and Limit',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'constitution.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'constitution-1': 'quality',
      'constitution-10': 'quality',
      'constitution-16': 'quality',
    },
  },
};

export const constitutionStates = {
  'constitution-1': state1,
  'constitution-10': state2,
  'constitution-16': state3,
};
