import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { CONTINGENCY_TOPIC_MAP } from './sources/contingency-topic-map';

const state1: DialecticState = {
  id: 'act-a-1',
  title: 'Formal actuality immediately contains possibility',
  concept: 'FormalActuality',
  phase: 'appearance',
  moments: [
    {
      name: 'formalActuality',
      definition:
        'Immediate actuality not yet total form but carrying possibility within',
      type: 'determination',
    },
    {
      name: 'innerOuterUnity',
      definition: 'In-itselfness and externality remain immediately unified',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'formalActuality',
    },
  ],
  invariants: [
    {
      id: 'act-a-1-inv-1',
      constraint: 'what is actual is possible',
      predicate: 'implies(actual(x), possible(x))',
    },
    {
      id: 'act-a-1-inv-2',
      constraint: 'actuality remains immediate and unreflected',
      predicate: 'and(equals(actuality, immediate), not(reflected(actuality)))',
    },
  ],
  forces: [
    {
      id: 'act-a-1-force-1',
      description: 'Immediate unity destabilizes into contingent duality',
      type: 'reflection',
      trigger: 'possibility.distinguishedFromActuality = true',
      effect: 'contingency.twoSidedness = explicit',
      targetState: 'act-a-8',
    },
  ],
  transitions: [
    {
      id: 'act-a-1-trans-1',
      from: 'act-a-1',
      to: 'act-a-8',
      mechanism: 'reflection',
      description: 'From formal actuality into contingency structure',
    },
  ],
  nextStates: ['act-a-8'],
  previousStates: ['act-b-9', 'mod-9'],
  provenance: {
    topicMapId: 'act-a-1-formal-actuality',
    lineRange: { start: 3, end: 13 },
    section: 'Contingency or Formal Actuality, Possibility, and Necessity',
    order: 1,
  },
  description: CONTINGENCY_TOPIC_MAP.entries[0]?.description,
  keyPoints: CONTINGENCY_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'act-a-8',
  title: 'Contingency as grounded and groundless unity',
  concept: 'Contingency',
  phase: 'appearance',
  moments: [
    {
      name: 'groundlessImmediacy',
      definition:
        'Contingent actuality appears as immediate without sufficient ground',
      type: 'externality',
    },
    {
      name: 'groundedPositedness',
      definition:
        'The same contingent is reflected into an other as its ground',
      type: 'reflection',
      relation: 'opposite',
      relatedTo: 'groundlessImmediacy',
    },
  ],
  invariants: [
    {
      id: 'act-a-8-inv-1',
      constraint: 'contingent has no ground because contingent',
      predicate: 'implies(contingent(x), noGround(x))',
    },
    {
      id: 'act-a-8-inv-2',
      constraint: 'contingent has ground because contingent',
      predicate: 'implies(contingent(x), hasGround(x))',
    },
  ],
  forces: [
    {
      id: 'act-a-8-force-1',
      description: 'Absolute restlessness turns contingency into necessity',
      type: 'contradiction',
      trigger: 'eachDetermination.turnsIntoOpposite = true',
      effect: 'identityInOther.emerges = necessity',
      targetState: 'act-a-9',
    },
  ],
  transitions: [
    {
      id: 'act-a-8-trans-1',
      from: 'act-a-8',
      to: 'act-a-9',
      mechanism: 'contradiction',
      description: 'From contingency duality to necessity through conversion',
    },
  ],
  nextStates: ['act-a-9'],
  previousStates: ['act-a-1'],
  provenance: {
    topicMapId: 'act-a-8-contingency-two-sides',
    lineRange: { start: 151, end: 226 },
    section: 'Contingency or Formal Actuality, Possibility, and Necessity',
    order: 8,
  },
  description: CONTINGENCY_TOPIC_MAP.entries[7]?.description,
  keyPoints: CONTINGENCY_TOPIC_MAP.entries[7]?.keyPoints,
};

const state3: DialecticState = {
  id: 'act-a-9',
  title: 'Absolute restlessness yields necessity',
  concept: 'NecessityFromContingency',
  phase: 'appearance',
  moments: [
    {
      name: 'absoluteRestlessness',
      definition: 'Immediate conversion of inner and outer in becoming',
      type: 'process',
    },
    {
      name: 'identityOfOpposites',
      definition:
        'Each determination rejoins itself in its opposite as necessity',
      type: 'sublation',
      relation: 'unified',
      relatedTo: 'absoluteRestlessness',
    },
  ],
  invariants: [
    {
      id: 'act-a-9-inv-1',
      constraint: 'becoming is contingency',
      predicate: 'equals(becoming, contingency)',
    },
    {
      id: 'act-a-9-inv-2',
      constraint: 'identity in opposition is necessity',
      predicate: 'equals(identity(opposites), necessity)',
    },
  ],
  forces: [
    {
      id: 'act-a-9-force-1',
      description: 'Necessity moves to determinate necessity',
      type: 'passover',
      trigger: 'necessity.identityEstablished = true',
      effect: 'determinateNecessity.emerges = true',
      targetState: 'act-c-1',
    },
  ],
  transitions: [
    {
      id: 'act-a-9-trans-1',
      from: 'act-a-9',
      to: 'act-c-1',
      mechanism: 'passover',
      description: 'From contingency necessity to absolute necessity',
    },
  ],
  nextStates: ['act-c-1'],
  previousStates: ['act-a-8'],
  provenance: {
    topicMapId: 'act-a-9-absolute-restlessness-necessity',
    lineRange: { start: 184, end: 216 },
    section: 'Contingency or Formal Actuality, Possibility, and Necessity',
    order: 9,
  },
  description: CONTINGENCY_TOPIC_MAP.entries[8]?.description,
  keyPoints: CONTINGENCY_TOPIC_MAP.entries[8]?.keyPoints,
};

export const contingencyIR: DialecticIR = {
  id: 'contingency-ir',
  title:
    'Contingency IR: Formal Actuality, Groundless/Grounded Unity, Necessity',
  section: 'ESSENCE - C. ACTUALITY - A. Contingency',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'contingency.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'act-a-1': 'appearance',
      'act-a-8': 'appearance',
      'act-a-9': 'appearance',
    },
  },
};

export const contingencyStates = {
  'act-a-1': state1,
  'act-a-8': state2,
  'act-a-9': state3,
};
