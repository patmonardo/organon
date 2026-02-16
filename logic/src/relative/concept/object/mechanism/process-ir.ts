import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { MECHANICAL_PROCESS_TOPIC_MAP } from './sources/process-topic-map';

const state1: DialecticState = {
  id: 'mech-proc-3',
  title: 'Formal mechanical process: communication, reaction, product',
  concept: 'FormalMechanicalProcess',
  phase: 'object',
  moments: [
    {
      name: 'communicationReactionCycle',
      definition:
        'Universality communicates and is particularized by reciprocal reaction',
      type: 'process',
    },
    {
      name: 'formalProductRest',
      definition:
        'Product returns to rest with determinateness still external/posited',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'communicationReactionCycle',
    },
  ],
  invariants: [
    {
      id: 'mech-proc-3-inv-1',
      constraint:
        'communication and reaction are one process in opposite determination',
      predicate: 'oneProcessOpposed(communication, reaction)',
    },
    {
      id: 'mech-proc-3-inv-2',
      constraint: 'formal product preserves externality of determinateness',
      predicate: 'external(determinateness(formalProduct))',
    },
  ],
  forces: [
    {
      id: 'mech-proc-3-force-1',
      description:
        'Formal product contradiction drives toward real determined opposition',
      type: 'contradiction',
      trigger: 'rest.externalToObject = true',
      effect: 'realProcess.emerges = true',
      targetState: 'mech-proc-6',
    },
  ],
  transitions: [
    {
      id: 'mech-proc-3-trans-1',
      from: 'mech-proc-3',
      to: 'mech-proc-6',
      mechanism: 'contradiction',
      description: 'From formal process to real mechanical process',
    },
  ],
  nextStates: ['mech-proc-6'],
  previousStates: ['mech-obj-7'],
  provenance: {
    topicMapId: 'mech-proc-3-formal-communication',
    lineRange: { start: 96, end: 147 },
    section: 'The Formal Mechanical Process',
    order: 3,
  },
  description: MECHANICAL_PROCESS_TOPIC_MAP.entries[2]?.description,
  keyPoints: MECHANICAL_PROCESS_TOPIC_MAP.entries[2]?.keyPoints,
};

const state2: DialecticState = {
  id: 'mech-proc-6',
  title: 'Real process: resistance, power, and fate',
  concept: 'RealMechanicalProcess',
  phase: 'object',
  moments: [
    {
      name: 'determinedOpposition',
      definition:
        'Objects stand in determined opposition of self-subsistent singularity and universality',
      type: 'determination',
    },
    {
      name: 'powerAsFate',
      definition:
        'Overpowering resistance reveals objective universality as fate',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'determinedOpposition',
    },
  ],
  invariants: [
    {
      id: 'mech-proc-6-inv-1',
      constraint:
        'violence is objective universality confronting external singularity',
      predicate:
        'equals(violence, objectiveUniversalityAgainstExternalSingularity)',
    },
    {
      id: 'mech-proc-6-inv-2',
      constraint: 'fate emerges where self-conscious deed externalizes itself',
      predicate: 'emergesFrom(fate, externalizedDeed)',
    },
  ],
  forces: [
    {
      id: 'mech-proc-6-force-1',
      description: 'Real process reflects negativity into center/law result',
      type: 'reflection',
      trigger: 'power.negativityReflected = true',
      effect: 'centerLawProduct.emerges = true',
      targetState: 'mech-proc-8',
    },
  ],
  transitions: [
    {
      id: 'mech-proc-6-trans-1',
      from: 'mech-proc-6',
      to: 'mech-proc-8',
      mechanism: 'reflection',
      description: 'From resistance/power to product as center and law',
    },
  ],
  nextStates: ['mech-proc-8'],
  previousStates: ['mech-proc-3'],
  provenance: {
    topicMapId: 'mech-proc-6-real-communication',
    lineRange: { start: 252, end: 325 },
    section: 'The Real Mechanical Process',
    order: 6,
  },
  description: MECHANICAL_PROCESS_TOPIC_MAP.entries[5]?.description,
  keyPoints: MECHANICAL_PROCESS_TOPIC_MAP.entries[5]?.keyPoints,
};

const state3: DialecticState = {
  id: 'mech-proc-8',
  title: 'Product as center and law, foundation of mechanism',
  concept: 'TransitionToAbsoluteMechanism',
  phase: 'object',
  moments: [
    {
      name: 'centeredObjectivity',
      definition:
        'Objectivity becomes centered singular unity through reflected determinateness',
      type: 'determination',
    },
    {
      name: 'lawAsRationalFate',
      definition: 'Law is immanent rational fate particularizing from within',
      type: 'mediation',
      relation: 'transitions',
      relatedTo: 'centeredObjectivity',
    },
  ],
  invariants: [
    {
      id: 'mech-proc-8-inv-1',
      constraint: 'center is objective oneness of process result',
      predicate: 'equals(center, objectiveOneness(processResult))',
    },
    {
      id: 'mech-proc-8-inv-2',
      constraint: 'law constitutes truth and foundation of process',
      predicate: 'grounds(law, mechanicalProcess)',
    },
  ],
  forces: [
    {
      id: 'mech-proc-8-force-1',
      description:
        'Centered lawful product passes to absolute mechanism articulation',
      type: 'passover',
      trigger: 'law.immanent = true',
      effect: 'absoluteMechanism.emerges = true',
      targetState: 'mech-abs-1',
    },
  ],
  transitions: [
    {
      id: 'mech-proc-8-trans-1',
      from: 'mech-proc-8',
      to: 'mech-abs-1',
      mechanism: 'passover',
      description: 'From mechanical process product to absolute mechanism',
    },
  ],
  nextStates: ['mech-abs-1'],
  previousStates: ['mech-proc-6'],
  provenance: {
    topicMapId: 'mech-proc-8-product-center-law',
    lineRange: { start: 411, end: 456 },
    section: 'The Product of Mechanical Process',
    order: 8,
  },
  description: MECHANICAL_PROCESS_TOPIC_MAP.entries[7]?.description,
  keyPoints: MECHANICAL_PROCESS_TOPIC_MAP.entries[7]?.keyPoints,
};

export const mechanicalProcessIR: DialecticIR = {
  id: 'mechanical-process-ir',
  title: 'Mechanical Process IR: Formal Cycle, Real Opposition, Lawful Product',
  section: 'CONCEPT - OBJECTIVITY - A. Mechanism - B. The Mechanical Process',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'process.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'mech-proc-3': 'object',
      'mech-proc-6': 'object',
      'mech-proc-8': 'object',
    },
  },
};

export const mechanicalProcessStates = {
  'mech-proc-3': state1,
  'mech-proc-6': state2,
  'mech-proc-8': state3,
};
