import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { EXISTENCE_TOPIC_MAP } from './sources/existence-topic-map';

const state1: DialecticState = {
  id: 'exist-1',
  title: 'Judgment as immediate truth of concept and reality',
  concept: 'JudgmentAsImmediateTruth',
  phase: 'subject',
  moments: [
    {
      name: 'immediateJudgment',
      definition:
        'Judgment begins as immediate agreement of concept and actuality',
      type: 'determination',
    },
    {
      name: 'inherenceForm',
      definition:
        'Subject appears immediate and predicate inheres as founded determination',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'immediateJudgment',
    },
  ],
  invariants: [
    {
      id: 'exist-1-inv-1',
      constraint: 'judgment is agreement of concept and reality',
      predicate: 'equals(judgment, agreement(concept, reality))',
    },
    {
      id: 'exist-1-inv-2',
      constraint: 'mode is immediate qualitative judgment',
      predicate: 'equals(mode(judgment), qualitativeImmediate)',
    },
  ],
  forces: [
    {
      id: 'exist-1-force-1',
      description:
        'Immediate truth negates abstract positivity into mediated negativity',
      type: 'negation',
      trigger: 'positiveForm.provesInsufficient = true',
      effect: 'negativeJudgment.emerges = true',
      targetState: 'exist-9',
    },
  ],
  transitions: [
    {
      id: 'exist-1-trans-1',
      from: 'exist-1',
      to: 'exist-9',
      mechanism: 'negation',
      description: 'From immediate positive judgment to its negative truth',
    },
  ],
  nextStates: ['exist-9'],
  previousStates: ['sing-13'],
  provenance: {
    topicMapId: 'exist-1-introduction-truth',
    lineRange: { start: 3, end: 24 },
    section: 'The Judgment of Existence',
    order: 1,
  },
  description: EXISTENCE_TOPIC_MAP.entries[0]?.description,
  keyPoints: EXISTENCE_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'exist-9',
  title: 'Negative judgment as truth of positive judgment',
  concept: 'NegativeJudgmentTruth',
  phase: 'subject',
  moments: [
    {
      name: 'negativeJudgment',
      definition: 'The singular is not abstractly universal but particular',
      type: 'negation',
    },
    {
      name: 'mediatedParticularity',
      definition:
        'Particularity arises as mediated determination through negative connection',
      type: 'mediation',
      relation: 'transforms',
      relatedTo: 'negativeJudgment',
    },
  ],
  invariants: [
    {
      id: 'exist-9-inv-1',
      constraint: 'positive judgment contains contradiction in content',
      predicate: 'contradictionIn(positiveJudgment, content)',
    },
    {
      id: 'exist-9-inv-2',
      constraint: 'negative judgment posits singular as particular',
      predicate: 'equals(singular, particular)',
    },
  ],
  forces: [
    {
      id: 'exist-9-force-1',
      description:
        'Negation of negation drives to infinite judgment and sublation of immediate form',
      type: 'sublation',
      trigger: 'negativeJudgment.reflected = true',
      effect: 'infiniteJudgment.emerges = true',
      targetState: 'exist-18',
    },
  ],
  transitions: [
    {
      id: 'exist-9-trans-1',
      from: 'exist-9',
      to: 'exist-18',
      mechanism: 'sublation',
      description:
        'From negative judgment to sublation of judgment of existence',
    },
  ],
  nextStates: ['exist-18'],
  previousStates: ['exist-1'],
  provenance: {
    topicMapId: 'exist-9-truth-in-negative',
    lineRange: { start: 338, end: 411 },
    section: 'The Negative Judgment',
    order: 9,
  },
  description: EXISTENCE_TOPIC_MAP.entries[8]?.description,
  keyPoints: EXISTENCE_TOPIC_MAP.entries[8]?.keyPoints,
};

const state3: DialecticState = {
  id: 'exist-18',
  title: 'Judgment of existence sublates into reflection',
  concept: 'TransitionToReflectionJudgment',
  phase: 'subject',
  moments: [
    {
      name: 'sublatedExistenceJudgment',
      definition: 'Qualitative extremes are sublated in copular identity',
      type: 'sublation',
    },
    {
      name: 'reflectedTerms',
      definition:
        'Terms are now reflected into themselves rather than immediately determined',
      type: 'reflection',
      relation: 'transitions',
      relatedTo: 'sublatedExistenceJudgment',
    },
  ],
  invariants: [
    {
      id: 'exist-18-inv-1',
      constraint: 'identity in copula tears itself into renewed judgment form',
      predicate: 'tearsInto(identity(copula), judgmentForm)',
    },
    {
      id: 'exist-18-inv-2',
      constraint: 'new terms are reflected determinations',
      predicate: 'reflectedIntoThemselves(terms)',
    },
  ],
  forces: [
    {
      id: 'exist-18-force-1',
      description: 'Reflected terms pass over to judgment of reflection',
      type: 'passover',
      trigger: 'terms.reflected = true',
      effect: 'reflectionJudgment.emerges = true',
      targetState: 'refl-1',
    },
  ],
  transitions: [
    {
      id: 'exist-18-trans-1',
      from: 'exist-18',
      to: 'refl-1',
      mechanism: 'passover',
      description: 'From judgment of existence to judgment of reflection',
    },
  ],
  nextStates: ['refl-1'],
  previousStates: ['exist-9'],
  provenance: {
    topicMapId: 'exist-18-transition-reflection',
    lineRange: { start: 784, end: 798 },
    section: 'The Infinite Judgment',
    order: 18,
  },
  description: EXISTENCE_TOPIC_MAP.entries[17]?.description,
  keyPoints: EXISTENCE_TOPIC_MAP.entries[17]?.keyPoints,
};

export const existenceIR: DialecticIR = {
  id: 'existence-ir',
  title:
    'Existence Judgment IR: Immediate Truth, Negative Truth, Reflection Transition',
  section: 'CONCEPT - SUBJECTIVITY - B. Judgment - A. Judgment of Existence',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'existence.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'exist-1': 'subject',
      'exist-9': 'subject',
      'exist-18': 'subject',
    },
  },
};

export const existenceStates = {
  'exist-1': state1,
  'exist-9': state2,
  'exist-18': state3,
};
