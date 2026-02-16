import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { existenceTopicMap } from './sources/existence-topic-map';

const state1: DialecticState = {
  id: 'existence-1',
  title: 'Existence as determinate immediate from becoming',
  concept: 'ExistenceImmediate',
  phase: 'quality',
  moments: [
    {
      name: 'immediateOneness',
      definition:
        'Existence is the simple oneness of being and nothing that appears as immediate',
      type: 'determination',
    },
    {
      name: 'determinatenessAsSuch',
      definition:
        'Non-being is taken up into being and posited as determinateness in the form of immediacy',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'immediateOneness',
    },
  ],
  invariants: [
    {
      id: 'existence-1-inv-1',
      constraint: 'existence proceeds from becoming as sublated mediation',
      predicate: 'proceedsFrom(existence, becoming)',
    },
    {
      id: 'existence-1-inv-2',
      constraint:
        'determinateness is intrinsic to existence as immediate being',
      predicate: 'intrinsicDeterminateness(existence)',
    },
  ],
  forces: [
    {
      id: 'existence-1-force-1',
      description:
        'The immediate determinateness differentiates itself into quality, reality, and negation',
      type: 'mediation',
      trigger: 'determinateness.requiresArticulation = true',
      effect: 'qualityDialectic.emerges = true',
      targetState: 'existence-9',
    },
  ],
  transitions: [
    {
      id: 'existence-1-trans-1',
      from: 'existence-1',
      to: 'existence-9',
      mechanism: 'mediation',
      description:
        'From existence-in-general to quality as reality and negation',
    },
  ],
  nextStates: ['existence-9'],
  previousStates: ['becoming-7'],
  provenance: {
    topicMapId: 'existence-2',
    lineRange: { start: 17, end: 28 },
    section: 'a. Existence in general',
    order: 1,
  },
  description: existenceTopicMap[1]?.description,
  keyPoints: existenceTopicMap[1]?.keyPoints,
};

const state2: DialecticState = {
  id: 'existence-9',
  title: 'Quality as reality and negation',
  concept: 'QualityDuality',
  phase: 'quality',
  moments: [
    {
      name: 'realityAndNegation',
      definition:
        'Quality is articulated as reality and negation, each a mode of existence',
      type: 'determination',
    },
    {
      name: 'limitAndRestriction',
      definition:
        'Negation appears as reflected determinateness, limit, and restriction',
      type: 'negation',
      relation: 'transitions',
      relatedTo: 'realityAndNegation',
    },
  ],
  invariants: [
    {
      id: 'existence-9-inv-1',
      constraint: 'reality and negation remain within one existence',
      predicate: 'withinOneExistence(reality, negation)',
    },
    {
      id: 'existence-9-inv-2',
      constraint: 'quality is inseparable from existence',
      predicate: 'inseparable(quality, existence)',
    },
  ],
  forces: [
    {
      id: 'existence-9-force-1',
      description:
        'The unity of reality and negation requires reflected self-reference as something',
      type: 'sublation',
      trigger: 'quality.reflectedIntoSelf = true',
      effect: 'somethingForm.posited = true',
      targetState: 'existence-16',
    },
  ],
  transitions: [
    {
      id: 'existence-9-trans-1',
      from: 'existence-9',
      to: 'existence-16',
      mechanism: 'sublation',
      description: 'From quality duality to something as being-in-itself',
    },
  ],
  nextStates: ['existence-16'],
  previousStates: ['existence-1'],
  provenance: {
    topicMapId: 'existence-9',
    lineRange: { start: 131, end: 143 },
    section: 'b. Quality',
    order: 2,
  },
  description: existenceTopicMap[8]?.description,
  keyPoints: existenceTopicMap[8]?.keyPoints,
};

const state3: DialecticState = {
  id: 'existence-16',
  title: 'Something as concrete alteration',
  concept: 'SomethingAlteration',
  phase: 'quality',
  moments: [
    {
      name: 'negationOfNegationAsSomething',
      definition:
        'Something is the first negation of negation, restored self-reference within existence',
      type: 'sublation',
    },
    {
      name: 'alterationInConcept',
      definition:
        'Something already contains concrete becoming as alteration toward its other',
      type: 'process',
      relation: 'transitions',
      relatedTo: 'negationOfNegationAsSomething',
    },
  ],
  invariants: [
    {
      id: 'existence-16-inv-1',
      constraint: 'something contains both self-reference and otherness',
      predicate: 'containsBoth(something, selfReference, otherness)',
    },
  ],
  forces: [
    {
      id: 'existence-16-force-1',
      description:
        'Alteration requires explicit differentiation into something and other',
      type: 'passover',
      trigger: 'otherness.qualitativelyPosited = true',
      effect: 'somethingOtherChapter.initiated = true',
      targetState: 'something-and-other-1',
    },
  ],
  transitions: [
    {
      id: 'existence-16-trans-1',
      from: 'existence-16',
      to: 'something-and-other-1',
      mechanism: 'passover',
      description: 'From something to the explicit something-other relation',
    },
  ],
  nextStates: ['something-and-other-1'],
  previousStates: ['existence-9'],
  provenance: {
    topicMapId: 'existence-16',
    lineRange: { start: 258, end: 267 },
    section: 'c. Something',
    order: 3,
  },
  description: existenceTopicMap[15]?.description,
  keyPoints: existenceTopicMap[15]?.keyPoints,
};

export const existenceIR: DialecticIR = {
  id: 'existence-ir',
  title: 'Existence IR: Determinate Existence as Such',
  section: 'BEING - QUALITY - A. Existence as Such',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'existence.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'existence-1': 'quality',
      'existence-9': 'quality',
      'existence-16': 'quality',
    },
  },
};

export const existenceStates = {
  'existence-1': state1,
  'existence-9': state2,
  'existence-16': state3,
};
