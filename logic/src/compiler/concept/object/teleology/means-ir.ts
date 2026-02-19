import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { meansTopicMap } from './souces/means-topic-map';

const state1: DialecticState = {
  id: 'means-1',
  title: 'Means as formal middle term',
  concept: 'MeansAsMiddleTerm',
  phase: 'object',
  moments: [
    {
      name: 'formalMiddleTerm',
      definition:
        'Means mediates purpose and objectivity as a first external linkage',
      type: 'mediation',
    },
    {
      name: 'finitudeExposed',
      definition:
        'Need for means exposes purpose as finite against indifferent externality',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'formalMiddleTerm',
    },
  ],
  invariants: [
    {
      id: 'means-1-inv-1',
      constraint: 'means is middle term of purposive syllogism',
      predicate: 'equals(means, middleTerm(purposiveSyllogism))',
    },
    {
      id: 'means-1-inv-2',
      constraint: 'means remains first as external objectivity',
      predicate: 'initiallyExternal(means)',
    },
  ],
  forces: [
    {
      id: 'means-1-force-1',
      description: 'Formal mediation develops into mechanical means',
      type: 'mediation',
      trigger: 'middleTerm.externalLinkage = explicit',
      effect: 'mechanicalMeans.emerges = true',
      targetState: 'means-4',
    },
  ],
  transitions: [
    {
      id: 'means-1-trans-1',
      from: 'means-1',
      to: 'means-4',
      mechanism: 'mediation',
      description: 'From formal middle term to mechanical means',
    },
  ],
  nextStates: ['means-4'],
  previousStates: ['tele-6'],
  provenance: {
    topicMapId: 'means-2',
    lineRange: { start: 27, end: 78 },
    section: 'B. THE MEANS',
    order: 1,
  },
  description: meansTopicMap[1]?.description,
  keyPoints: meansTopicMap[1]?.keyPoints,
};

const state2: DialecticState = {
  id: 'means-4',
  title: 'Means as mechanical object and concept-totality',
  concept: 'MechanicalMeans',
  phase: 'object',
  moments: [
    {
      name: 'mechanicalObjectMedius',
      definition:
        'Means appears as mechanical object where concept and objectivity are externally linked',
      type: 'determination',
    },
    {
      name: 'penetrableTotality',
      definition:
        'As implicit concept-totality, the means is penetrable and serviceable to purpose',
      type: 'reflection',
      relation: 'transforms',
      relatedTo: 'mechanicalObjectMedius',
    },
  ],
  invariants: [
    {
      id: 'means-4-inv-1',
      constraint: 'means is objective yet conceptually pervious',
      predicate: 'and(objective(means), perviousToConcept(means))',
    },
    {
      id: 'means-4-inv-2',
      constraint: 'object side is rendered powerless before purpose',
      predicate: 'renderedPowerless(objectSide, purpose)',
    },
  ],
  forces: [
    {
      id: 'means-4-force-1',
      description:
        'Means-totality drives purposive activity against presupposition',
      type: 'reflection',
      trigger: 'purpose.activityThroughMeans = explicit',
      effect: 'meansActivity.emerges = true',
      targetState: 'means-6',
    },
  ],
  transitions: [
    {
      id: 'means-4-trans-1',
      from: 'means-4',
      to: 'means-6',
      mechanism: 'reflection',
      description: 'From mechanical means to means-activity',
    },
  ],
  nextStates: ['means-6'],
  previousStates: ['means-1'],
  provenance: {
    topicMapId: 'means-5',
    lineRange: { start: 125, end: 162 },
    section: 'B. THE MEANS',
    order: 2,
  },
  description: meansTopicMap[4]?.description,
  keyPoints: meansTopicMap[4]?.keyPoints,
};

const state3: DialecticState = {
  id: 'means-6',
  title: 'Activity through means remains against presupposition',
  concept: 'TransitionToRealizedPurpose',
  phase: 'object',
  moments: [
    {
      name: 'activityAgainstPresupposition',
      definition:
        'Purpose, now active, still confronts presupposed external objectivity',
      type: 'process',
    },
    {
      name: 'realizationBridge',
      definition:
        'The means-activity becomes the immediate bridge into realized purpose',
      type: 'passover',
      relation: 'transitions',
      relatedTo: 'activityAgainstPresupposition',
    },
  ],
  invariants: [
    {
      id: 'means-6-inv-1',
      constraint:
        'objectivity is within concept yet still external as presupposed',
      predicate:
        'and(withinConcept(objectivity), stillPresupposedExternal(objectivity))',
    },
    {
      id: 'means-6-inv-2',
      constraint: 'purpose is no longer mere impulse but explicit activity',
      predicate: 'explicitActivity(purpose)',
    },
  ],
  forces: [
    {
      id: 'means-6-force-1',
      description: 'Means mediation passes over into realized purpose process',
      type: 'passover',
      trigger: 'means.activityComplete = true',
      effect: 'realizedPurpose.emerges = true',
      targetState: 'realized-2',
    },
  ],
  transitions: [
    {
      id: 'means-6-trans-1',
      from: 'means-6',
      to: 'realized-2',
      mechanism: 'passover',
      description: 'From means to realized purpose',
    },
  ],
  nextStates: ['realized-2'],
  previousStates: ['means-4'],
  provenance: {
    topicMapId: 'means-6',
    lineRange: { start: 163, end: 181 },
    section: 'B. THE MEANS',
    order: 3,
  },
  description: meansTopicMap[5]?.description,
  keyPoints: meansTopicMap[5]?.keyPoints,
};

export const meansIR: DialecticIR = {
  id: 'means-ir',
  title: 'Means IR: Formal Middle, Mechanical Means, Realization Bridge',
  section: 'CONCEPT - OBJECTIVITY - C. Teleology - B. The Means',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'means.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'means-1': 'object',
      'means-4': 'object',
      'means-6': 'object',
    },
  },
};

export const meansStates = {
  'means-1': state1,
  'means-4': state2,
  'means-6': state3,
};
