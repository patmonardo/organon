import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { COGNITION_TOPIC_MAP } from './sources/cognition-topic-map';

const state1: DialecticState = {
  id: 'cognition-1',
  title: 'Cognition as elevation above life',
  concept: 'IdeaOfCognitionInGeneral',
  phase: 'subject',
  moments: [
    {
      name: 'doublingOfIdea',
      definition:
        'The idea differentiates into subjective concept and objective life while remaining a self-related unity',
      type: 'mediation',
    },
    {
      name: 'conceptAsSubjectMatter',
      definition:
        'Concept confronts itself as object and thereby grounds thought, spirit, and self-consciousness',
      type: 'reflection',
      relation: 'contains',
      relatedTo: 'doublingOfIdea',
    },
  ],
  invariants: [
    {
      id: 'cognition-1-inv-1',
      constraint:
        'cognition preserves life while elevating it into conceptual universality',
      predicate: 'elevatesWithoutAbolishing(cognition, life)',
    },
    {
      id: 'cognition-1-inv-2',
      constraint: 'subject and object are both moments of one idea',
      predicate: 'unityOfMoments(subjectiveConcept, objectiveLife)',
    },
  ],
  forces: [
    {
      id: 'cognition-1-force-1',
      description:
        'Self-reference of concept exposes finitude and drives critical differentiation',
      type: 'negation',
      trigger: 'concept.selfReference = explicit',
      effect: 'criticalMediation.emerges = true',
      targetState: 'cognition-4',
    },
  ],
  transitions: [
    {
      id: 'cognition-1-trans-1',
      from: 'cognition-1',
      to: 'cognition-4',
      mechanism: 'negation',
      description:
        'From immediate cognition to reflective critique and purposive truth-seeking',
    },
  ],
  nextStates: ['cognition-4'],
  previousStates: ['genus-3'],
  provenance: {
    topicMapId: 'cognition-1-life-cognition',
    lineRange: { start: 6, end: 41 },
    section: 'The Idea of Cognition (Introduction/Genera)',
    order: 1,
  },
  description: COGNITION_TOPIC_MAP.entries[0]?.description,
  keyPoints: COGNITION_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'cognition-4',
  title: 'Idea as purpose seeking truth',
  concept: 'TheoreticalIdea',
  phase: 'subject',
  moments: [
    {
      name: 'truthImpulse',
      definition:
        'Cognition seeks identity of concept and actuality, but initially as a subjective purpose',
      type: 'determination',
    },
    {
      name: 'syllogisticMediation',
      definition:
        'Concept and objective world stand as extremes whose unity must be posited through cognition',
      type: 'mediation',
      relation: 'transitions',
      relatedTo: 'truthImpulse',
    },
  ],
  invariants: [
    {
      id: 'cognition-4-inv-1',
      constraint:
        'theoretical cognition remains finite while objective content is still given',
      predicate: 'finiteWhileGiven(theoreticalCognition)',
    },
    {
      id: 'cognition-4-inv-2',
      constraint:
        'concept remains active principle in transforming given objectivity',
      predicate: 'activePrinciple(concept)',
    },
  ],
  forces: [
    {
      id: 'cognition-4-force-1',
      description:
        'Finite theoretical striving compels deepened transition through spirit-threshold mediation',
      type: 'passover',
      trigger: 'subjectiveTruthSeeking.reachesLimit = true',
      effect: 'spiritThreshold.explicit = true',
      targetState: 'cognition-6',
    },
  ],
  transitions: [
    {
      id: 'cognition-4-trans-1',
      from: 'cognition-4',
      to: 'cognition-6',
      mechanism: 'passover',
      description:
        'From theoretical truth-seeking to spirit-threshold articulation',
    },
  ],
  nextStates: ['cognition-6'],
  previousStates: ['cognition-1'],
  provenance: {
    topicMapId: 'cognition-4-purpose-truth',
    lineRange: { start: 462, end: 531 },
    section: 'The Idea of Cognition (Introduction/Genera)',
    order: 2,
  },
  description: COGNITION_TOPIC_MAP.entries[3]?.description,
  keyPoints: COGNITION_TOPIC_MAP.entries[3]?.keyPoints,
};

const state3: DialecticState = {
  id: 'cognition-6',
  title: 'Life-to-spirit transition as cognition horizon',
  concept: 'CognitiveSpiritThreshold',
  phase: 'subject',
  moments: [
    {
      name: 'truthOfLifeAsSpirit',
      definition:
        'The truth of life is spirit, where universality is explicit in a free conceptual subject',
      type: 'passover',
    },
    {
      name: 'incompleteConsummation',
      definition:
        'This horizon remains not yet consummated and therefore points beyond itself to determinate cognition-processes',
      type: 'contradiction',
      relation: 'contains',
      relatedTo: 'truthOfLifeAsSpirit',
    },
  ],
  invariants: [
    {
      id: 'cognition-6-inv-1',
      constraint:
        'spirit emerges as truth of life only through sublation of abstract singularity',
      predicate: 'emergesThroughSublation(spirit, abstractSingularity)',
    },
    {
      id: 'cognition-6-inv-2',
      constraint:
        'concept now confronts itself explicitly as its own subject matter',
      predicate: 'selfConfrontation(concept)',
    },
  ],
  forces: [
    {
      id: 'cognition-6-force-1',
      description:
        'The not-yet-consummated cognitive horizon pushes toward the full articulation of truth',
      type: 'mediation',
      trigger: 'consummation.notYet = true',
      effect: 'ideaOfTrue.develops = true',
      targetState: 'idea-true-1',
    },
  ],
  transitions: [
    {
      id: 'cognition-6-trans-1',
      from: 'cognition-6',
      to: 'idea-true-1',
      mechanism: 'mediation',
      description: 'From spirit threshold into articulated idea of the true',
    },
  ],
  nextStates: ['idea-true-1'],
  previousStates: ['cognition-4'],
  provenance: {
    topicMapId: 'cognition-3-life-spirit',
    lineRange: { start: 318, end: 460 },
    section: 'The Idea of Cognition (Introduction/Genera)',
    order: 3,
  },
  description: COGNITION_TOPIC_MAP.entries[2]?.description,
  keyPoints: COGNITION_TOPIC_MAP.entries[2]?.keyPoints,
};

export const cognitionIR: DialecticIR = {
  id: 'cognition-ir',
  title: 'Cognition IR: General Cognition, Theoretical Truth, Spirit Threshold',
  section: 'CONCEPT - IDEA - B. Cognition - 1. Introduction',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'cognition.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'cognition-1': 'subject',
      'cognition-4': 'subject',
      'cognition-6': 'subject',
    },
  },
};

export const cognitionStates = {
  'cognition-1': state1,
  'cognition-4': state2,
  'cognition-6': state3,
};
