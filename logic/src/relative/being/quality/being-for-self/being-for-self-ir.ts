import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { beingForSelfTopicMap } from './sources/being-for-self-topic-map';

const state1: DialecticState = {
  id: 'being-for-self-a',
  title: 'Being-for-itself as sublated otherness',
  concept: 'GeneralBeingForSelf',
  phase: 'quality',
  moments: [
    {
      name: 'infiniteTurningBack',
      definition:
        'Being-for-itself is the infinite turning back into itself by sublating otherness',
      type: 'sublation',
    },
    {
      name: 'idealityInConsciousness',
      definition:
        'The other persists only as moment, giving the form of ideality',
      type: 'reflection',
      relation: 'contains',
      relatedTo: 'infiniteTurningBack',
    },
  ],
  invariants: [
    {
      id: 'being-for-self-a-inv-1',
      constraint: 'otherness is retained only as sublated moment',
      predicate: 'retainedAsSublated(otherness)',
    },
    {
      id: 'being-for-self-a-inv-2',
      constraint: 'self-reference is constituted through negation of limit',
      predicate: 'selfReferenceThroughNegation(limit)',
    },
  ],
  forces: [
    {
      id: 'being-for-self-a-force-1',
      description:
        'Infinite self-reference must be articulated as existence bent back into being-for-one',
      type: 'mediation',
      trigger: 'selfReference.requiresExistentialMoment = true',
      effect: 'beingForOne.explicit = true',
      targetState: 'being-for-self-b',
    },
  ],
  transitions: [
    {
      id: 'being-for-self-a-trans-1',
      from: 'being-for-self-a',
      to: 'being-for-self-b',
      mechanism: 'mediation',
      description: 'From general being-for-itself to being-for-one',
    },
  ],
  nextStates: ['being-for-self-b'],
  previousStates: ['affirmative-infinity-16'],
  provenance: {
    topicMapId: 'being-for-self-a',
    lineRange: { start: 49, end: 103 },
    section: 'A. BEING-FOR-ITSELF AS SUCH',
    order: 1,
  },
  description: beingForSelfTopicMap[0]?.description,
  keyPoints: beingForSelfTopicMap[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'being-for-self-b',
  title: 'Being-for-one as inseparable moment of ideality',
  concept: 'BeingForOne',
  phase: 'quality',
  moments: [
    {
      name: 'forOneNotForAnother',
      definition:
        'Ideality is for-one, yet this one is only the selfsame being-for-itself',
      type: 'determination',
    },
    {
      name: 'inseparableUnity',
      definition:
        'Being-for-itself and being-for-one are not two separate determinacies',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'forOneNotForAnother',
    },
  ],
  invariants: [
    {
      id: 'being-for-self-b-inv-1',
      constraint: 'being-for-one and being-for-itself are internally unified',
      predicate: 'internalUnity(beingForOne, beingForItself)',
    },
    {
      id: 'being-for-self-b-inv-2',
      constraint: 'the one-for-which is not an external other',
      predicate: 'notExternalOther(oneForWhich)',
    },
  ],
  forces: [
    {
      id: 'being-for-self-b-force-1',
      description:
        'Inseparable ideality collapses into immediate abstract unity as the one',
      type: 'sublation',
      trigger: 'moments.sunkIntoImmediacy = true',
      effect: 'one.emerges = true',
      targetState: 'being-for-self-c',
    },
  ],
  transitions: [
    {
      id: 'being-for-self-b-trans-1',
      from: 'being-for-self-b',
      to: 'being-for-self-c',
      mechanism: 'sublation',
      description: 'From being-for-one to the one',
    },
  ],
  nextStates: ['being-for-self-c'],
  previousStates: ['being-for-self-a'],
  provenance: {
    topicMapId: 'being-for-self-b',
    lineRange: { start: 129, end: 192 },
    section: 'b. Being-for-one',
    order: 2,
  },
  description: beingForSelfTopicMap[2]?.description,
  keyPoints: beingForSelfTopicMap[2]?.keyPoints,
};

const state3: DialecticState = {
  id: 'being-for-self-c',
  title: 'The one as abstract limit of being-for-itself',
  concept: 'TheOne',
  phase: 'quality',
  moments: [
    {
      name: 'immediateUnity',
      definition:
        'Moments collapse into simple immediacy, yielding the one as existent-for-itself',
      type: 'determination',
    },
    {
      name: 'selfReferentialContradiction',
      definition:
        'The one contains moments that are both identical and opposed',
      type: 'contradiction',
      relation: 'transitions',
      relatedTo: 'immediateUnity',
    },
  ],
  invariants: [
    {
      id: 'being-for-self-c-inv-1',
      constraint: 'the one is simple self-reference of sublating negation',
      predicate: 'simpleSelfReference(theOne)',
    },
  ],
  forces: [
    {
      id: 'being-for-self-c-force-1',
      description:
        'The one must externalize its contradiction into the many and their relation',
      type: 'passover',
      trigger: 'one.requiresExternalization = true',
      effect: 'oneManyDialectic.initiated = true',
      targetState: 'one-many-a',
    },
  ],
  transitions: [
    {
      id: 'being-for-self-c-trans-1',
      from: 'being-for-self-c',
      to: 'one-many-a',
      mechanism: 'passover',
      description: 'From the one to one-and-many',
    },
  ],
  nextStates: ['one-many-a'],
  previousStates: ['being-for-self-b'],
  provenance: {
    topicMapId: 'being-for-self-c',
    lineRange: { start: 194, end: 231 },
    section: 'c. The one',
    order: 3,
  },
  description: beingForSelfTopicMap[3]?.description,
  keyPoints: beingForSelfTopicMap[3]?.keyPoints,
};

export const beingForSelfIR: DialecticIR = {
  id: 'being-for-self-ir',
  title: 'Being-for-Self IR: Self-Reference, Being-for-One, The One',
  section: 'BEING - QUALITY - C. Being-for-Self',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'being-for-self.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'being-for-self-a': 'quality',
      'being-for-self-b': 'quality',
      'being-for-self-c': 'quality',
    },
  },
};

export const beingForSelfStates = {
  'being-for-self-a': state1,
  'being-for-self-b': state2,
  'being-for-self-c': state3,
};
