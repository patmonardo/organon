import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { ABSOLUTE_MECHANISM_TOPIC_MAP } from './sources/mechanism-topic-map';

const state1: DialecticState = {
  id: 'mech-abs-1',
  title: 'Center as objective singularity and striving',
  concept: 'AbsoluteMechanismCenter',
  phase: 'object',
  moments: [
    {
      name: 'objectiveCenter',
      definition:
        'Manifold objectivity gathers into centered singular universality',
      type: 'determination',
    },
    {
      name: 'strivingCentrality',
      definition:
        'Externality appears as striving back toward center through centrality',
      type: 'process',
      relation: 'contains',
      relatedTo: 'objectiveCenter',
    },
  ],
  invariants: [
    {
      id: 'mech-abs-1-inv-1',
      constraint: 'center is objective singular middle',
      predicate: 'equals(center, objectiveSingularMiddle)',
    },
    {
      id: 'mech-abs-1-inv-2',
      constraint:
        'central body functions as immanent genus of mechanical field',
      predicate: 'equals(centralBody, immanentGenus)',
    },
  ],
  forces: [
    {
      id: 'mech-abs-1-force-1',
      description:
        'Centered universality develops syllogistic articulation of mechanism',
      type: 'mediation',
      trigger: 'center.striving = immanent',
      effect: 'syllogisticMechanism.emerges = true',
      targetState: 'mech-abs-2',
    },
  ],
  transitions: [
    {
      id: 'mech-abs-1-trans-1',
      from: 'mech-abs-1',
      to: 'mech-abs-2',
      mechanism: 'mediation',
      description:
        'From objective center to free-mechanism syllogistic structure',
    },
  ],
  nextStates: ['mech-abs-2'],
  previousStates: ['mech-proc-8'],
  provenance: {
    topicMapId: 'mech-abs-1-center-singularity',
    lineRange: { start: 4, end: 64 },
    section: 'Absolute Mechanism - The Center',
    order: 1,
  },
  description: ABSOLUTE_MECHANISM_TOPIC_MAP.entries[0]?.description,
  keyPoints: ABSOLUTE_MECHANISM_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'mech-abs-2',
  title: 'Free mechanism and immanent law',
  concept: 'FreeMechanismLaw',
  phase: 'object',
  moments: [
    {
      name: 'threefoldSyllogisticField',
      definition:
        'Absolute center, relative centers, and formal objects form one free-mechanism structure',
      type: 'mediation',
    },
    {
      name: 'lawEmergence',
      definition: 'External order passes into immanent objective law',
      type: 'reflection',
      relation: 'transforms',
      relatedTo: 'threefoldSyllogisticField',
    },
  ],
  invariants: [
    {
      id: 'mech-abs-2-inv-1',
      constraint:
        'free mechanism has objective universality as fundamental determination',
      predicate:
        'fundamentalDetermination(freeMechanism, objectiveUniversality)',
    },
    {
      id: 'mech-abs-2-inv-2',
      constraint: 'law is immanent, not merely external arrangement',
      predicate: 'and(immanent(law), not(mereArrangement(law)))',
    },
  ],
  forces: [
    {
      id: 'mech-abs-2-force-1',
      description:
        'Law differentiates idealized and external reality into tense opposition',
      type: 'negation',
      trigger: 'law.differencePosited = true',
      effect: 'transitionToChemism.prepared = true',
      targetState: 'mech-abs-4',
    },
  ],
  transitions: [
    {
      id: 'mech-abs-2-trans-1',
      from: 'mech-abs-2',
      to: 'mech-abs-4',
      mechanism: 'negation',
      description: 'From immanent law to objectified opposition',
    },
  ],
  nextStates: ['mech-abs-4'],
  previousStates: ['mech-abs-1'],
  provenance: {
    topicMapId: 'mech-abs-2-center-syllogistic',
    lineRange: { start: 66, end: 165 },
    section: 'Absolute Mechanism - The Center',
    order: 2,
  },
  description: ABSOLUTE_MECHANISM_TOPIC_MAP.entries[1]?.description,
  keyPoints: ABSOLUTE_MECHANISM_TOPIC_MAP.entries[1]?.keyPoints,
};

const state3: DialecticState = {
  id: 'mech-abs-4',
  title: 'Mechanism-to-chemism bridge via objectified opposition',
  concept: 'TransitionToChemism',
  phase: 'object',
  moments: [
    {
      name: 'objectifiedOpposition',
      definition:
        'Centrality falls apart into reciprocally negative and tense objectivities',
      type: 'contradiction',
    },
    {
      name: 'chemismPassover',
      definition: 'Free mechanism determines itself into chemism',
      type: 'passover',
      relation: 'transitions',
      relatedTo: 'objectifiedOpposition',
    },
  ],
  invariants: [
    {
      id: 'mech-abs-4-inv-1',
      constraint:
        'law remains immanent but no longer sufficient as closed mechanism',
      predicate: 'and(immanent(law), insufficientAsClosedMechanism(law))',
    },
    {
      id: 'mech-abs-4-inv-2',
      constraint:
        'opposition is mutually negative, not indifferent externality',
      predicate: 'mutuallyNegative(opposedObjectivities)',
    },
  ],
  forces: [
    {
      id: 'mech-abs-4-force-1',
      description: 'Objectified negativity initiates chemical object relation',
      type: 'passover',
      trigger: 'opposition.tense = true',
      effect: 'chemicalObject.emerges = true',
      targetState: 'chem-obj-1',
    },
  ],
  transitions: [
    {
      id: 'mech-abs-4-trans-1',
      from: 'mech-abs-4',
      to: 'chem-obj-1',
      mechanism: 'passover',
      description: 'From absolute mechanism to chemism',
    },
  ],
  nextStates: ['chem-obj-1'],
  previousStates: ['mech-abs-2'],
  provenance: {
    topicMapId: 'mech-abs-4-transition-chemism',
    lineRange: { start: 236, end: 278 },
    section: 'Absolute Mechanism - Transition',
    order: 4,
  },
  description: ABSOLUTE_MECHANISM_TOPIC_MAP.entries[3]?.description,
  keyPoints: ABSOLUTE_MECHANISM_TOPIC_MAP.entries[3]?.keyPoints,
};

export const mechanismIR: DialecticIR = {
  id: 'mechanism-ir',
  title: 'Mechanism IR: Center, Free Law, Chemism Bridge',
  section: 'CONCEPT - OBJECTIVITY - A. Mechanism - C. Absolute Mechanism',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'mechanism.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'mech-abs-1': 'object',
      'mech-abs-2': 'object',
      'mech-abs-4': 'object',
    },
  },
};

export const mechanismStates = {
  'mech-abs-1': state1,
  'mech-abs-2': state2,
  'mech-abs-4': state3,
};
