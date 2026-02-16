import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { processTopicMap } from './sources/process-topic-map';

const state1: DialecticState = {
  id: 'chem-proc-1',
  title: 'Affinity and communicative middle term',
  concept: 'ChemicalAffinityProcess',
  phase: 'object',
  moments: [
    {
      name: 'tensedObjects',
      definition:
        'Objects are in reciprocal tension and seek to sublate one-sidedness through affinity',
      type: 'determination',
    },
    {
      name: 'middleElementOfCommunication',
      definition:
        'A formal middle term mediates relation as communication element',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'tensedObjects',
    },
  ],
  invariants: [
    {
      id: 'chem-proc-1-inv-1',
      constraint: 'affinity presupposes objects in reciprocal tension',
      predicate: 'presupposes(affinity, reciprocalTension(objects))',
    },
    {
      id: 'chem-proc-1-inv-2',
      constraint: 'middle term is formal communication medium',
      predicate: 'isFormalCommunicationMedium(middleTerm)',
    },
  ],
  forces: [
    {
      id: 'chem-proc-1-force-1',
      description:
        'Reciprocal communication resolves immediate tension into neutral product',
      type: 'mediation',
      trigger: 'reciprocalBalancing.active = true',
      effect: 'neutralProduct.emerges = true',
      targetState: 'chem-proc-4',
    },
  ],
  transitions: [
    {
      id: 'chem-proc-1-trans-1',
      from: 'chem-proc-1',
      to: 'chem-proc-4',
      mechanism: 'mediation',
      description: 'From affinity to neutralized product',
    },
  ],
  nextStates: ['chem-proc-4'],
  previousStates: ['chem-obj-4'],
  provenance: {
    topicMapId: 'process-1',
    lineRange: { start: 4, end: 45 },
    section: 'B. THE PROCESS',
    order: 1,
  },
  description: processTopicMap[0]?.description,
  keyPoints: processTopicMap[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'chem-proc-4',
  title: 'Neutral product and externalized negativity',
  concept: 'ChemicalNeutrality',
  phase: 'object',
  moments: [
    {
      name: 'formalNeutrality',
      definition:
        'Process yields a neutral product where opposition is blunted into formal unity',
      type: 'determination',
    },
    {
      name: 'negativityStepsOutside',
      definition:
        'Essential negativity persists but external to the neutral object as restless activity',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'formalNeutrality',
    },
  ],
  invariants: [
    {
      id: 'chem-proc-4-inv-1',
      constraint: 'neutral product retains possibility of renewed tension',
      predicate: 'retainsPotentialForTension(neutralProduct)',
    },
    {
      id: 'chem-proc-4-inv-2',
      constraint: 'negativity is no longer immanent to product form',
      predicate: 'externalized(negativity, neutralProduct)',
    },
  ],
  forces: [
    {
      id: 'chem-proc-4-force-1',
      description:
        'External negativity disrupts neutrality and drives disjunctive articulation',
      type: 'contradiction',
      trigger: 'negativity.restless = true',
      effect: 'disjunctiveTotality.emerges = true',
      targetState: 'chem-proc-7',
    },
  ],
  transitions: [
    {
      id: 'chem-proc-4-trans-1',
      from: 'chem-proc-4',
      to: 'chem-proc-7',
      mechanism: 'contradiction',
      description: 'From formal neutrality to disjunctive totality',
    },
  ],
  nextStates: ['chem-proc-7'],
  previousStates: ['chem-proc-1'],
  provenance: {
    topicMapId: 'process-4',
    lineRange: { start: 62, end: 108 },
    section: 'B. THE PROCESS',
    order: 2,
  },
  description: processTopicMap[3]?.description,
  keyPoints: processTopicMap[3]?.keyPoints,
};

const state3: DialecticState = {
  id: 'chem-proc-7',
  title: 'Elemental liberation and self-sublation of chemism',
  concept: 'TransitionToChemismProper',
  phase: 'object',
  moments: [
    {
      name: 'elementalLiberation',
      definition:
        'Elemental objects are liberated from immediate chemical tension',
      type: 'sublation',
    },
    {
      name: 'returnAndHigherPassover',
      definition:
        'Chemism returns to its beginning and in this return sublates itself into a higher sphere',
      type: 'passover',
      relation: 'transitions',
      relatedTo: 'elementalLiberation',
    },
  ],
  invariants: [
    {
      id: 'chem-proc-7-inv-1',
      constraint: 'chemism returns to beginning through real process',
      predicate: 'returnsToBeginningThroughRealProcess(chemism)',
    },
    {
      id: 'chem-proc-7-inv-2',
      constraint: 'return is simultaneously self-sublation to higher sphere',
      predicate:
        'simultaneous(returnToConcept(chemism), selfSublation(chemism))',
    },
  ],
  forces: [
    {
      id: 'chem-proc-7-force-1',
      description:
        'Chemism as process-totality gives way to transition of chemism proper',
      type: 'passover',
      trigger: 'selfSublation(chemism) = true',
      effect: 'transitionOfChemism.emerges = true',
      targetState: 'chem-1',
    },
  ],
  transitions: [
    {
      id: 'chem-proc-7-trans-1',
      from: 'chem-proc-7',
      to: 'chem-1',
      mechanism: 'passover',
      description: 'From process totality to transition of chemism',
    },
  ],
  nextStates: ['chem-1'],
  previousStates: ['chem-proc-4'],
  provenance: {
    topicMapId: 'process-7',
    lineRange: { start: 156, end: 180 },
    section: 'B. THE PROCESS',
    order: 3,
  },
  description: processTopicMap[6]?.description,
  keyPoints: processTopicMap[6]?.keyPoints,
};

export const chemicalProcessIR: DialecticIR = {
  id: 'chemical-process-ir',
  title: 'Chemical Process IR: Affinity, Neutrality, Self-Sublation',
  section: 'CONCEPT - OBJECTIVITY - B. Chemism - B. The Process',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'process.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'chem-proc-1': 'object',
      'chem-proc-4': 'object',
      'chem-proc-7': 'object',
    },
  },
};

export const chemicalProcessStates = {
  'chem-proc-1': state1,
  'chem-proc-4': state2,
  'chem-proc-7': state3,
};
