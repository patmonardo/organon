import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { RELATIVE_NECESSITY_TOPIC_MAP } from './sources/relative-necessity-topic-map';

const state1: DialecticState = {
  id: 'act-b-1',
  title: 'Real actuality as formal necessity with content',
  concept: 'RealActuality',
  phase: 'appearance',
  moments: [
    {
      name: 'formalNecessity',
      definition:
        'Necessity appears as immediate unity still formal in its moments',
      type: 'determination',
    },
    {
      name: 'realActuality',
      definition:
        'Actuality as indifferent unity of form determinations with manifold content',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'formalNecessity',
    },
  ],
  invariants: [
    {
      id: 'act-b-1-inv-1',
      constraint: 'realActuality contains possibility as in-itself',
      predicate: 'contains(realActuality, possibilityInItself)',
    },
    {
      id: 'act-b-1-inv-2',
      constraint: 'unity of moments remains immediate',
      predicate: 'equals(unity(moments), immediate)',
    },
  ],
  forces: [
    {
      id: 'act-b-1-force-1',
      description:
        'Content-rich actuality drives toward completion of conditions',
      type: 'immanence',
      trigger: 'realActuality.content = manifoldCircumstances',
      effect: 'realPossibility.totalityOfConditions = sought',
      targetState: 'act-b-6',
    },
  ],
  transitions: [
    {
      id: 'act-b-1-trans-1',
      from: 'act-b-1',
      to: 'act-b-6',
      mechanism: 'reflection',
      description:
        'From immediate real actuality to contradiction and completion of conditions',
    },
  ],
  nextStates: ['act-b-6'],
  previousStates: ['mod-9'],
  provenance: {
    topicMapId: 'act-b-1-formal-necessity-real-actuality',
    lineRange: { start: 3, end: 18 },
    section: 'Relative Necessity or Real Actuality, Possibility, and Necessity',
    order: 1,
  },
  description: RELATIVE_NECESSITY_TOPIC_MAP.entries[0]?.description,
  keyPoints: RELATIVE_NECESSITY_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'act-b-6',
  title: 'Real possibility contradiction and complete conditions',
  concept: 'RealPossibilityCompletion',
  phase: 'appearance',
  moments: [
    {
      name: 'realPossibility',
      definition: 'Possibility as in-itself full of determinate conditions',
      type: 'determination',
    },
    {
      name: 'conditionTotality',
      definition:
        'Completeness where all conditions are present and fact is there',
      type: 'contradiction',
      relation: 'transforms',
      relatedTo: 'realPossibility',
    },
  ],
  invariants: [
    {
      id: 'act-b-6-inv-1',
      constraint:
        'real possibility includes manifold connection and opposition',
      predicate: 'contains(realPossibility, manifoldOpposition)',
    },
    {
      id: 'act-b-6-inv-2',
      constraint: 'complete conditions imply fact actual presence',
      predicate: 'implies(allConditionsPresent, factActuallyThere)',
    },
  ],
  forces: [
    {
      id: 'act-b-6-force-1',
      description: 'Self-sublating conditions recoil into necessity',
      type: 'sublation',
      trigger: 'conditionTotality.complete = true',
      effect: 'realNecessity.emerges = true',
      targetState: 'act-b-9',
    },
  ],
  transitions: [
    {
      id: 'act-b-6-trans-1',
      from: 'act-b-6',
      to: 'act-b-9',
      mechanism: 'reflection',
      description: 'From completed conditions to relative necessity',
    },
  ],
  nextStates: ['act-b-9'],
  previousStates: ['act-b-1'],
  provenance: {
    topicMapId: 'act-b-6-real-possibility-contradiction',
    lineRange: { start: 88, end: 159 },
    section: 'Relative Necessity or Real Actuality, Possibility, and Necessity',
    order: 6,
  },
  description: RELATIVE_NECESSITY_TOPIC_MAP.entries[5]?.description,
  keyPoints: RELATIVE_NECESSITY_TOPIC_MAP.entries[5]?.keyPoints,
};

const state3: DialecticState = {
  id: 'act-b-9',
  title: 'Relative necessity begins from contingency',
  concept: 'RelativeNecessity',
  phase: 'appearance',
  moments: [
    {
      name: 'presupposedContingency',
      definition:
        'Necessity starts from contingent multiplicity and presupposed unity',
      type: 'reflection',
    },
    {
      name: 'necessityFromConditions',
      definition: 'Real possibility becomes necessity through positedness',
      type: 'mediation',
      relation: 'transitions',
      relatedTo: 'presupposedContingency',
    },
  ],
  invariants: [
    {
      id: 'act-b-9-inv-1',
      constraint: 'relative necessity has presupposition',
      predicate: 'has(relativeNecessity, presupposition)',
    },
    {
      id: 'act-b-9-inv-2',
      constraint: 'movement and presupposing remain distinct',
      predicate: 'distinct(movement, presupposing)',
    },
  ],
  forces: [
    {
      id: 'act-b-9-force-1',
      description:
        'Presupposed necessity collapses into formal contingency articulation',
      type: 'passover',
      trigger: 'relativeNecessity.limitation = explicit',
      effect: 'formalActuality.emerges = true',
      targetState: 'act-a-1',
    },
  ],
  transitions: [
    {
      id: 'act-b-9-trans-1',
      from: 'act-b-9',
      to: 'act-a-1',
      mechanism: 'passover',
      description: 'From relative necessity into contingency/formal actuality',
    },
  ],
  nextStates: ['act-a-1'],
  previousStates: ['act-b-6'],
  provenance: {
    topicMapId: 'act-b-9-relative-necessity',
    lineRange: { start: 187, end: 260 },
    section: 'Relative Necessity or Real Actuality, Possibility, and Necessity',
    order: 9,
  },
  description: RELATIVE_NECESSITY_TOPIC_MAP.entries[8]?.description,
  keyPoints: RELATIVE_NECESSITY_TOPIC_MAP.entries[8]?.keyPoints,
};

export const relativeNecessityIR: DialecticIR = {
  id: 'relative-necessity-ir',
  title:
    'Relative Necessity IR: Real Actuality, Condition Completion, Presupposed Necessity',
  section: 'ESSENCE - C. ACTUALITY - B. Actuality - Relative Necessity',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'relative-necessity.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'act-b-1': 'appearance',
      'act-b-6': 'appearance',
      'act-b-9': 'appearance',
    },
  },
};

export const relativeNecessityStates = {
  'act-b-1': state1,
  'act-b-6': state2,
  'act-b-9': state3,
};
