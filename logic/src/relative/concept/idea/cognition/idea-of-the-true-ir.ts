import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { IDEA_OF_THE_TRUE_TOPIC_MAP } from './sources/idea-of-the-true-topic-map';

const state1: DialecticState = {
  id: 'idea-true-1',
  title: 'Impulse of truth and finitude of theoretical cognition',
  concept: 'TheoreticalTruthImpulse',
  phase: 'subject',
  moments: [
    {
      name: 'truthAsComparativeRelation',
      definition:
        'Theoretical cognition seeks truth by comparing concept and actuality through judgmental relation',
      type: 'mediation',
    },
    {
      name: 'finiteSynthesis',
      definition:
        'Its synthesis remains finite because objective unity is only posited for subjectivity',
      type: 'contradiction',
      relation: 'contains',
      relatedTo: 'truthAsComparativeRelation',
    },
  ],
  invariants: [
    {
      id: 'idea-true-1-inv-1',
      constraint:
        'cognition begins from certainty of concept over against a presupposed world',
      predicate: 'presupposedWorldGiven(cognition)',
    },
    {
      id: 'idea-true-1-inv-2',
      constraint: 'finitude persists while unity is external conjunction',
      predicate: 'externalConjunctionImpliesFinitude(cognition)',
    },
  ],
  forces: [
    {
      id: 'idea-true-1-force-1',
      description:
        'Finite cognition differentiates into analytic and synthetic procedures',
      type: 'reflection',
      trigger: 'truthImpulse.requiresMethodicDetermination = true',
      effect: 'methodicSplit.appears = true',
      targetState: 'idea-true-4',
    },
  ],
  transitions: [
    {
      id: 'idea-true-1-trans-1',
      from: 'idea-true-1',
      to: 'idea-true-4',
      mechanism: 'reflection',
      description:
        'From initial truth impulse to synthetic determination of cognition',
    },
  ],
  nextStates: ['idea-true-4'],
  previousStates: ['cognition-6'],
  provenance: {
    topicMapId: 'idea-true-1-impulse-finite',
    lineRange: { start: 1, end: 656 },
    section: 'The Idea of the True',
    order: 1,
  },
  description: IDEA_OF_THE_TRUE_TOPIC_MAP.entries[0]?.description,
  keyPoints: IDEA_OF_THE_TRUE_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'idea-true-4',
  title: 'Synthetic cognition through definition and division',
  concept: 'SyntheticCognitionStructure',
  phase: 'subject',
  moments: [
    {
      name: 'definitionAsConceptualReduction',
      definition:
        'Definition translates given objectivity into conceptual form but retains contingency in selected marks',
      type: 'determination',
    },
    {
      name: 'divisionAsParticularization',
      definition:
        'Division advances universal into particular systematization, yet without full immanent principle',
      type: 'process',
      relation: 'transitions',
      relatedTo: 'definitionAsConceptualReduction',
    },
  ],
  invariants: [
    {
      id: 'idea-true-4-inv-1',
      constraint:
        'synthetic cognition seeks necessity but keeps singular content as given',
      predicate: 'givenSingularityPersists(syntheticCognition)',
    },
    {
      id: 'idea-true-4-inv-2',
      constraint:
        'systematic order remains partly formal where immanent grounds are absent',
      predicate: 'formalOrderWithoutImmanentGround(divisionalMethod)',
    },
  ],
  forces: [
    {
      id: 'idea-true-4-force-1',
      description:
        'Synthetic development culminates in theorem, proof, and construction',
      type: 'mediation',
      trigger: 'definitionAndDivision.requireConcreteNecessity = true',
      effect: 'theoremStage.emerges = true',
      targetState: 'idea-true-6',
    },
  ],
  transitions: [
    {
      id: 'idea-true-4-trans-1',
      from: 'idea-true-4',
      to: 'idea-true-6',
      mechanism: 'mediation',
      description:
        'From definition/division to theorem and proof as concrete synthetic moment',
    },
  ],
  nextStates: ['idea-true-6'],
  previousStates: ['idea-true-1'],
  provenance: {
    topicMapId: 'idea-true-4-synthetic-definition',
    lineRange: { start: 1208, end: 1648 },
    section: 'The Idea of the True',
    order: 2,
  },
  description: IDEA_OF_THE_TRUE_TOPIC_MAP.entries[3]?.description,
  keyPoints: IDEA_OF_THE_TRUE_TOPIC_MAP.entries[3]?.keyPoints,
};

const state3: DialecticState = {
  id: 'idea-true-6',
  title: 'Theorem and proof transition from necessity to practical idea',
  concept: 'TransitionToPracticalIdea',
  phase: 'subject',
  moments: [
    {
      name: 'proofMediation',
      definition:
        'Theorem requires proof and construction, displaying necessary relation of real determinations',
      type: 'mediation',
    },
    {
      name: 'necessityToFreedomPassage',
      definition:
        'At its limit, synthetic necessity passes into freedom of concept as practical idea',
      type: 'passover',
      relation: 'passesOver',
      relatedTo: 'proofMediation',
    },
  ],
  invariants: [
    {
      id: 'idea-true-6-inv-1',
      constraint:
        'cognition does not yet attain full unity of concept and subject matter',
      predicate: 'notYetAttained(fullTruthUnity)',
    },
    {
      id: 'idea-true-6-inv-2',
      constraint:
        'transition to practice is grounded in limit of finite theoretical method',
      predicate: 'groundedTransition(theoreticalLimit, practicalIdea)',
    },
  ],
  forces: [
    {
      id: 'idea-true-6-force-1',
      description:
        'Finite truth-method is sublated into practical self-determining good',
      type: 'passover',
      trigger: 'necessity.reachesFreedomThreshold = true',
      effect: 'ideaOfGood.emerges = true',
      targetState: 'idea-good-1',
    },
  ],
  transitions: [
    {
      id: 'idea-true-6-trans-1',
      from: 'idea-true-6',
      to: 'idea-good-1',
      mechanism: 'passover',
      description: 'From idea of the true to the practical idea of the good',
    },
  ],
  nextStates: ['idea-good-1'],
  previousStates: ['idea-true-4'],
  provenance: {
    topicMapId: 'idea-true-6-synthetic-theorem',
    lineRange: { start: 2014, end: 2727 },
    section: 'The Idea of the True',
    order: 3,
  },
  description: IDEA_OF_THE_TRUE_TOPIC_MAP.entries[5]?.description,
  keyPoints: IDEA_OF_THE_TRUE_TOPIC_MAP.entries[5]?.keyPoints,
};

export const ideaOfTheTrueIR: DialecticIR = {
  id: 'idea-of-the-true-ir',
  title: 'Idea of the True IR: Impulse, Synthetic Method, Practical Transition',
  section: 'CONCEPT - IDEA - B. Cognition - 2. The Idea of the True',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'idea-of-the-true.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'idea-true-1': 'subject',
      'idea-true-4': 'subject',
      'idea-true-6': 'subject',
    },
  },
};

export const ideaOfTheTrueStates = {
  'idea-true-1': state1,
  'idea-true-4': state2,
  'idea-true-6': state3,
};
