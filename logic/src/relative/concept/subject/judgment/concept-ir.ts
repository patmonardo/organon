import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { CONCEPT_TOPIC_MAP } from './sources/concept-topic-map';

const state1: DialecticState = {
  id: 'conc-1',
  title: 'Judgment of the concept as ought and adjudication',
  concept: 'JudgmentOfConceptAsOught',
  phase: 'subject',
  moments: [
    {
      name: 'ought',
      definition:
        'Concept appears as ought against which actuality is measured',
      type: 'determination',
    },
    {
      name: 'adjudication',
      definition:
        'Predicates of value express true adjudication of fact by concept',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'ought',
    },
  ],
  invariants: [
    {
      id: 'conc-1-inv-1',
      constraint:
        'judgment of concept is objective in relation to conceptive determinateness',
      predicate: 'objectiveIn(judgmentOfConcept, conceptiveDeterminateness)',
    },
    {
      id: 'conc-1-inv-2',
      constraint: 'evaluation measures actuality against concept',
      predicate: 'measuresAgainst(actuality, concept)',
    },
  ],
  forces: [
    {
      id: 'conc-1-force-1',
      description: 'Modality progression drives toward apodictic objectivity',
      type: 'mediation',
      trigger: 'assertoricAndProblematic.unstable = true',
      effect: 'apodicticMoment.emerges = true',
      targetState: 'conc-10',
    },
  ],
  transitions: [
    {
      id: 'conc-1-trans-1',
      from: 'conc-1',
      to: 'conc-10',
      mechanism: 'mediation',
      description: 'From initial ought-judgment to apodictic objectivity',
    },
  ],
  nextStates: ['conc-10'],
  previousStates: ['nec-15'],
  provenance: {
    topicMapId: 'conc-1-introduction-ought',
    lineRange: { start: 4, end: 19 },
    section: 'The Judgment of the Concept',
    order: 1,
  },
  description: CONCEPT_TOPIC_MAP.entries[0]?.description,
  keyPoints: CONCEPT_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'conc-10',
  title: 'Apodictic judgment as truly objective correspondence',
  concept: 'ApodicticObjectivity',
  phase: 'subject',
  moments: [
    {
      name: 'apodicticCorrespondence',
      definition:
        'Subject and predicate correspond as same concept in concrete universality',
      type: 'determination',
    },
    {
      name: 'groundInConstitution',
      definition:
        'Judgment has its ground in constitution and developed copula',
      type: 'reflection',
      relation: 'contains',
      relatedTo: 'apodicticCorrespondence',
    },
  ],
  invariants: [
    {
      id: 'conc-10-inv-1',
      constraint: 'subject and predicate correspond through one concept',
      predicate: 'equals(correspondence(subject, predicate), oneConcept)',
    },
    {
      id: 'conc-10-inv-2',
      constraint: 'copula is no longer abstract being but grounded connection',
      predicate: 'equals(copula, groundedConnection)',
    },
  ],
  forces: [
    {
      id: 'conc-10-force-1',
      description:
        'Apodictic grounding sublates judgment-form into completed copular content',
      type: 'sublation',
      trigger: 'groundInConstitution.explicit = true',
      effect: 'ideaBridge.matures = true',
      targetState: 'conc-14',
    },
  ],
  transitions: [
    {
      id: 'conc-10-trans-1',
      from: 'conc-10',
      to: 'conc-14',
      mechanism: 'sublation',
      description: 'From apodictic objectivity to syllogistic repletion',
    },
  ],
  nextStates: ['conc-14'],
  previousStates: ['conc-1'],
  provenance: {
    topicMapId: 'conc-10-apodictic-objective',
    lineRange: { start: 262, end: 288 },
    section: 'The Apodictic Judgment',
    order: 10,
  },
  description: CONCEPT_TOPIC_MAP.entries[9]?.description,
  keyPoints: CONCEPT_TOPIC_MAP.entries[9]?.keyPoints,
};

const state3: DialecticState = {
  id: 'conc-14',
  title: 'Fourth moment bridge: judgment of concept to syllogism and Idea',
  concept: 'IdeaBridgeFromJudgment',
  phase: 'subject',
  moments: [
    {
      name: 'accomplishedCopula',
      definition:
        'Copula is replete of content and holds the whole concept in connection',
      type: 'mediation',
    },
    {
      name: 'syllogisticEmergence',
      definition:
        'By this repletion, judgment becomes syllogism and opens Idea-level trajectory',
      type: 'passover',
      relation: 'transitions',
      relatedTo: 'accomplishedCopula',
    },
  ],
  invariants: [
    {
      id: 'conc-14-inv-1',
      constraint: 'subject and predicate are each whole concept',
      predicate: 'and(wholeConcept(subject), wholeConcept(predicate))',
    },
    {
      id: 'conc-14-inv-2',
      constraint: 'unity re-emerges through determinate connective form',
      predicate: 'reemerges(unityOfConcept, determinateConnection)',
    },
  ],
  forces: [
    {
      id: 'conc-14-force-1',
      description: 'Replete copula passes into syllogism of existence',
      type: 'passover',
      trigger: 'copula.repleteOfContent = true',
      effect: 'syllogismOfExistence.emerges = true',
      targetState: 'syl-exist-1',
    },
  ],
  transitions: [
    {
      id: 'conc-14-trans-1',
      from: 'conc-14',
      to: 'syl-exist-1',
      mechanism: 'passover',
      description:
        'From judgment of concept to syllogism (Idea-bridge handoff)',
    },
  ],
  nextStates: ['syl-exist-1'],
  previousStates: ['conc-10'],
  provenance: {
    topicMapId: 'conc-14-transition-syllogism',
    lineRange: { start: 346, end: 374 },
    section: 'The Apodictic Judgment',
    order: 14,
  },
  description: CONCEPT_TOPIC_MAP.entries[13]?.description,
  keyPoints: CONCEPT_TOPIC_MAP.entries[13]?.keyPoints,
};

export const conceptJudgmentIR: DialecticIR = {
  id: 'concept-judgment-ir',
  title:
    'Concept Judgment IR (Fourth Moment): Ought, Apodictic Truth, Idea Bridge',
  section:
    'CONCEPT - SUBJECTIVITY - B. Judgment - D. Judgment of the Concept (Idea Bridge)',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'concept.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'conc-1': 'subject',
      'conc-10': 'subject',
      'conc-14': 'subject',
    },
  },
};

export const conceptJudgmentStates = {
  'conc-1': state1,
  'conc-10': state2,
  'conc-14': state3,
};
