import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { REFLECTION_TOPIC_MAP } from './sources/reflection-topic-map';

const state1: DialecticState = {
  id: 'refl-1',
  title: 'Universal collected into unity in reflected subjectivity',
  concept: 'CollectedReflectedUniversality',
  phase: 'subject',
  moments: [
    {
      name: 'collectedUniversality',
      definition:
        'Universal is gathered by reflection from manifold appearance',
      type: 'reflection',
    },
    {
      name: 'subsumptiveRelation',
      definition:
        'Judgment now takes the form of subsumption rather than inherence',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'collectedUniversality',
    },
  ],
  invariants: [
    {
      id: 'refl-1-inv-1',
      constraint: 'movement is in subject as reflected in-itselfness',
      predicate: 'equals(movementLocus, reflectedSubject)',
    },
    {
      id: 'refl-1-inv-2',
      constraint: 'predicate functions as essential universal basis',
      predicate: 'essentialBasis(predicate)',
    },
  ],
  forces: [
    {
      id: 'refl-1-force-1',
      description:
        'Reflected universality extends toward allness and universal judgment',
      type: 'mediation',
      trigger: 'subsumption.explicit = true',
      effect: 'universalJudgment.emerges = true',
      targetState: 'refl-9',
    },
  ],
  transitions: [
    {
      id: 'refl-1-trans-1',
      from: 'refl-1',
      to: 'refl-9',
      mechanism: 'mediation',
      description:
        'From reflected singular/particular movement to universal judgment',
    },
  ],
  nextStates: ['refl-9'],
  previousStates: ['exist-18'],
  provenance: {
    topicMapId: 'refl-1-introduction-unity',
    lineRange: { start: 3, end: 57 },
    section: 'The Judgment of Reflection',
    order: 1,
  },
  description: REFLECTION_TOPIC_MAP.entries[0]?.description,
  keyPoints: REFLECTION_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'refl-9',
  title: 'Universal judgment and allness disclose objective universality',
  concept: 'AllnessToObjectiveUniversality',
  phase: 'subject',
  moments: [
    {
      name: 'allness',
      definition: 'External universality appears first as allness of singulars',
      type: 'determination',
    },
    {
      name: 'objectiveUniversality',
      definition:
        'Singularity expands into identity with universality as objective form',
      type: 'sublation',
      relation: 'transforms',
      relatedTo: 'allness',
    },
  ],
  invariants: [
    {
      id: 'refl-9-inv-1',
      constraint: 'empirical allness alone remains externally mediated',
      predicate: 'externalMediation(empiricalAllness)',
    },
    {
      id: 'refl-9-inv-2',
      constraint:
        'objective universality requires reflected identity of singularity and universal',
      predicate:
        'equals(objectiveUniversality, reflectedIdentity(singularity, universal))',
    },
  ],
  forces: [
    {
      id: 'refl-9-force-1',
      description:
        'Objective universality sublates subsumption and forms genus identity',
      type: 'sublation',
      trigger: 'objectiveUniversality.posited = true',
      effect: 'necessityConnection.emerges = true',
      targetState: 'refl-14',
    },
  ],
  transitions: [
    {
      id: 'refl-9-trans-1',
      from: 'refl-9',
      to: 'refl-14',
      mechanism: 'sublation',
      description: 'From universal judgment to transition into necessity',
    },
  ],
  nextStates: ['refl-14'],
  previousStates: ['refl-1'],
  provenance: {
    topicMapId: 'refl-9-allness-external',
    lineRange: { start: 243, end: 295 },
    section: 'The Universal Judgment',
    order: 9,
  },
  description: REFLECTION_TOPIC_MAP.entries[8]?.description,
  keyPoints: REFLECTION_TOPIC_MAP.entries[8]?.keyPoints,
};

const state3: DialecticState = {
  id: 'refl-14',
  title: 'Identity in the copula grounds necessity judgment',
  concept: 'TransitionToNecessityJudgment',
  phase: 'subject',
  moments: [
    {
      name: 'copularIdentity',
      definition:
        'Subject and predicate coincide as one and same universal nature',
      type: 'determination',
    },
    {
      name: 'innerNecessaryConnection',
      definition:
        'Difference is retained as unessential within necessary connection',
      type: 'mediation',
      relation: 'transitions',
      relatedTo: 'copularIdentity',
    },
  ],
  invariants: [
    {
      id: 'refl-14-inv-1',
      constraint: 'subject and predicate are equal in copular identity',
      predicate: 'equals(subject, predicate)',
    },
    {
      id: 'refl-14-inv-2',
      constraint: 'nature in-and-for-itself acts as connective ground',
      predicate: 'grounds(natureInAndForItself, connection)',
    },
  ],
  forces: [
    {
      id: 'refl-14-force-1',
      description:
        'Copular identity passes over into objective universality of necessity',
      type: 'passover',
      trigger: 'innerNecessaryConnection.established = true',
      effect: 'necessityJudgment.emerges = true',
      targetState: 'nec-1',
    },
  ],
  transitions: [
    {
      id: 'refl-14-trans-1',
      from: 'refl-14',
      to: 'nec-1',
      mechanism: 'passover',
      description: 'From judgment of reflection to judgment of necessity',
    },
  ],
  nextStates: ['nec-1'],
  previousStates: ['refl-9'],
  provenance: {
    topicMapId: 'refl-14-transition-necessity',
    lineRange: { start: 403, end: 432 },
    section: 'The Universal Judgment',
    order: 14,
  },
  description: REFLECTION_TOPIC_MAP.entries[13]?.description,
  keyPoints: REFLECTION_TOPIC_MAP.entries[13]?.keyPoints,
};

export const reflectionIR: DialecticIR = {
  id: 'reflection-ir',
  title: 'Reflection Judgment IR: Subsumption, Allness, Necessity Transition',
  section: 'CONCEPT - SUBJECTIVITY - B. Judgment - B. Judgment of Reflection',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'reflection.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'refl-1': 'subject',
      'refl-9': 'subject',
      'refl-14': 'subject',
    },
  },
};

export const reflectionStates = {
  'refl-1': state1,
  'refl-9': state2,
  'refl-14': state3,
};
