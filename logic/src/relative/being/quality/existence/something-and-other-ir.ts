import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { somethingAndOtherTopicMap } from './sources/something-and-other-topic-map';

const state1: DialecticState = {
  id: 'something-and-other-1',
  title: 'Something and other as immediate pair',
  concept: 'ImmediateSomethingOther',
  phase: 'quality',
  moments: [
    {
      name: 'indifferentOthers',
      definition:
        'Something and other first appear as indifferent immediacies with external negation',
      type: 'determination',
    },
    {
      name: 'outlineOfFinitude',
      definition:
        'Their relation already points toward determination, constitution, limit, and finitude',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'indifferentOthers',
    },
  ],
  invariants: [
    {
      id: 'something-and-other-1-inv-1',
      constraint: 'each side is both something and other',
      predicate: 'reciprocalOtherness(something, other)',
    },
    {
      id: 'something-and-other-1-inv-2',
      constraint: 'negation is not merely external in developed relation',
      predicate: 'notPurelyExternal(negation)',
    },
  ],
  forces: [
    {
      id: 'something-and-other-1-force-1',
      description:
        'Indifference destabilizes into internal relation of being-for-other and being-in-itself',
      type: 'mediation',
      trigger: 'otherness.internalized = true',
      effect: 'relationalMoments.explicit = true',
      targetState: 'something-and-other-9',
    },
  ],
  transitions: [
    {
      id: 'something-and-other-1-trans-1',
      from: 'something-and-other-1',
      to: 'something-and-other-9',
      mechanism: 'mediation',
      description: 'From immediate duality to being-for-other/being-in-itself',
    },
  ],
  nextStates: ['something-and-other-9'],
  previousStates: ['existence-16'],
  provenance: {
    topicMapId: 'something-and-other-1',
    lineRange: { start: 4, end: 19 },
    section: 'B. FINITUDE',
    order: 1,
  },
  description: somethingAndOtherTopicMap[0]?.description,
  keyPoints: somethingAndOtherTopicMap[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'something-and-other-9',
  title: 'Being-for-other and being-in-itself as one structure',
  concept: 'RelationalIdentity',
  phase: 'quality',
  moments: [
    {
      name: 'beingForOther',
      definition:
        'Something refers to otherness while preserving itself in non-being',
      type: 'determination',
    },
    {
      name: 'beingInItself',
      definition:
        'Self-reference and other-reference are two moments of one unity',
      type: 'reflection',
      relation: 'contains',
      relatedTo: 'beingForOther',
    },
  ],
  invariants: [
    {
      id: 'something-and-other-9-inv-1',
      constraint:
        'being-for-other and being-in-itself are mutually implicative',
      predicate: 'mutuallyImplicative(beingForOther, beingInItself)',
    },
    {
      id: 'something-and-other-9-inv-2',
      constraint: 'identity is concrete rather than abstract isolation',
      predicate: 'concreteIdentity(something)',
    },
  ],
  forces: [
    {
      id: 'something-and-other-9-force-1',
      description:
        'Identity of moments drives determinateness to become explicit determination',
      type: 'sublation',
      trigger: 'moments.identifiedInOne = true',
      effect: 'determination.explicit = true',
      targetState: 'something-and-other-17',
    },
  ],
  transitions: [
    {
      id: 'something-and-other-9-trans-1',
      from: 'something-and-other-9',
      to: 'something-and-other-17',
      mechanism: 'sublation',
      description:
        'From relational identity to reflected determinateness as quality',
    },
  ],
  nextStates: ['something-and-other-17'],
  previousStates: ['something-and-other-1'],
  provenance: {
    topicMapId: 'something-and-other-9',
    lineRange: { start: 126, end: 145 },
    section: 'a. Something and other',
    order: 2,
  },
  description: somethingAndOtherTopicMap[8]?.description,
  keyPoints: somethingAndOtherTopicMap[8]?.keyPoints,
};

const state3: DialecticState = {
  id: 'something-and-other-17',
  title: 'Determinateness reflected into itself',
  concept: 'DeterminationRecovered',
  phase: 'quality',
  moments: [
    {
      name: 'reflectedDeterminateness',
      definition:
        'Being-for-other is in the something itself, so determinateness is reflected and simple again',
      type: 'reflection',
    },
    {
      name: 'qualityAsDetermination',
      definition:
        'This reflected simplicity becomes determination as quality for the next development',
      type: 'determination',
      relation: 'transitions',
      relatedTo: 'reflectedDeterminateness',
    },
  ],
  invariants: [
    {
      id: 'something-and-other-17-inv-1',
      constraint: 'unity of in-itself and for-other is now internally posited',
      predicate: 'internallyPositedUnity(something)',
    },
  ],
  forces: [
    {
      id: 'something-and-other-17-force-1',
      description:
        'Reflected determination requires explicit treatment of determination, constitution, and limit',
      type: 'passover',
      trigger: 'determination.needsFurtherArticulation = true',
      effect: 'constitutionChapter.initiated = true',
      targetState: 'constitution-1',
    },
  ],
  transitions: [
    {
      id: 'something-and-other-17-trans-1',
      from: 'something-and-other-17',
      to: 'constitution-1',
      mechanism: 'passover',
      description: 'To determination, constitution, and limit',
    },
  ],
  nextStates: ['constitution-1'],
  previousStates: ['something-and-other-9'],
  provenance: {
    topicMapId: 'something-and-other-17',
    lineRange: { start: 324, end: 329 },
    section: 'a. Something and other',
    order: 3,
  },
  description: somethingAndOtherTopicMap[16]?.description,
  keyPoints: somethingAndOtherTopicMap[16]?.keyPoints,
};

export const somethingAndOtherIR: DialecticIR = {
  id: 'something-and-other-ir',
  title: 'Something and Other IR: Relational Finitude',
  section: 'BEING - QUALITY - B. Finitude - a. Something and Other',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'something-and-other.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'something-and-other-1': 'quality',
      'something-and-other-9': 'quality',
      'something-and-other-17': 'quality',
    },
  },
};

export const somethingAndOtherStates = {
  'something-and-other-1': state1,
  'something-and-other-9': state2,
  'something-and-other-17': state3,
};
