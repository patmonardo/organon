import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { infinityTopicMap } from './sources/infinity-topic-map';

const state1: DialecticState = {
  id: 'infinity-1',
  title: 'Infinite as affirmative negation of finite',
  concept: 'InfiniteInGeneral',
  phase: 'quality',
  moments: [
    {
      name: 'negationOfNegation',
      definition:
        'The infinite is the negation of negation, affirmative being restored from restriction',
      type: 'sublation',
    },
    {
      name: 'spiritAtHome',
      definition:
        'The infinite is where spirit is at home as universality and freedom',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'negationOfNegation',
    },
  ],
  invariants: [
    {
      id: 'infinity-1-inv-1',
      constraint: 'infinite = negation of negation',
      predicate: 'equals(infinite, negationOfNegation)',
    },
    {
      id: 'infinity-1-inv-2',
      constraint: 'infinite is affirmative being beyond restriction',
      predicate: 'affirmativeBeyondRestriction(infinite)',
    },
  ],
  forces: [
    {
      id: 'infinity-1-force-1',
      description:
        'The concept of infinite compels exposition of finite self-transcendence',
      type: 'mediation',
      trigger: 'infinite.requiresConcreteGenesis = true',
      effect: 'finiteSelfTranscendence.explicit = true',
      targetState: 'infinity-4',
    },
  ],
  transitions: [
    {
      id: 'infinity-1-trans-1',
      from: 'infinity-1',
      to: 'infinity-4',
      mechanism: 'mediation',
      description: 'From infinite in general to finite self-transcendence',
    },
  ],
  nextStates: ['infinity-4'],
  previousStates: ['finitude-13'],
  provenance: {
    topicMapId: 'infinity-3',
    lineRange: { start: 48, end: 58 },
    section: 'a. The infinite in general',
    order: 1,
  },
  description: infinityTopicMap[2]?.description,
  keyPoints: infinityTopicMap[2]?.keyPoints,
};

const state2: DialecticState = {
  id: 'infinity-4',
  title: 'Finite transcends itself into infinite',
  concept: 'FiniteSelfTranscendence',
  phase: 'quality',
  moments: [
    {
      name: 'finiteAsSelfNegating',
      definition:
        'The finite refers itself to itself as restriction and negates that restriction',
      type: 'negation',
    },
    {
      name: 'infiniteAsVocation',
      definition:
        'Infinity is the finite’s own affirmative vocation, not an external beyond',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'finiteAsSelfNegating',
    },
  ],
  invariants: [
    {
      id: 'infinity-4-inv-1',
      constraint: 'finite becomes infinite through its own nature',
      predicate: 'becomesThroughOwnNature(finite, infinite)',
    },
    {
      id: 'infinity-4-inv-2',
      constraint: 'infinite is not externally imposed on finite',
      predicate: 'not(externallyImposed(infinite, finite))',
    },
  ],
  forces: [
    {
      id: 'infinity-4-force-1',
      description:
        'Self-transcending finite culminates in finite vanishing into the infinite',
      type: 'sublation',
      trigger: 'finite.selfNegation.completed = true',
      effect: 'finiteVanishedIntoInfinite = true',
      targetState: 'infinity-6',
    },
  ],
  transitions: [
    {
      id: 'infinity-4-trans-1',
      from: 'infinity-4',
      to: 'infinity-6',
      mechanism: 'sublation',
      description: 'From self-transcendence to vanishing of finite',
    },
  ],
  nextStates: ['infinity-6'],
  previousStates: ['infinity-1'],
  provenance: {
    topicMapId: 'infinity-5',
    lineRange: { start: 66, end: 84 },
    section: 'a. The infinite in general',
    order: 2,
  },
  description: infinityTopicMap[4]?.description,
  keyPoints: infinityTopicMap[4]?.keyPoints,
};

const state3: DialecticState = {
  id: 'infinity-6',
  title: 'Finite vanishes, only infinite is',
  concept: 'InfiniteResult',
  phase: 'quality',
  moments: [
    {
      name: 'finiteVanishing',
      definition: 'The finite has vanished into the infinite',
      type: 'sublation',
    },
    {
      name: 'onlyInfiniteIs',
      definition:
        'What is, is only the infinite as result of finite self-sublation',
      type: 'determination',
      relation: 'transitions',
      relatedTo: 'finiteVanishing',
    },
  ],
  invariants: [
    {
      id: 'infinity-6-inv-1',
      constraint: 'finite has no independent standing over against infinite',
      predicate: 'not(independentStanding(finite, infinite))',
    },
  ],
  forces: [
    {
      id: 'infinity-6-force-1',
      description:
        'Resulting infinite unfolds into the alternating determination of bad infinity',
      type: 'passover',
      trigger: 'infinite.resultRequiresFurtherDetermination = true',
      effect: 'badInfiniteDialectic.initiated = true',
      targetState: 'alternating-infinity-1',
    },
  ],
  transitions: [
    {
      id: 'infinity-6-trans-1',
      from: 'infinity-6',
      to: 'alternating-infinity-1',
      mechanism: 'passover',
      description: 'From infinite-in-general to alternating infinity',
    },
  ],
  nextStates: ['alternating-infinity-1'],
  previousStates: ['infinity-4'],
  provenance: {
    topicMapId: 'infinity-6',
    lineRange: { start: 85, end: 92 },
    section: 'a. The infinite in general',
    order: 3,
  },
  description: infinityTopicMap[5]?.description,
  keyPoints: infinityTopicMap[5]?.keyPoints,
};

export const infinityIR: DialecticIR = {
  id: 'infinity-ir',
  title: 'Infinity IR: Infinite in General',
  section: 'BEING - QUALITY - C. Infinity - a. The Infinite in General',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'infinity.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'infinity-1': 'quality',
      'infinity-4': 'quality',
      'infinity-6': 'quality',
    },
  },
};

export const infinityStates = {
  'infinity-1': state1,
  'infinity-4': state2,
  'infinity-6': state3,
};
