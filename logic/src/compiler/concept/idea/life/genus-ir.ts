import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { GENUS_TOPIC_MAP } from './sources/genus-topic-map';

const state1: DialecticState = {
  id: 'genus-1',
  title: 'Genus as identity, duplication, and third stage of life',
  concept: 'GenusIdentityDuplication',
  phase: 'subject',
  moments: [
    {
      name: 'genusIdentity',
      definition:
        'Life determines itself as identity with what had stood as indifferent otherness',
      type: 'determination',
    },
    {
      name: 'selfRelationAsDuplication',
      definition:
        'Individual life relates to itself as another living being, generating duplicated self-reference',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'genusIdentity',
    },
  ],
  invariants: [
    {
      id: 'genus-1-inv-1',
      constraint:
        'genus is the third stage and truth of life in immediate form',
      predicate: 'thirdStageTruthOfLife(genus)',
    },
    {
      id: 'genus-1-inv-2',
      constraint:
        'externality persists only as immanent moment of living totality',
      predicate: 'immanentized(externality, livingTotality)',
    },
  ],
  forces: [
    {
      id: 'genus-1-force-1',
      description:
        'Duplicated self-relation intensifies into contradiction of self-feeling across individuals',
      type: 'contradiction',
      trigger: 'selfFeeling.identityInAnother = demanded',
      effect: 'genusImpulse.emerges = true',
      targetState: 'genus-2',
    },
  ],
  transitions: [
    {
      id: 'genus-1-trans-1',
      from: 'genus-1',
      to: 'genus-2',
      mechanism: 'contradiction',
      description: 'From duplicated identity to propagative genus impulse',
    },
  ],
  nextStates: ['genus-2'],
  previousStates: ['life-process-3'],
  provenance: {
    topicMapId: 'genus-1-identity-duplication',
    lineRange: { start: 3, end: 49 },
    section: 'The Genus',
    order: 1,
  },
  description: GENUS_TOPIC_MAP.entries[0]?.description,
  keyPoints: GENUS_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'genus-2',
  title: 'Propagation and germ as reflective actuality of genus',
  concept: 'GenusPropagationAndGerm',
  phase: 'subject',
  moments: [
    {
      name: 'germAsSubjectiveTotality',
      definition:
        'The germ contains complete individuality in conceptual inwardness before developed external articulation',
      type: 'reflection',
    },
    {
      name: 'propagationAsGenusActuality',
      definition:
        'Genus acquires actuality through reflection into itself as propagation of living individuality',
      type: 'process',
      relation: 'transforms',
      relatedTo: 'germAsSubjectiveTotality',
    },
  ],
  invariants: [
    {
      id: 'genus-2-inv-1',
      constraint:
        'realized identity arises only through sublation of isolated singularity',
      predicate: 'requiresSublationOfIsolation(realizedGenusIdentity)',
    },
    {
      id: 'genus-2-inv-2',
      constraint:
        'propagation repeats life while also preparing elevation beyond immediacy',
      predicate: 'doubleCharacter(propagation, repetition, elevation)',
    },
  ],
  forces: [
    {
      id: 'genus-2-force-1',
      description:
        'Propagation culminates in death of immediate life and transition to explicit universality',
      type: 'sublation',
      trigger: 'immediateIndividuality.exhaustedInGenusProcess = true',
      effect: 'ideaOfCognition.thresholdReached = true',
      targetState: 'genus-3',
    },
  ],
  transitions: [
    {
      id: 'genus-2-trans-1',
      from: 'genus-2',
      to: 'genus-3',
      mechanism: 'sublation',
      description:
        'From propagation cycle to explicit universality of genus-for-itself',
    },
  ],
  nextStates: ['genus-3'],
  previousStates: ['genus-1'],
  provenance: {
    topicMapId: 'genus-2-propagation-germ',
    lineRange: { start: 51, end: 124 },
    section: 'The Genus',
    order: 2,
  },
  description: GENUS_TOPIC_MAP.entries[1]?.description,
  keyPoints: GENUS_TOPIC_MAP.entries[1]?.keyPoints,
};

const state3: DialecticState = {
  id: 'genus-3',
  title: 'Death of life and transition to idea of cognition',
  concept: 'TransitionToCognition',
  phase: 'subject',
  moments: [
    {
      name: 'negativeIdentityOfGenus',
      definition:
        'Genus asserts itself by generating and sublating singular individuals in one negative unity',
      type: 'negation',
    },
    {
      name: 'comingToBeOfSpirit',
      definition:
        'The perishing of immediate life is simultaneously the emergence of spirit as explicit universal idea',
      type: 'passover',
      relation: 'passesOver',
      relatedTo: 'negativeIdentityOfGenus',
    },
  ],
  invariants: [
    {
      id: 'genus-3-inv-1',
      constraint:
        'universality is now posited for itself rather than only implicit in genus',
      predicate: 'forItself(universality)',
    },
    {
      id: 'genus-3-inv-2',
      constraint:
        'the idea now relates to itself explicitly as idea in cognition',
      predicate: 'selfRelatedAsIdea(idea)',
    },
  ],
  forces: [
    {
      id: 'genus-3-force-1',
      description:
        'Life as genus passes into the introductory moment of cognition',
      type: 'passover',
      trigger: 'lifeUniversality.explicitForItself = true',
      effect: 'cognitionBegins = true',
      targetState: 'cognition-1',
    },
  ],
  transitions: [
    {
      id: 'genus-3-trans-1',
      from: 'genus-3',
      to: 'cognition-1',
      mechanism: 'passover',
      description: 'From genus process to the Idea of Cognition',
    },
  ],
  nextStates: ['cognition-1'],
  previousStates: ['genus-2'],
  provenance: {
    topicMapId: 'genus-3-transition-cognition',
    lineRange: { start: 126, end: 156 },
    section: 'The Genus',
    order: 3,
  },
  description: GENUS_TOPIC_MAP.entries[2]?.description,
  keyPoints: GENUS_TOPIC_MAP.entries[2]?.keyPoints,
};

export const genusIR: DialecticIR = {
  id: 'genus-ir',
  title: 'Genus IR: Identity/Duplication, Propagation, Transition to Cognition',
  section: 'CONCEPT - IDEA - A. Life - C. The Genus',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'genus.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'genus-1': 'subject',
      'genus-2': 'subject',
      'genus-3': 'subject',
    },
  },
};

export const genusStates = {
  'genus-1': state1,
  'genus-2': state2,
  'genus-3': state3,
};
