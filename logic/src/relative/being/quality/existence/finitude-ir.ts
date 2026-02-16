import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { finitudeTopicMap } from './sources/finitude-topic-map';

const state1: DialecticState = {
  id: 'finitude-1',
  title: 'Finite as immanent perishing',
  concept: 'FiniteNature',
  phase: 'quality',
  moments: [
    {
      name: 'nonBeingAsNature',
      definition:
        'Finite being has non-being as its own nature and carries its end within itself',
      type: 'negation',
    },
    {
      name: 'perishingAsTruth',
      definition:
        'The finite does not merely alter; it perishes by its own in-itself contradiction',
      type: 'process',
      relation: 'contains',
      relatedTo: 'nonBeingAsNature',
    },
  ],
  invariants: [
    {
      id: 'finitude-1-inv-1',
      constraint: 'finite being includes the germ of its transgression',
      predicate: 'containsGermOfPerishing(finite)',
    },
    {
      id: 'finitude-1-inv-2',
      constraint: 'hour of birth coincides with hour of death',
      predicate: 'equals(hourOfBirth, hourOfDeath)',
    },
  ],
  forces: [
    {
      id: 'finitude-1-force-1',
      description:
        'Immanent negativity develops into explicit structure of restriction and ought',
      type: 'mediation',
      trigger: 'finite.negativity.explicit = true',
      effect: 'restrictionOughtStructure.emerges = true',
      targetState: 'finitude-8',
    },
  ],
  transitions: [
    {
      id: 'finitude-1-trans-1',
      from: 'finitude-1',
      to: 'finitude-8',
      mechanism: 'mediation',
      description: 'From immediate finitude to restriction and ought',
    },
  ],
  nextStates: ['finitude-8'],
  previousStates: ['constitution-16'],
  provenance: {
    topicMapId: 'finitude-1',
    lineRange: { start: 4, end: 37 },
    section: 'c. Finitude',
    order: 1,
  },
  description: finitudeTopicMap[0]?.description,
  keyPoints: finitudeTopicMap[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'finitude-8',
  title: 'Restriction and ought as one finite structure',
  concept: 'RestrictionOught',
  phase: 'quality',
  moments: [
    {
      name: 'doubleDeterminationOfOught',
      definition:
        'Ought is in-itselfness against negation yet itself bound to restriction',
      type: 'determination',
    },
    {
      name: 'indivisibleRestriction',
      definition:
        'Restriction and ought are inseparable moments of one finite being',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'doubleDeterminationOfOught',
    },
  ],
  invariants: [
    {
      id: 'finitude-8-inv-1',
      constraint: 'finite transcends restriction only through restriction',
      predicate: 'transcendsThroughRestriction(finite)',
    },
    {
      id: 'finitude-8-inv-2',
      constraint: 'ought and restriction mutually determine each other',
      predicate: 'mutualDetermination(ought, restriction)',
    },
  ],
  forces: [
    {
      id: 'finitude-8-force-1',
      description:
        'Mutual negation of ought and restriction turns the finite into explicit contradiction',
      type: 'contradiction',
      trigger: 'ought.and.restriction.opposed = true',
      effect: 'finite.selfSublation.activated = true',
      targetState: 'finitude-13',
    },
  ],
  transitions: [
    {
      id: 'finitude-8-trans-1',
      from: 'finitude-8',
      to: 'finitude-13',
      mechanism: 'contradiction',
      description: 'From finite opposition to transition into the infinite',
    },
  ],
  nextStates: ['finitude-13'],
  previousStates: ['finitude-1'],
  provenance: {
    topicMapId: 'finitude-8',
    lineRange: { start: 203, end: 227 },
    section: '(b) Restriction and the ought',
    order: 2,
  },
  description: finitudeTopicMap[7]?.description,
  keyPoints: finitudeTopicMap[7]?.keyPoints,
};

const state3: DialecticState = {
  id: 'finitude-13',
  title: 'Finite rejoining itself as infinite',
  concept: 'TransitionToInfinite',
  phase: 'quality',
  moments: [
    {
      name: 'perishingOfPerishing',
      definition:
        'In self-negation the finite rejoins itself, so perishing is not the last word',
      type: 'sublation',
    },
    {
      name: 'identityAsInfinite',
      definition:
        'Identity with itself as negation of negation is affirmative being: the infinite',
      type: 'determination',
      relation: 'transitions',
      relatedTo: 'perishingOfPerishing',
    },
  ],
  invariants: [
    {
      id: 'finitude-13-inv-1',
      constraint: 'finite passes into infinite by its own concept',
      predicate: 'selfTransition(finite, infinite)',
    },
  ],
  forces: [
    {
      id: 'finitude-13-force-1',
      description:
        'The infinite result must be unfolded as alternating infinity and true infinity',
      type: 'passover',
      trigger: 'infinite.result.posited = true',
      effect: 'infinityDialectic.initiated = true',
      targetState: 'infinity-1',
    },
  ],
  transitions: [
    {
      id: 'finitude-13-trans-1',
      from: 'finitude-13',
      to: 'infinity-1',
      mechanism: 'passover',
      description: 'From finitude to infinity dialectic',
    },
  ],
  nextStates: ['infinity-1'],
  previousStates: ['finitude-8'],
  provenance: {
    topicMapId: 'finitude-13',
    lineRange: { start: 288, end: 310 },
    section: '(c) Transition of the finite into the infinite',
    order: 3,
  },
  description: finitudeTopicMap[12]?.description,
  keyPoints: finitudeTopicMap[12]?.keyPoints,
};

export const finitudeIR: DialecticIR = {
  id: 'finitude-ir',
  title: 'Finitude IR: Restriction, Ought, and Transition to Infinite',
  section: 'BEING - QUALITY - B. Finitude - c. Finitude',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'finitude.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'finitude-1': 'quality',
      'finitude-8': 'quality',
      'finitude-13': 'quality',
    },
  },
};

export const finitudeStates = {
  'finitude-1': state1,
  'finitude-8': state2,
  'finitude-13': state3,
};
