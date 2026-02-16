import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { ABSOLUTE_NECESSITY_TOPIC_MAP } from './sources/absolute-necessity-topic-map';

const state1: DialecticState = {
  id: 'act-c-1',
  title: 'Determinate necessity contains contingency',
  concept: 'DeterminateNecessity',
  phase: 'appearance',
  moments: [
    {
      name: 'determinateNecessity',
      definition: 'Real necessity has content and determinateness',
      type: 'determination',
    },
    {
      name: 'innerContingency',
      definition: 'Negation is internal to necessity as its own moment',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'determinateNecessity',
    },
  ],
  invariants: [
    {
      id: 'act-c-1-inv-1',
      constraint: 'necessity includes contingency as determinateness',
      predicate: 'contains(necessity, contingency)',
    },
    {
      id: 'act-c-1-inv-2',
      constraint: 'actuality cannot be otherwise',
      predicate: 'not(canBeOtherwise(actuality))',
    },
  ],
  forces: [
    {
      id: 'act-c-1-force-1',
      description:
        'Necessity resolves distinctions into absolute truth of being and essence',
      type: 'sublation',
      trigger: 'difference(content, form) = transparent',
      effect: 'absoluteNecessity.truth = explicit',
      targetState: 'act-c-6',
    },
  ],
  transitions: [
    {
      id: 'act-c-1-trans-1',
      from: 'act-c-1',
      to: 'act-c-6',
      mechanism: 'sublation',
      description: 'From determinate necessity to absolute necessity as truth',
    },
  ],
  nextStates: ['act-c-6'],
  previousStates: ['act-a-9'],
  provenance: {
    topicMapId: 'act-c-1-determinate-necessity',
    lineRange: { start: 3, end: 16 },
    section: 'Absolute Necessity',
    order: 1,
  },
  description: ABSOLUTE_NECESSITY_TOPIC_MAP.entries[0]?.description,
  keyPoints: ABSOLUTE_NECESSITY_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'act-c-6',
  title: 'Absolute necessity as truth of being and essence',
  concept: 'AbsoluteNecessityTruth',
  phase: 'appearance',
  moments: [
    {
      name: 'beingEssenceIdentity',
      definition: 'Pure being and pure essence are one and the same',
      type: 'reflection',
    },
    {
      name: 'selfGrounding',
      definition: 'Absolute necessity has condition and ground only in itself',
      type: 'immanence',
      relation: 'unified',
      relatedTo: 'beingEssenceIdentity',
    },
  ],
  invariants: [
    {
      id: 'act-c-6-inv-1',
      constraint: 'absolutely necessary is because it is',
      predicate: 'becauseItIs(absolutelyNecessary)',
    },
    {
      id: 'act-c-6-inv-2',
      constraint: 'possibility and actuality coincide in self-reference',
      predicate: 'equals(possibility, actuality)',
    },
  ],
  forces: [
    {
      id: 'act-c-6-force-1',
      description:
        'Absolute necessity externalizes as the transition to substance',
      type: 'reflection',
      trigger: 'absoluteNecessity.negativityBreaksForth = true',
      effect: 'substance.transitionPrepared = true',
      targetState: 'act-c-11',
    },
  ],
  transitions: [
    {
      id: 'act-c-6-trans-1',
      from: 'act-c-6',
      to: 'act-c-11',
      mechanism: 'reflection',
      description: 'From absolute necessity truth to substance transition',
    },
  ],
  nextStates: ['act-c-11'],
  previousStates: ['act-c-1'],
  provenance: {
    topicMapId: 'act-c-6-absolute-necessity-truth',
    lineRange: { start: 105, end: 127 },
    section: 'Absolute Necessity',
    order: 6,
  },
  description: ABSOLUTE_NECESSITY_TOPIC_MAP.entries[5]?.description,
  keyPoints: ABSOLUTE_NECESSITY_TOPIC_MAP.entries[5]?.keyPoints,
};

const state3: DialecticState = {
  id: 'act-c-11',
  title: 'Substance as identity of being in negation',
  concept: 'TransitionToSubstance',
  phase: 'appearance',
  moments: [
    {
      name: 'substanceIdentity',
      definition: 'Identity of being with itself in negation is substance',
      type: 'determination',
    },
    {
      name: 'selfExternalizingMovement',
      definition: 'Necessity reveals itself through its own externalization',
      type: 'process',
      relation: 'transitions',
      relatedTo: 'substanceIdentity',
    },
  ],
  invariants: [
    {
      id: 'act-c-11-inv-1',
      constraint: 'unity in contingency refers to itself as substance',
      predicate: 'equals(unity(negation, contingency), substance)',
    },
    {
      id: 'act-c-11-inv-2',
      constraint: 'transition is the absolute own exposition',
      predicate: 'equals(transition, absoluteOwnExposition)',
    },
  ],
  forces: [
    {
      id: 'act-c-11-force-1',
      description: 'Transition culminates in substantial relation',
      type: 'passover',
      trigger: 'substanceIdentity.established = true',
      effect: 'relationOfSubstantiality.emerges = true',
      targetState: 'sub-a-1',
    },
  ],
  transitions: [
    {
      id: 'act-c-11-trans-1',
      from: 'act-c-11',
      to: 'sub-a-1',
      mechanism: 'passover',
      description: 'From absolute necessity to relation of substantiality',
    },
  ],
  nextStates: ['sub-a-1'],
  previousStates: ['act-c-6'],
  provenance: {
    topicMapId: 'act-c-11-substance',
    lineRange: { start: 223, end: 231 },
    section: 'Absolute Necessity',
    order: 11,
  },
  description: ABSOLUTE_NECESSITY_TOPIC_MAP.entries[10]?.description,
  keyPoints: ABSOLUTE_NECESSITY_TOPIC_MAP.entries[10]?.keyPoints,
};

export const absoluteNecessityIR: DialecticIR = {
  id: 'absolute-necessity-ir',
  title:
    'Absolute Necessity IR: Determinate Necessity, Truth, Substance Transition',
  section: 'ESSENCE - C. ACTUALITY - C. Absolute Necessity',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'absolute-necessity.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'act-c-1': 'appearance',
      'act-c-6': 'appearance',
      'act-c-11': 'appearance',
    },
  },
};

export const absoluteNecessityStates = {
  'act-c-1': state1,
  'act-c-6': state2,
  'act-c-11': state3,
};
