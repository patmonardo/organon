import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { PARTICULAR_TOPIC_MAP } from './sources/particular-topic-map';

const state1: DialecticState = {
  id: 'part-1',
  title: "Particularity as universal's immanent determinateness",
  concept: 'ImmanentParticularity',
  phase: 'subject',
  moments: [
    {
      name: 'particularity',
      definition:
        "Determinateness of the concept as universal's own immanent moment",
      type: 'determination',
    },
    {
      name: 'selfWithItself',
      definition:
        'Universal is with itself in particularity, not limited by an external other',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'particularity',
    },
  ],
  invariants: [
    {
      id: 'part-1-inv-1',
      constraint: 'particularity belongs to conceptive determinateness',
      predicate: 'equals(particularity, conceptiveDeterminateness)',
    },
    {
      id: 'part-1-inv-2',
      constraint: 'particularity is not external limit',
      predicate: 'not(externalLimit(particularity))',
    },
  ],
  forces: [
    {
      id: 'part-1-force-1',
      description:
        'Immanent determinateness unfolds into totality/completeness',
      type: 'mediation',
      trigger: 'particularity.immanent = true',
      effect: 'totality.emerges = true',
      targetState: 'part-4',
    },
  ],
  transitions: [
    {
      id: 'part-1-trans-1',
      from: 'part-1',
      to: 'part-4',
      mechanism: 'mediation',
      description: 'From immanent particularity to true logical division',
    },
  ],
  nextStates: ['part-4'],
  previousStates: ['univ-11'],
  provenance: {
    topicMapId: 'part-1-determinateness-immanent',
    lineRange: { start: 2, end: 9 },
    section: 'The Particular Concept',
    order: 1,
  },
  description: PARTICULAR_TOPIC_MAP.entries[0]?.description,
  keyPoints: PARTICULAR_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'part-4',
  title: 'True logical division and one determinateness',
  concept: 'LogicalDivision',
  phase: 'subject',
  moments: [
    {
      name: 'coordinatedParticulars',
      definition:
        'Universal and particular stand as two particulars, coordinated and subordinated',
      type: 'determination',
    },
    {
      name: 'simpleNegativity',
      definition:
        'Their opposition is one determinateness: negativity in simple form',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'coordinatedParticulars',
    },
  ],
  invariants: [
    {
      id: 'part-4-inv-1',
      constraint: 'true division is concept self-division',
      predicate: 'equals(trueDivision, conceptSelfDivision)',
    },
    {
      id: 'part-4-inv-2',
      constraint: 'opposed sides share one essential determinateness',
      predicate: 'equals(determinateness(opposedSides), oneNegativity)',
    },
  ],
  forces: [
    {
      id: 'part-4-force-1',
      description:
        'Division intensifies into abstract universality and understanding',
      type: 'reflection',
      trigger: 'difference.fixed = true',
      effect: 'understandingMoment.emerges = true',
      targetState: 'part-10',
    },
  ],
  transitions: [
    {
      id: 'part-4-trans-1',
      from: 'part-4',
      to: 'part-10',
      mechanism: 'reflection',
      description:
        'From logical division to understanding/abstract universality',
    },
  ],
  nextStates: ['part-10'],
  previousStates: ['part-1'],
  provenance: {
    topicMapId: 'part-4-true-logical-division',
    lineRange: { start: 73, end: 95 },
    section: 'The Particular Concept',
    order: 4,
  },
  description: PARTICULAR_TOPIC_MAP.entries[3]?.description,
  keyPoints: PARTICULAR_TOPIC_MAP.entries[3]?.keyPoints,
};

const state3: DialecticState = {
  id: 'part-12',
  title: 'Absolute turning back yields singularity',
  concept: 'TransitionToSingularity',
  phase: 'subject',
  moments: [
    {
      name: 'selfReferringDeterminateness',
      definition:
        'Determinate universality posited for itself as absolute negativity',
      type: 'determination',
    },
    {
      name: 'absoluteTurningBack',
      definition:
        'Concept turns back absolutely into itself and posits loss as singularity',
      type: 'sublation',
      relation: 'transforms',
      relatedTo: 'selfReferringDeterminateness',
    },
  ],
  invariants: [
    {
      id: 'part-12-inv-1',
      constraint: 'self-referring determinateness is singularity',
      predicate: 'equals(selfReferringDeterminateness, singularity)',
    },
    {
      id: 'part-12-inv-2',
      constraint:
        'particularity and singularity immediately pass into each other',
      predicate: 'passesInto(particularity, singularity)',
    },
  ],
  forces: [
    {
      id: 'part-12-force-1',
      description: 'Turning back passes over into singular concept',
      type: 'passover',
      trigger: 'absoluteTurningBack.established = true',
      effect: 'singularConcept.emerges = true',
      targetState: 'sing-1',
    },
  ],
  transitions: [
    {
      id: 'part-12-trans-1',
      from: 'part-12',
      to: 'sing-1',
      mechanism: 'passover',
      description: 'From particular concept to singular concept',
    },
  ],
  nextStates: ['sing-1'],
  previousStates: ['part-4'],
  provenance: {
    topicMapId: 'part-12-transition-singularity',
    lineRange: { start: 428, end: 453 },
    section: 'The Particular Concept',
    order: 12,
  },
  description: PARTICULAR_TOPIC_MAP.entries[11]?.description,
  keyPoints: PARTICULAR_TOPIC_MAP.entries[11]?.keyPoints,
};

export const particularIR: DialecticIR = {
  id: 'particular-ir',
  title:
    'Particular IR: Immanent Determinateness, Division, Singularity Transition',
  section: 'CONCEPT - SUBJECTIVITY - A. The Concept - 2. The Particular',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'particular.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'part-1': 'subject',
      'part-4': 'subject',
      'part-12': 'subject',
    },
  },
};

export const particularStates = {
  'part-1': state1,
  'part-4': state2,
  'part-12': state3,
};
