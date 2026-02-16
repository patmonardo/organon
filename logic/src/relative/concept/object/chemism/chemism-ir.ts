import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { chemismTopicMap } from './sources/chemism-topic-map';

const state1: DialecticState = {
  id: 'chem-1',
  title: 'Chemism as first negation and falling-apart syllogisms',
  concept: 'ChemismFirstNegation',
  phase: 'object',
  moments: [
    {
      name: 'firstNegationOfIndifferentObjectivity',
      definition:
        'Chemism negates indifferent objectivity but remains burdened by immediacy and externality',
      type: 'negation',
    },
    {
      name: 'threeSyllogismsFallApart',
      definition:
        'Formal neutrality, real neutrality, and self-realizing concept are present but externally conditioned',
      type: 'contradiction',
      relation: 'contains',
      relatedTo: 'firstNegationOfIndifferentObjectivity',
    },
  ],
  invariants: [
    {
      id: 'chem-1-inv-1',
      constraint: 'chemism is first negation of indifferent objectivity',
      predicate: 'equals(chemism, firstNegation(indifferentObjectivity))',
    },
    {
      id: 'chem-1-inv-2',
      constraint: 'three syllogisms are constituted but externally conditioned',
      predicate:
        'and(equals(syllogisms.count, 3), externallyConditioned(syllogisms))',
    },
  ],
  forces: [
    {
      id: 'chem-1-force-1',
      description:
        'Internal necessity differentiates the process into stages that sublate externality',
      type: 'mediation',
      trigger: 'conditionedProcesses.repeat = true',
      effect: 'stagewiseSublation.emerges = true',
      targetState: 'chem-2',
    },
  ],
  transitions: [
    {
      id: 'chem-1-trans-1',
      from: 'chem-1',
      to: 'chem-2',
      mechanism: 'mediation',
      description: 'From externally conditioned totality to staged sublation',
    },
  ],
  nextStates: ['chem-2'],
  previousStates: ['chem-proc-7'],
  provenance: {
    topicMapId: 'chemism-2',
    lineRange: { start: 25, end: 78 },
    section: 'C. TRANSITION OF CHEMISM',
    order: 1,
  },
  description: chemismTopicMap[1]?.description,
  keyPoints: chemismTopicMap[1]?.keyPoints,
};

const state2: DialecticState = {
  id: 'chem-2',
  title: 'Processes as stages of externality sublation',
  concept: 'ChemismStagewiseSublation',
  phase: 'object',
  moments: [
    {
      name: 'neutralizationAndDissolution',
      definition:
        'Neutralization and dissolution are distinct but necessarily united moments of one process',
      type: 'process',
    },
    {
      name: 'immanentPresupposingEmerges',
      definition:
        'Through repeated process, mediation becomes increasingly immanent self-determination',
      type: 'reflection',
      relation: 'transforms',
      relatedTo: 'neutralizationAndDissolution',
    },
  ],
  invariants: [
    {
      id: 'chem-2-inv-1',
      constraint: 'processes are necessary stages of de-externalization',
      predicate: 'necessaryStagesOfSublation(processes)',
    },
    {
      id: 'chem-2-inv-2',
      constraint: 'concept mediation tends toward self-determination',
      predicate: 'tendsTowardSelfDetermination(conceptMediation)',
    },
  ],
  forces: [
    {
      id: 'chem-2-force-1',
      description:
        'Last residue of indifferent immediacy is sublated into objective-free concept',
      type: 'sublation',
      trigger: 'externality.residue = minimal',
      effect: 'objectiveFreeConcept.emerges = true',
      targetState: 'chem-6',
    },
  ],
  transitions: [
    {
      id: 'chem-2-trans-1',
      from: 'chem-2',
      to: 'chem-6',
      mechanism: 'sublation',
      description: 'From staged mediation to concept liberation',
    },
  ],
  nextStates: ['chem-6'],
  previousStates: ['chem-1'],
  provenance: {
    topicMapId: 'chemism-5',
    lineRange: { start: 80, end: 105 },
    section: 'C. TRANSITION OF CHEMISM',
    order: 2,
  },
  description: chemismTopicMap[4]?.description,
  keyPoints: chemismTopicMap[4]?.keyPoints,
};

const state3: DialecticState = {
  id: 'chem-6',
  title: 'Objective-free concept as purpose',
  concept: 'TransitionToTeleology',
  phase: 'object',
  moments: [
    {
      name: 'conceptLiberatedFromExternality',
      definition:
        'The concept sublates external moments of objective existence into simple unity',
      type: 'sublation',
    },
    {
      name: 'purposeAsObjectiveFreeConcept',
      definition:
        'Freed concept relates to objectivity as unessential reality and appears as purpose',
      type: 'passover',
      relation: 'transitions',
      relatedTo: 'conceptLiberatedFromExternality',
    },
  ],
  invariants: [
    {
      id: 'chem-6-inv-1',
      constraint: 'external immediacy is now concept-moment',
      predicate: 'convertedToConceptMoment(externalImmediacy)',
    },
    {
      id: 'chem-6-inv-2',
      constraint: 'objective-free concept equals purpose',
      predicate: 'equals(objectiveFreeConcept, purpose)',
    },
  ],
  forces: [
    {
      id: 'chem-6-force-1',
      description:
        'Liberated concept determines itself as teleological activity',
      type: 'passover',
      trigger: 'objectiveFreeConcept.realized = true',
      effect: 'teleology.emerges = true',
      targetState: 'tele-1',
    },
  ],
  transitions: [
    {
      id: 'chem-6-trans-1',
      from: 'chem-6',
      to: 'tele-1',
      mechanism: 'passover',
      description: 'From chemism to teleology',
    },
  ],
  nextStates: ['tele-1'],
  previousStates: ['chem-2'],
  provenance: {
    topicMapId: 'chemism-6',
    lineRange: { start: 106, end: 119 },
    section: 'C. TRANSITION OF CHEMISM',
    order: 3,
  },
  description: chemismTopicMap[5]?.description,
  keyPoints: chemismTopicMap[5]?.keyPoints,
};

export const chemismIR: DialecticIR = {
  id: 'chemism-ir',
  title: 'Chemism IR: First Negation, Staged Sublation, Purpose',
  section: 'CONCEPT - OBJECTIVITY - B. Chemism - C. Transition of Chemism',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'chemism.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'chem-1': 'object',
      'chem-2': 'object',
      'chem-6': 'object',
    },
  },
};

export const chemismStates = {
  'chem-1': state1,
  'chem-2': state2,
  'chem-6': state3,
};
