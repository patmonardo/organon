import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { IDEA_OF_THE_GOOD_TOPIC_MAP } from './sources/idea-of-the-good-topic-map';

const state1: DialecticState = {
  id: 'idea-good-1',
  title: 'Practical idea as self-determining good',
  concept: 'PracticalIdeaOfGood',
  phase: 'subject',
  moments: [
    {
      name: 'goodAsObjectiveDemand',
      definition:
        'The good is conceptually valid in-and-for-itself and demands external actuality through will',
      type: 'determination',
    },
    {
      name: 'practicalSelfPositing',
      definition:
        'Practical idea posits its own determination and sublates given-world determinacies as null',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'goodAsObjectiveDemand',
    },
  ],
  invariants: [
    {
      id: 'idea-good-1-inv-1',
      constraint:
        'practical certainty treats objective world as non-actual over against concept',
      predicate: 'practicalCertaintyNegatesGivenWorld(practicalIdea)',
    },
    {
      id: 'idea-good-1-inv-2',
      constraint:
        'content is finite in determinateness but infinite in conceptual form',
      predicate: 'finiteContentInfiniteForm(good)',
    },
  ],
  forces: [
    {
      id: 'idea-good-1-force-1',
      description:
        'Realization exposes finitude and the persistence of the ought',
      type: 'contradiction',
      trigger: 'good.realization = externallyContingent',
      effect: 'oughtStructure.explicit = true',
      targetState: 'idea-good-2',
    },
  ],
  transitions: [
    {
      id: 'idea-good-1-trans-1',
      from: 'idea-good-1',
      to: 'idea-good-2',
      mechanism: 'contradiction',
      description:
        'From practical good to contradiction of realization and finitude',
    },
  ],
  nextStates: ['idea-good-2'],
  previousStates: ['idea-true-6'],
  provenance: {
    topicMapId: 'idea-good-1-practical-good',
    lineRange: { start: 1, end: 95 },
    section: 'The Idea of the Good',
    order: 1,
  },
  description: IDEA_OF_THE_GOOD_TOPIC_MAP.entries[0]?.description,
  keyPoints: IDEA_OF_THE_GOOD_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'idea-good-2',
  title: 'The ought and finitude of practical realization',
  concept: 'OughtAndPracticalFinitude',
  phase: 'subject',
  moments: [
    {
      name: 'externalContingency',
      definition:
        'Realized good remains exposed to contingency, conflict, and destruction in external actuality',
      type: 'externality',
    },
    {
      name: 'twoWorldContradiction',
      definition:
        'Subjective transparency and objective manifold stand opposed as unresolved practical contradiction',
      type: 'contradiction',
      relation: 'contains',
      relatedTo: 'externalContingency',
    },
  ],
  invariants: [
    {
      id: 'idea-good-2-inv-1',
      constraint:
        'good appears as ought while abstract being stands as its negation',
      predicate: 'oughtStructure(good)',
    },
    {
      id: 'idea-good-2-inv-2',
      constraint:
        'practical idea requires completion in unity with theoretical truth',
      predicate: 'requiresCompletionInTrue(practicalIdea)',
    },
  ],
  forces: [
    {
      id: 'idea-good-2-force-1',
      description:
        'Contradiction of the ought drives self-sublation of presupposed externality',
      type: 'sublation',
      trigger: 'ought.contradiction = explicit',
      effect: 'presuppositionSublated = true',
      targetState: 'idea-good-3',
    },
  ],
  transitions: [
    {
      id: 'idea-good-2-trans-1',
      from: 'idea-good-2',
      to: 'idea-good-3',
      mechanism: 'sublation',
      description:
        'From ought contradiction to transition beyond practical finitude',
    },
  ],
  nextStates: ['idea-good-3'],
  previousStates: ['idea-good-1'],
  provenance: {
    topicMapId: 'idea-good-2-realization-finitude',
    lineRange: { start: 96, end: 195 },
    section: 'The Idea of the Good',
    order: 2,
  },
  description: IDEA_OF_THE_GOOD_TOPIC_MAP.entries[1]?.description,
  keyPoints: IDEA_OF_THE_GOOD_TOPIC_MAP.entries[1]?.keyPoints,
};

const state3: DialecticState = {
  id: 'idea-good-3',
  title: 'Sublation of presupposition and emergence of absolute idea',
  concept: 'TransitionToAbsoluteIdea',
  phase: 'subject',
  moments: [
    {
      name: 'selfSublatingMediation',
      definition:
        'Mediation sublates itself so immediacy is preserved only as sublated presupposition',
      type: 'sublation',
    },
    {
      name: 'unityOfTheoreticalAndPractical',
      definition:
        'Cognition is restored and unified with practical idea in objective concept truly existing',
      type: 'reflection',
      relation: 'passesOver',
      relatedTo: 'selfSublatingMediation',
    },
  ],
  invariants: [
    {
      id: 'idea-good-3-inv-1',
      constraint:
        'subjective singularity vanishes with the sublation of presupposition',
      predicate: 'vanishesWithSublatedPresupposition(subjectiveSingularity)',
    },
    {
      id: 'idea-good-3-inv-2',
      constraint:
        'objective world now has concept as inner ground and subsistence',
      predicate: 'conceptGroundsObjectivity(world)',
    },
  ],
  forces: [
    {
      id: 'idea-good-3-force-1',
      description:
        'The completed unity of practical and theoretical idea passes into absolute idea',
      type: 'passover',
      trigger: 'unityOfTrueAndGood = complete',
      effect: 'absoluteIdea.emerges = true',
      targetState: 'absolute-idea-1',
    },
  ],
  transitions: [
    {
      id: 'idea-good-3-trans-1',
      from: 'idea-good-3',
      to: 'absolute-idea-1',
      mechanism: 'passover',
      description: 'From idea of the good to the absolute idea',
    },
  ],
  nextStates: ['absolute-idea-1'],
  previousStates: ['idea-good-2'],
  provenance: {
    topicMapId: 'idea-good-3-absolute-idea',
    lineRange: { start: 196, end: 363 },
    section: 'The Idea of the Good',
    order: 3,
  },
  description: IDEA_OF_THE_GOOD_TOPIC_MAP.entries[2]?.description,
  keyPoints: IDEA_OF_THE_GOOD_TOPIC_MAP.entries[2]?.keyPoints,
};

export const ideaOfTheGoodIR: DialecticIR = {
  id: 'idea-of-the-good-ir',
  title: 'Idea of the Good IR: Practical Good, Ought, Absolute Transition',
  section: 'CONCEPT - IDEA - B. Cognition - 3. The Idea of the Good',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'idea-of-the-good.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'idea-good-1': 'subject',
      'idea-good-2': 'subject',
      'idea-good-3': 'subject',
    },
  },
};

export const ideaOfTheGoodStates = {
  'idea-good-1': state1,
  'idea-good-2': state2,
  'idea-good-3': state3,
};
