import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { NECESSITY_TOPIC_MAP } from './sources/necessity-topic-map';

const state1: DialecticState = {
  id: 'nec-1',
  title: 'Objective universality as genus and species',
  concept: 'ObjectiveUniversality',
  phase: 'subject',
  moments: [
    {
      name: 'objectiveUniversality',
      definition: 'Universality exists in-and-for-itself as immanent necessity',
      type: 'determination',
    },
    {
      name: 'genusSpeciesForm',
      definition:
        'Distinction is posited as essential through genus and species',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'objectiveUniversality',
    },
  ],
  invariants: [
    {
      id: 'nec-1-inv-1',
      constraint: 'necessity is posited and immanent in distinction',
      predicate: 'and(posited(necessity), immanent(distinction))',
    },
    {
      id: 'nec-1-inv-2',
      constraint:
        'subjective contingency gives way to objective universal structure',
      predicate: 'subordinates(contingency, objectiveUniversality)',
    },
  ],
  forces: [
    {
      id: 'nec-1-force-1',
      description:
        'Objective universality develops into disjunctive conceptive form',
      type: 'mediation',
      trigger: 'genusSpeciesForm.explicit = true',
      effect: 'disjunctiveNecessity.emerges = true',
      targetState: 'nec-7',
    },
  ],
  transitions: [
    {
      id: 'nec-1-trans-1',
      from: 'nec-1',
      to: 'nec-7',
      mechanism: 'mediation',
      description: 'From objective universality to disjunctive necessity',
    },
  ],
  nextStates: ['nec-7'],
  previousStates: ['refl-14'],
  provenance: {
    topicMapId: 'nec-1-introduction-objective',
    lineRange: { start: 4, end: 24 },
    section: 'The Judgment of Necessity',
    order: 1,
  },
  description: NECESSITY_TOPIC_MAP.entries[0]?.description,
  keyPoints: NECESSITY_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'nec-7',
  title: 'Disjunctive judgment as necessity of concept',
  concept: 'DisjunctiveNecessity',
  phase: 'subject',
  moments: [
    {
      name: 'disjunctiveTotality',
      definition:
        'Objective universality is posited with differentiated determinations in totality',
      type: 'determination',
    },
    {
      name: 'negativeUnity',
      definition:
        'Either-or exclusion and as-well-as identity are one conceptual unity',
      type: 'contradiction',
      relation: 'contains',
      relatedTo: 'disjunctiveTotality',
    },
  ],
  invariants: [
    {
      id: 'nec-7-inv-1',
      constraint: 'disjunctive structure expresses necessity of concept',
      predicate: 'equals(disjunctiveStructure, conceptiveNecessity)',
    },
    {
      id: 'nec-7-inv-2',
      constraint: 'genus remains concrete unity of determinate particulars',
      predicate: 'equals(genus, concreteUnity(determinateParticulars))',
    },
  ],
  forces: [
    {
      id: 'nec-7-force-1',
      description: 'Negative unity determines itself as conceptive partition',
      type: 'reflection',
      trigger: 'negativeUnity.selfDetermined = true',
      effect: 'judgmentOfConcept.prepared = true',
      targetState: 'nec-15',
    },
  ],
  transitions: [
    {
      id: 'nec-7-trans-1',
      from: 'nec-7',
      to: 'nec-15',
      mechanism: 'reflection',
      description: 'From disjunctive necessity to concept-posited judgment',
    },
  ],
  nextStates: ['nec-15'],
  previousStates: ['nec-1'],
  provenance: {
    topicMapId: 'nec-7-disjunctive-either-or',
    lineRange: { start: 208, end: 243 },
    section: 'The Disjunctive Judgment',
    order: 7,
  },
  description: NECESSITY_TOPIC_MAP.entries[6]?.description,
  keyPoints: NECESSITY_TOPIC_MAP.entries[6]?.keyPoints,
};

const state3: DialecticState = {
  id: 'nec-15',
  title: 'Judgment itself disjoined as concept-posited unity',
  concept: 'TransitionToJudgmentOfConcept',
  phase: 'subject',
  moments: [
    {
      name: 'judgmentSelfDisjoined',
      definition:
        'Subject and predicate are themselves disjunctive members of one concept',
      type: 'sublation',
    },
    {
      name: 'copulaAsConcept',
      definition:
        'Copula becomes concept itself as developed connectedness and negative unity',
      type: 'mediation',
      relation: 'transitions',
      relatedTo: 'judgmentSelfDisjoined',
    },
  ],
  invariants: [
    {
      id: 'nec-15-inv-1',
      constraint: 'members are moments of one identical concept',
      predicate: 'equals(members(judgment), moments(oneConcept))',
    },
    {
      id: 'nec-15-inv-2',
      constraint: 'judgment of necessity rises to judgment of concept',
      predicate: 'risesTo(judgmentOfNecessity, judgmentOfConcept)',
    },
  ],
  forces: [
    {
      id: 'nec-15-force-1',
      description: 'Concept-posited copula passes to fourth judgment moment',
      type: 'passover',
      trigger: 'copulaAsConcept.explicit = true',
      effect: 'conceptJudgment.emerges = true',
      targetState: 'conc-1',
    },
  ],
  transitions: [
    {
      id: 'nec-15-trans-1',
      from: 'nec-15',
      to: 'conc-1',
      mechanism: 'passover',
      description: 'From judgment of necessity to judgment of the concept',
    },
  ],
  nextStates: ['conc-1'],
  previousStates: ['nec-7'],
  provenance: {
    topicMapId: 'nec-15-transition-concept',
    lineRange: { start: 424, end: 450 },
    section: 'The Disjunctive Judgment',
    order: 15,
  },
  description: NECESSITY_TOPIC_MAP.entries[14]?.description,
  keyPoints: NECESSITY_TOPIC_MAP.entries[14]?.keyPoints,
};

export const necessityIR: DialecticIR = {
  id: 'necessity-ir',
  title:
    'Necessity Judgment IR: Objective Universality, Disjunction, Concept Transition',
  section: 'CONCEPT - SUBJECTIVITY - B. Judgment - C. Judgment of Necessity',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'necessity.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'nec-1': 'subject',
      'nec-7': 'subject',
      'nec-15': 'subject',
    },
  },
};

export const necessityStates = {
  'nec-1': state1,
  'nec-7': state2,
  'nec-15': state3,
};
