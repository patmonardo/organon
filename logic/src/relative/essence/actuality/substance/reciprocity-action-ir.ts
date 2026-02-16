import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { RECIPROCITY_ACTION_TOPIC_MAP } from './sources/reciprocity-action-topic-map';

const state1: DialecticState = {
  id: 'sub-c-1',
  title: 'Mechanism is sublated in reciprocity',
  concept: 'MechanismSublated',
  phase: 'appearance',
  moments: [
    {
      name: 'mechanismExternality',
      definition:
        'Finite causality appears as external transposition between substances',
      type: 'externality',
    },
    {
      name: 'reciprocity',
      definition:
        'Reciprocity sublates mechanism through mediated originariness',
      type: 'sublation',
      relation: 'negates',
      relatedTo: 'mechanismExternality',
    },
  ],
  invariants: [
    {
      id: 'sub-c-1-inv-1',
      constraint: 'reciprocity removes immediate substantial persistence',
      predicate: 'sublates(reciprocity, immediateSubstantialPersistence)',
    },
    {
      id: 'sub-c-1-inv-2',
      constraint: 'originariness mediates itself through negation',
      predicate: 'mediatesThrough(originariness, negation)',
    },
  ],
  forces: [
    {
      id: 'sub-c-1-force-1',
      description: 'Reciprocal conditioning reveals concept as attained',
      type: 'mediation',
      trigger: 'activePassive.identity = explicit',
      effect: 'conceptAttained.emerges = true',
      targetState: 'sub-c-3',
    },
  ],
  transitions: [
    {
      id: 'sub-c-1-trans-1',
      from: 'sub-c-1',
      to: 'sub-c-3',
      mechanism: 'mediation',
      description: 'From mechanism sublation to concept attainment',
    },
  ],
  nextStates: ['sub-c-3'],
  previousStates: ['sub-b-9'],
  provenance: {
    topicMapId: 'sub-c-1-mechanism-sublated',
    lineRange: { start: 3, end: 17 },
    section: 'Reciprocity of Action',
    order: 1,
  },
  description: RECIPROCITY_ACTION_TOPIC_MAP.entries[0]?.description,
  keyPoints: RECIPROCITY_ACTION_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'sub-c-3',
  title: 'Reciprocity attains concept and elevates necessity to freedom',
  concept: 'ConceptAttained',
  phase: 'appearance',
  moments: [
    {
      name: 'returnedCausality',
      definition: 'Causality returns to the absolute concept',
      type: 'reflection',
    },
    {
      name: 'freedomEmergence',
      definition: 'Inner necessity manifests as free self-related movement',
      type: 'immanence',
      relation: 'transforms',
      relatedTo: 'returnedCausality',
    },
  ],
  invariants: [
    {
      id: 'sub-c-3-inv-1',
      constraint: 'necessity becomes freedom through manifestation',
      predicate: 'becomes(necessity, freedom)',
    },
    {
      id: 'sub-c-3-inv-2',
      constraint: 'originative causality arises from and returns into negation',
      predicate: 'cycles(originativeCausality, negation)',
    },
  ],
  forces: [
    {
      id: 'sub-c-3-force-1',
      description:
        'Reciprocity differentiates into universal, singular, particular',
      type: 'sublation',
      trigger: 'oneReflection.selfDifferentiates = true',
      effect: 'conceptTriad.emerges = true',
      targetState: 'sub-c-5',
    },
  ],
  transitions: [
    {
      id: 'sub-c-3-trans-1',
      from: 'sub-c-3',
      to: 'sub-c-5',
      mechanism: 'sublation',
      description: 'From attained concept to explicit concept triad',
    },
  ],
  nextStates: ['sub-c-5'],
  previousStates: ['sub-c-1'],
  provenance: {
    topicMapId: 'sub-c-3-concept-attained',
    lineRange: { start: 62, end: 81 },
    section: 'Reciprocity of Action',
    order: 3,
  },
  description: RECIPROCITY_ACTION_TOPIC_MAP.entries[2]?.description,
  keyPoints: RECIPROCITY_ACTION_TOPIC_MAP.entries[2]?.keyPoints,
};

const state3: DialecticState = {
  id: 'sub-c-5',
  title: 'Concept as universal singular and particular',
  concept: 'ConceptTransition',
  phase: 'subject',
  moments: [
    {
      name: 'universality',
      definition: 'Self-identical totality as universal',
      type: 'determination',
    },
    {
      name: 'singularityParticularity',
      definition: 'Singular negativity and particular unity in one reflection',
      type: 'mediation',
      relation: 'unified',
      relatedTo: 'universality',
    },
  ],
  invariants: [
    {
      id: 'sub-c-5-inv-1',
      constraint: 'three totalities are one and the same reflection',
      predicate: 'equals(totalitiesThree, oneReflection)',
    },
    {
      id: 'sub-c-5-inv-2',
      constraint: 'concept opens realm of subjectivity and freedom',
      predicate: 'opens(concept, realmSubjectivityFreedom)',
    },
  ],
  forces: [],
  transitions: [],
  nextStates: [],
  previousStates: ['sub-c-3'],
  provenance: {
    topicMapId: 'sub-c-5-concept-universal-singular',
    lineRange: { start: 121, end: 162 },
    section: 'Reciprocity of Action',
    order: 5,
  },
  description: RECIPROCITY_ACTION_TOPIC_MAP.entries[4]?.description,
  keyPoints: RECIPROCITY_ACTION_TOPIC_MAP.entries[4]?.keyPoints,
};

export const reciprocityActionIR: DialecticIR = {
  id: 'reciprocity-action-ir',
  title: 'Reciprocity Action IR: Mechanism Sublation, Freedom, Concept Triad',
  section:
    'ESSENCE - C. ACTUALITY - Absolute Relation - c. Reciprocity of Action',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'reciprocity-action.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'sub-c-1': 'appearance',
      'sub-c-3': 'appearance',
      'sub-c-5': 'subject',
    },
  },
};

export const reciprocityActionStates = {
  'sub-c-1': state1,
  'sub-c-3': state2,
  'sub-c-5': state3,
};
