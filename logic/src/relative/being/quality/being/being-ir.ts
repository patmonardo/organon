import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { beingTopicMap } from './sources/being-topic-map';

const state1: DialecticState = {
  id: 'being-1',
  title: 'Pure being as indeterminate immediacy',
  concept: 'PureBeing',
  phase: 'quality',
  moments: [
    {
      name: 'pureIndeterminacy',
      definition:
        'Being has no internal or external difference and is equal only to itself',
      type: 'determination',
    },
    {
      name: 'immediateSelfEquality',
      definition: 'Its immediacy excludes all determinate content',
      type: 'quality',
    },
  ],
  invariants: [
    {
      id: 'being-1-inv-1',
      constraint: 'pure being excludes determinate predicates',
      predicate: 'not(hasDetermination(pureBeing))',
    },
    {
      id: 'being-1-inv-2',
      constraint: 'immediacy is identical with indeterminacy',
      predicate: 'equals(immediacy, indeterminacy)',
    },
  ],
  forces: [
    {
      id: 'being-1-force-1',
      description:
        'Because pure being is empty of determination, it reflects into emptiness',
      type: 'reflection',
      trigger: 'indeterminacy.absolute = true',
      effect: 'emptiness.articulated = true',
      targetState: 'being-4',
    },
  ],
  transitions: [
    {
      id: 'being-1-trans-1',
      from: 'being-1',
      to: 'being-4',
      mechanism: 'reflection',
      description: 'Pure indeterminacy is grasped as emptiness',
    },
  ],
  nextStates: ['being-4'],
  previousStates: ['method-advance-6'],
  provenance: {
    topicMapId: 'being-1',
    lineRange: { start: 32, end: 39 },
    section: 'A. BEING',
    order: 1,
  },
  description: beingTopicMap[0]?.description,
  keyPoints: beingTopicMap[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'being-4',
  title: 'Being as emptiness and pure nothing',
  concept: 'EmptyBeing',
  phase: 'quality',
  moments: [
    {
      name: 'emptyIntuitingThinking',
      definition:
        'To intuit or think pure being yields only empty intuiting and empty thinking',
      type: 'mediation',
    },
    {
      name: 'identityWithNothing',
      definition:
        'The indeterminate immediate is neither more nor less than nothing',
      type: 'negation',
      relation: 'transitions',
      relatedTo: 'emptyIntuitingThinking',
    },
  ],
  invariants: [
    {
      id: 'being-4-inv-1',
      constraint: 'pure being and pure nothing coincide in indeterminacy',
      predicate: 'equals(pureBeing, pureNothing)',
    },
    {
      id: 'being-4-inv-2',
      constraint: 'emptiness persists in both intuiting and thinking',
      predicate: 'emptyMode(intuiting, thinking)',
    },
  ],
  forces: [
    {
      id: 'being-4-force-1',
      description:
        'The identity with nothing destabilizes static being and demands explicit transition',
      type: 'negation',
      trigger: 'being.equalsNothing = true',
      effect: 'transitionMoment.activated = true',
      targetState: 'being-6',
    },
  ],
  transitions: [
    {
      id: 'being-4-trans-1',
      from: 'being-4',
      to: 'being-6',
      mechanism: 'negation',
      description: 'Identity with nothing forces becoming as movement',
    },
  ],
  nextStates: ['being-6'],
  previousStates: ['being-1'],
  provenance: {
    topicMapId: 'being-2',
    lineRange: { start: 40, end: 47 },
    section: 'A. BEING',
    order: 2,
  },
  description: beingTopicMap[1]?.description,
  keyPoints: beingTopicMap[1]?.keyPoints,
};

const state3: DialecticState = {
  id: 'being-6',
  title: 'Being sublates into the explicit standpoint of nothing',
  concept: 'TransitionToNothing',
  phase: 'quality',
  moments: [
    {
      name: 'selfCancellingImmediacy',
      definition:
        'Immediacy cancels itself because its content is only the absence of content',
      type: 'sublation',
    },
    {
      name: 'explicitNothing',
      definition: 'Nothing is now posited as the direct truth of pure being',
      type: 'determination',
      relation: 'transitions',
      relatedTo: 'selfCancellingImmediacy',
    },
  ],
  invariants: [
    {
      id: 'being-6-inv-1',
      constraint:
        'transition preserves identity of being and nothing as one movement',
      predicate: 'preservesIdentityInTransition(being, nothing)',
    },
  ],
  forces: [
    {
      id: 'being-6-force-1',
      description:
        'Being hands off to the chapter where nothing is treated in itself',
      type: 'passover',
      trigger: 'nothing.explicitlyPosited = true',
      effect: 'nothingChapter.initiated = true',
      targetState: 'nothing-1',
    },
  ],
  transitions: [
    {
      id: 'being-6-trans-1',
      from: 'being-6',
      to: 'nothing-1',
      mechanism: 'passover',
      description: 'From pure being to pure nothing',
    },
  ],
  nextStates: ['nothing-1'],
  previousStates: ['being-4'],
  provenance: {
    topicMapId: 'being-2',
    lineRange: { start: 40, end: 47 },
    section: 'A. BEING',
    order: 3,
  },
  description: beingTopicMap[1]?.description,
  keyPoints: beingTopicMap[1]?.keyPoints,
};

export const beingIR: DialecticIR = {
  id: 'being-ir',
  title: 'Being IR: Pure Being',
  section: 'BEING - QUALITY - A. Being',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'being.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'being-1': 'quality',
      'being-4': 'quality',
      'being-6': 'quality',
    },
  },
};

export const beingStates = {
  'being-1': state1,
  'being-4': state2,
  'being-6': state3,
};
