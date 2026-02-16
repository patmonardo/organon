import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { oneManyTopicMap } from './sources/one-many-topic-map';

const state1: DialecticState = {
  id: 'one-many-a',
  title: 'The one and the void',
  concept: 'OneAndVoid',
  phase: 'quality',
  moments: [
    {
      name: 'unalterableOne',
      definition:
        'The one is self-referential, unalterable, and excludes becoming-other',
      type: 'determination',
    },
    {
      name: 'voidAsPositedNothing',
      definition:
        'Within this immediacy, nothing appears as the void outside yet tied to the one',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'unalterableOne',
    },
  ],
  invariants: [
    {
      id: 'one-many-a-inv-1',
      constraint: 'one and void stand in one relation while remaining opposed',
      predicate: 'singleRelationWithOpposition(one, void)',
    },
    {
      id: 'one-many-a-inv-2',
      constraint: 'immediacy preserves negation as constitutive',
      predicate: 'constitutiveNegation(immediacy)',
    },
  ],
  forces: [
    {
      id: 'one-many-a-force-1',
      description:
        'Self-reference of the one externalizes as repulsion into plurality',
      type: 'negation',
      trigger: 'one.negativelyRefersToItself = true',
      effect: 'manyOnes.posited = true',
      targetState: 'one-many-b-1',
    },
  ],
  transitions: [
    {
      id: 'one-many-a-trans-1',
      from: 'one-many-a',
      to: 'one-many-b-1',
      mechanism: 'negation',
      description: 'From one-and-void to repulsion and plurality',
    },
  ],
  nextStates: ['one-many-b-1'],
  previousStates: ['being-for-self-c'],
  provenance: {
    topicMapId: 'one-many-a',
    lineRange: { start: 43, end: 82 },
    section: 'a. The one within',
    order: 1,
  },
  description: oneManyTopicMap[1]?.description,
  keyPoints: oneManyTopicMap[1]?.keyPoints,
};

const state2: DialecticState = {
  id: 'one-many-b-1',
  title: 'Repulsion and many ones',
  concept: 'RepulsivePlurality',
  phase: 'quality',
  moments: [
    {
      name: 'selfRepulsionOfOne',
      definition:
        'The one repels itself from itself, positing many ones as its own output',
      type: 'process',
    },
    {
      name: 'pluralityAsExternalInfinity',
      definition:
        'Plurality appears as externally laid-out infinity with void as limit',
      type: 'externality',
      relation: 'transitions',
      relatedTo: 'selfRepulsionOfOne',
    },
  ],
  invariants: [
    {
      id: 'one-many-b-1-inv-1',
      constraint: 'many ones are mutually presupposed through repulsion',
      predicate: 'mutualPresupposition(manyOnes)',
    },
    {
      id: 'one-many-b-1-inv-2',
      constraint: 'repulsion keeps connection while appearing as separation',
      predicate: 'connectedSeparation(repulsion)',
    },
  ],
  forces: [
    {
      id: 'one-many-b-1-force-1',
      description:
        'Repulsion crossing into identity demands attraction as self-positing-in-one',
      type: 'sublation',
      trigger: 'repulsion.crossesIntoIdentity = true',
      effect: 'attraction.emerges = true',
      targetState: 'one-many-c',
    },
  ],
  transitions: [
    {
      id: 'one-many-b-1-trans-1',
      from: 'one-many-b-1',
      to: 'one-many-c',
      mechanism: 'sublation',
      description: 'From repulsive plurality to attractional unity',
    },
  ],
  nextStates: ['one-many-c'],
  previousStates: ['one-many-a'],
  provenance: {
    topicMapId: 'one-many-c',
    lineRange: { start: 111, end: 248 },
    section: 'c. Many ones / Repulsion',
    order: 2,
  },
  description: oneManyTopicMap[3]?.description,
  keyPoints: oneManyTopicMap[3]?.keyPoints,
};

const state3: DialecticState = {
  id: 'one-many-c',
  title: 'Transition from repulsion to attraction',
  concept: 'RepulsionToAttraction',
  phase: 'quality',
  moments: [
    {
      name: 'oneOfAttraction',
      definition:
        'Attraction posits the mediated one that gathers many without abstractly swallowing them',
      type: 'mediation',
    },
    {
      name: 'unityOfRepulsionAttraction',
      definition:
        'Repulsion and attraction are inseparable moments of one process',
      type: 'sublation',
      relation: 'contains',
      relatedTo: 'oneOfAttraction',
    },
  ],
  invariants: [
    {
      id: 'one-many-c-inv-1',
      constraint: 'attraction presupposes repulsion and vice versa',
      predicate: 'mutualPresupposition(attraction, repulsion)',
    },
  ],
  forces: [
    {
      id: 'one-many-c-force-1',
      description:
        'The unity of repulsion and attraction requires explicit treatment as a dedicated chapter',
      type: 'passover',
      trigger: 'repulsionAttraction.unity.explicit = true',
      effect: 'attractionChapter.initiated = true',
      targetState: 'attraction-a',
    },
  ],
  transitions: [
    {
      id: 'one-many-c-trans-1',
      from: 'one-many-c',
      to: 'attraction-a',
      mechanism: 'passover',
      description:
        'From one-many dialectic to repulsion and attraction chapter',
    },
  ],
  nextStates: ['attraction-a'],
  previousStates: ['one-many-b-1'],
  provenance: {
    topicMapId: 'one-many-c',
    lineRange: { start: 111, end: 248 },
    section: 'c. Many ones / Repulsion',
    order: 3,
  },
  description: oneManyTopicMap[3]?.description,
  keyPoints: oneManyTopicMap[3]?.keyPoints,
};

export const oneManyIR: DialecticIR = {
  id: 'one-many-ir',
  title: 'One and Many IR: One, Void, Repulsion, Attraction',
  section: 'BEING - QUALITY - C. Being-for-Self - B. One and Many',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'one-many.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'one-many-a': 'quality',
      'one-many-b-1': 'quality',
      'one-many-c': 'quality',
    },
  },
};

export const oneManyStates = {
  'one-many-a': state1,
  'one-many-b-1': state2,
  'one-many-c': state3,
};
