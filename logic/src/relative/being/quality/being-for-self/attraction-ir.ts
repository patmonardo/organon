import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { attractionTopicMap } from './sources/attraction-topic-map';

const state1: DialecticState = {
  id: 'attraction-a',
  title: 'Mutual exclusion as unstable self-preservation',
  concept: 'ExclusionOfOne',
  phase: 'quality',
  moments: [
    {
      name: 'mutualRepulsion',
      definition:
        'Many ones mutually repel and preserve themselves only through reciprocal negation',
      type: 'negation',
    },
    {
      name: 'dissolvingSelfPreservation',
      definition:
        'This preservation is already dissolution because repulsion is a self-relation',
      type: 'contradiction',
      relation: 'transitions',
      relatedTo: 'mutualRepulsion',
    },
  ],
  invariants: [
    {
      id: 'attraction-a-inv-1',
      constraint: 'repulsion remains connected to what it excludes',
      predicate: 'connectedExclusion(repulsion)',
    },
    {
      id: 'attraction-a-inv-2',
      constraint:
        'self-preservation through exclusion implies internal collapse',
      predicate: 'preservationImpliesDissolution(ones)',
    },
  ],
  forces: [
    {
      id: 'attraction-a-force-1',
      description:
        'Repulsion crossing into identity posits attraction as its own truth',
      type: 'sublation',
      trigger: 'repulsion.crossesIntoSelfIdentity = true',
      effect: 'attractingOne.emerges = true',
      targetState: 'attraction-b',
    },
  ],
  transitions: [
    {
      id: 'attraction-a-trans-1',
      from: 'attraction-a',
      to: 'attraction-b',
      mechanism: 'sublation',
      description: 'From exclusion to attracting one',
    },
  ],
  nextStates: ['attraction-b'],
  previousStates: ['one-many-c'],
  provenance: {
    topicMapId: 'attraction-a-2',
    lineRange: { start: 65, end: 136 },
    section: 'C. a. Exclusion of the one',
    order: 1,
  },
  description: attractionTopicMap[1]?.description,
  keyPoints: attractionTopicMap[1]?.keyPoints,
};

const state2: DialecticState = {
  id: 'attraction-b',
  title: 'The one one of attraction',
  concept: 'AttractingOne',
  phase: 'quality',
  moments: [
    {
      name: 'mediatedOne',
      definition:
        'Attraction posits a mediated one that includes repulsion in its own determination',
      type: 'mediation',
    },
    {
      name: 'realizedIdeality',
      definition:
        'The one one is realized ideality, preserving plurality within gathered unity',
      type: 'sublation',
      relation: 'contains',
      relatedTo: 'mediatedOne',
    },
  ],
  invariants: [
    {
      id: 'attraction-b-inv-1',
      constraint: 'attraction requires ongoing repulsion as condition',
      predicate: 'requires(attraction, repulsion)',
    },
    {
      id: 'attraction-b-inv-2',
      constraint:
        'unity preserves many as moments rather than annihilating them',
      predicate: 'preservingUnity(one, many)',
    },
  ],
  forces: [
    {
      id: 'attraction-b-force-1',
      description:
        'Mediated one compels explicit conceptual connection of repulsion and attraction',
      type: 'mediation',
      trigger: 'unityOfForces.requiresExplicitForm = true',
      effect: 'connectionChapter.explicit = true',
      targetState: 'attraction-c',
    },
  ],
  transitions: [
    {
      id: 'attraction-b-trans-1',
      from: 'attraction-b',
      to: 'attraction-c',
      mechanism: 'mediation',
      description: 'From attracting one to explicit force-unity',
    },
  ],
  nextStates: ['attraction-c'],
  previousStates: ['attraction-a'],
  provenance: {
    topicMapId: 'attraction-b',
    lineRange: { start: 139, end: 211 },
    section: 'b. The one one of attraction',
    order: 2,
  },
  description: attractionTopicMap[2]?.description,
  keyPoints: attractionTopicMap[2]?.keyPoints,
};

const state3: DialecticState = {
  id: 'attraction-c',
  title: 'Repulsion and attraction as self-mediation',
  concept: 'UnifiedForceProcess',
  phase: 'quality',
  moments: [
    {
      name: 'inseparableDuality',
      definition:
        'Repulsion and attraction are distinct only as mutually mediating moments of one process',
      type: 'mediation',
    },
    {
      name: 'selfPresupposingUnity',
      definition:
        'Each force presupposes and contains the other as its own moment',
      type: 'reflection',
      relation: 'contains',
      relatedTo: 'inseparableDuality',
    },
  ],
  invariants: [
    {
      id: 'attraction-c-inv-1',
      constraint: 'each force is self-mediation through the other',
      predicate: 'selfMediationThroughOther(repulsion, attraction)',
    },
  ],
  forces: [
    {
      id: 'attraction-c-force-1',
      description:
        'Unified qualitative force-process passes to quantity as stabilized determinateness',
      type: 'passover',
      trigger: 'qualitativeForceUnity.stabilized = true',
      effect: 'quantityChapter.initiated = true',
      targetState: 'quantity-1',
    },
  ],
  transitions: [
    {
      id: 'attraction-c-trans-1',
      from: 'attraction-c',
      to: 'quantity-1',
      mechanism: 'passover',
      description: 'From force-unity to quantity',
    },
  ],
  nextStates: ['quantity-1'],
  previousStates: ['attraction-b'],
  provenance: {
    topicMapId: 'attraction-c-2',
    lineRange: { start: 303, end: 393 },
    section: 'c. The connection of repulsion and attraction',
    order: 3,
  },
  description: attractionTopicMap[4]?.description,
  keyPoints: attractionTopicMap[4]?.keyPoints,
};

export const attractionIR: DialecticIR = {
  id: 'attraction-ir',
  title: 'Attraction IR: Exclusion, Attraction, and Their Unity',
  section: 'BEING - QUALITY - C. Being-for-Self - C. Repulsion and Attraction',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'attraction.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'attraction-a': 'quality',
      'attraction-b': 'quality',
      'attraction-c': 'quality',
    },
  },
};

export const attractionStates = {
  'attraction-a': state1,
  'attraction-b': state2,
  'attraction-c': state3,
};
