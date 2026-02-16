import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { subjectiveTopicMap } from './souces/subjective-topic-map';

const state1: DialecticState = {
  id: 'tele-1',
  title: 'Purpose rediscovered as objective-free concept',
  concept: 'SubjectivePurpose',
  phase: 'object',
  moments: [
    {
      name: 'purposeRediscovered',
      definition:
        'In centrality and chemism the concept rediscovers itself as purposive unity',
      type: 'determination',
    },
    {
      name: 'rationalConcreteUnity',
      definition:
        'Purpose is concrete rationality holding objective difference in absolute unity',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'purposeRediscovered',
    },
  ],
  invariants: [
    {
      id: 'tele-1-inv-1',
      constraint: 'purpose is exempt from mere transition process',
      predicate: 'exemptFromMereTransition(purpose)',
    },
    {
      id: 'tele-1-inv-2',
      constraint: 'purpose is rational in concrete existence',
      predicate: 'equals(purpose, rationalInConcreteExistence)',
    },
  ],
  forces: [
    {
      id: 'tele-1-force-1',
      description: 'Purpose unfolds its inner syllogistic articulation',
      type: 'mediation',
      trigger: 'purpose.selfRepellingUnity = explicit',
      effect: 'innerSyllogism.emerges = true',
      targetState: 'tele-4',
    },
  ],
  transitions: [
    {
      id: 'tele-1-trans-1',
      from: 'tele-1',
      to: 'tele-4',
      mechanism: 'mediation',
      description: 'From rediscovered purpose to its inner syllogism',
    },
  ],
  nextStates: ['tele-4'],
  previousStates: ['chem-6'],
  provenance: {
    topicMapId: 'subjective-1',
    lineRange: { start: 2, end: 50 },
    section: 'A. THE SUBJECTIVE PURPOSE',
    order: 1,
  },
  description: subjectiveTopicMap[0]?.description,
  keyPoints: subjectiveTopicMap[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'tele-4',
  title: 'Purpose as finite syllogism and presupposition',
  concept: 'FiniteSubjectivePurpose',
  phase: 'object',
  moments: [
    {
      name: 'innerSyllogism',
      definition:
        'Purpose is internally syllogistic as self-equal universality and singularizing negativity',
      type: 'determination',
    },
    {
      name: 'finitudeAsPresupposition',
      definition:
        'Purpose stands over against presupposed mechanical/chemical objectivity',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'innerSyllogism',
    },
  ],
  invariants: [
    {
      id: 'tele-4-inv-1',
      constraint: 'purpose is finite by objective presupposition',
      predicate: 'finiteByObjectivePresupposition(purpose)',
    },
    {
      id: 'tele-4-inv-2',
      constraint: 'self-determining activity is immediately external to itself',
      predicate: 'immediatelyExternal(selfDeterminingActivity)',
    },
  ],
  forces: [
    {
      id: 'tele-4-force-1',
      description: 'Finite purpose passes into realization movement',
      type: 'reflection',
      trigger: 'presupposition.mustBeSublated = true',
      effect: 'realizationMovement.emerges = true',
      targetState: 'tele-6',
    },
  ],
  transitions: [
    {
      id: 'tele-4-trans-1',
      from: 'tele-4',
      to: 'tele-6',
      mechanism: 'reflection',
      description: 'From finite purposiveness to realization movement',
    },
  ],
  nextStates: ['tele-6'],
  previousStates: ['tele-1'],
  provenance: {
    topicMapId: 'subjective-3',
    lineRange: { start: 75, end: 119 },
    section: 'A. THE SUBJECTIVE PURPOSE',
    order: 2,
  },
  description: subjectiveTopicMap[2]?.description,
  keyPoints: subjectiveTopicMap[2]?.keyPoints,
};

const state3: DialecticState = {
  id: 'tele-6',
  title: 'Realization movement posits means',
  concept: 'TransitionToMeans',
  phase: 'object',
  moments: [
    {
      name: 'sublatingPresupposition',
      definition:
        'Purpose negates immediate objectivity and posits it as concept-determined',
      type: 'sublation',
    },
    {
      name: 'meansPositing',
      definition:
        'Self-determination is at once presupposing and therefore first posits means',
      type: 'passover',
      relation: 'transitions',
      relatedTo: 'sublatingPresupposition',
    },
  ],
  invariants: [
    {
      id: 'tele-6-inv-1',
      constraint: 'realization is unification of objective being with purpose',
      predicate: 'equals(realization, unification(objectiveBeing, purpose))',
    },
    {
      id: 'tele-6-inv-2',
      constraint: 'self-positing of purpose is simultaneously presupposing',
      predicate: 'simultaneous(selfPositing(purpose), presupposing(purpose))',
    },
  ],
  forces: [
    {
      id: 'tele-6-force-1',
      description: 'Realization of purpose opens the means stage',
      type: 'passover',
      trigger: 'purpose.activity = explicit',
      effect: 'means.emerges = true',
      targetState: 'means-1',
    },
  ],
  transitions: [
    {
      id: 'tele-6-trans-1',
      from: 'tele-6',
      to: 'means-1',
      mechanism: 'passover',
      description: 'From subjective purpose to means',
    },
  ],
  nextStates: ['means-1'],
  previousStates: ['tele-4'],
  provenance: {
    topicMapId: 'subjective-4',
    lineRange: { start: 121, end: 164 },
    section: 'A. THE SUBJECTIVE PURPOSE',
    order: 3,
  },
  description: subjectiveTopicMap[3]?.description,
  keyPoints: subjectiveTopicMap[3]?.keyPoints,
};

export const teleologyIR: DialecticIR = {
  id: 'teleology-ir',
  title: 'Teleology IR: Rediscovered Purpose, Finitude, Means Bridge',
  section: 'CONCEPT - OBJECTIVITY - C. Teleology - A. Subjective Purpose',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'subjective.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'tele-1': 'object',
      'tele-4': 'object',
      'tele-6': 'object',
    },
  },
};

export const teleologyStates = {
  'tele-1': state1,
  'tele-4': state2,
  'tele-6': state3,
};
