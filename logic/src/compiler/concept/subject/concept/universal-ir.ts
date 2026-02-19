import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { UNIVERSAL_TOPIC_MAP } from './sources/universal-topic-map';

const state1: DialecticState = {
  id: 'univ-1',
  title: 'Pure concept as absolutely infinite and unconditioned',
  concept: 'PureConcept',
  phase: 'subject',
  moments: [
    {
      name: 'pureConcept',
      definition: 'Concept as free, unconditioned, and absolutely infinite',
      type: 'determination',
    },
    {
      name: 'geneticMediation',
      definition:
        'Being and essence are sublated into conceptive self-mediation',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'pureConcept',
    },
  ],
  invariants: [
    {
      id: 'univ-1-inv-1',
      constraint: 'concept is unconditioned',
      predicate: 'equals(concept, unconditioned)',
    },
    {
      id: 'univ-1-inv-2',
      constraint: 'determinateness is self-referring',
      predicate: 'selfReferring(determinateness)',
    },
  ],
  forces: [
    {
      id: 'univ-1-force-1',
      description: 'Pure concept determines itself as universality',
      type: 'mediation',
      trigger: 'concept.selfIdentity = explicit',
      effect: 'universality.emerges = true',
      targetState: 'univ-6',
    },
  ],
  transitions: [
    {
      id: 'univ-1-trans-1',
      from: 'univ-1',
      to: 'univ-6',
      mechanism: 'mediation',
      description: 'From pure concept to universal as creative principle',
    },
  ],
  nextStates: ['univ-6'],
  previousStates: ['sub-c-5'],
  provenance: {
    topicMapId: 'univ-1-genesis-absoluteness',
    lineRange: { start: 1, end: 28 },
    section: 'The Universal Concept',
    order: 1,
  },
  description: UNIVERSAL_TOPIC_MAP.entries[0]?.description,
  keyPoints: UNIVERSAL_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'univ-6',
  title: 'Universal as essence and creative principle',
  concept: 'CreativeUniversality',
  phase: 'subject',
  moments: [
    {
      name: 'creativePrinciple',
      definition: 'Universal as informing, self-mediating absolute negativity',
      type: 'determination',
    },
    {
      name: 'freePowerLove',
      definition: 'Universal as free power that preserves itself in its other',
      type: 'reflection',
      relation: 'contains',
      relatedTo: 'creativePrinciple',
    },
  ],
  invariants: [
    {
      id: 'univ-6-inv-1',
      constraint: 'universal is essence of its determination',
      predicate: 'equals(universal, essence(determination))',
    },
    {
      id: 'univ-6-inv-2',
      constraint: 'determinateness is positedness, not external limitation',
      predicate: 'equals(determinateness, positedness)',
    },
  ],
  forces: [
    {
      id: 'univ-6-force-1',
      description:
        'Creative universality internalizes determinateness as particularity',
      type: 'reflection',
      trigger: 'universal.containsDeterminateness = true',
      effect: 'particularity.emerges = true',
      targetState: 'univ-11',
    },
  ],
  transitions: [
    {
      id: 'univ-6-trans-1',
      from: 'univ-6',
      to: 'univ-11',
      mechanism: 'reflection',
      description:
        'From creative universality to self-differentiating universals',
    },
  ],
  nextStates: ['univ-11'],
  previousStates: ['univ-1'],
  provenance: {
    topicMapId: 'univ-6-essence-creative-principle',
    lineRange: { start: 118, end: 155 },
    section: 'The Universal Concept',
    order: 6,
  },
  description: UNIVERSAL_TOPIC_MAP.entries[5]?.description,
  keyPoints: UNIVERSAL_TOPIC_MAP.entries[5]?.keyPoints,
};

const state3: DialecticState = {
  id: 'univ-11',
  title: 'Universal freely differentiates into universal differences',
  concept: 'SelfDifferentiatingUniversal',
  phase: 'subject',
  moments: [
    {
      name: 'internalDifferentiation',
      definition:
        'Universal differentiates itself through self-referring negativity',
      type: 'process',
    },
    {
      name: 'finiteUniversality',
      definition:
        'Finite isolates are forms through which concept clothes differences',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'internalDifferentiation',
    },
  ],
  invariants: [
    {
      id: 'univ-11-inv-1',
      constraint: 'differences are themselves universals',
      predicate: 'forall(difference, universal(difference))',
    },
    {
      id: 'univ-11-inv-2',
      constraint: 'true higher universal is inward second negation',
      predicate: 'equals(higherUniversal, inwardSecondNegation)',
    },
  ],
  forces: [
    {
      id: 'univ-11-force-1',
      description:
        'Differentiated universal passes over to explicit particularity',
      type: 'passover',
      trigger: 'universal.differencesFixed = true',
      effect: 'particularConcept.emerges = true',
      targetState: 'part-1',
    },
  ],
  transitions: [
    {
      id: 'univ-11-trans-1',
      from: 'univ-11',
      to: 'part-1',
      mechanism: 'passover',
      description: 'From universal concept to particular concept',
    },
  ],
  nextStates: ['part-1'],
  previousStates: ['univ-6'],
  provenance: {
    topicMapId: 'univ-11-creative-differentiation',
    lineRange: { start: 279, end: 296 },
    section: 'The Universal Concept',
    order: 11,
  },
  description: UNIVERSAL_TOPIC_MAP.entries[10]?.description,
  keyPoints: UNIVERSAL_TOPIC_MAP.entries[10]?.keyPoints,
};

export const universalIR: DialecticIR = {
  id: 'universal-ir',
  title:
    'Universal IR: Pure Concept, Creative Universality, Self-Differentiation',
  section: 'CONCEPT - SUBJECTIVITY - A. The Concept - 1. The Universal',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'universal.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'univ-1': 'subject',
      'univ-6': 'subject',
      'univ-11': 'subject',
    },
  },
};

export const universalStates = {
  'univ-1': state1,
  'univ-6': state2,
  'univ-11': state3,
};
