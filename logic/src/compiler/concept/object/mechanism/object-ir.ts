import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { MECHANICAL_OBJECT_TOPIC_MAP } from './sources/object-topic-map';

const state1: DialecticState = {
  id: 'mech-obj-1',
  title: 'Mechanical object as syllogistic equilibrium',
  concept: 'MechanicalObjectEquilibrium',
  phase: 'object',
  moments: [
    {
      name: 'equilibriumIdentity',
      definition:
        'Object stands as immediate identity of mediated syllogistic moments',
      type: 'determination',
    },
    {
      name: 'totalityWithoutMatterFormSplit',
      definition:
        'Past relations of matter/form and accidents are sublated in objective totality',
      type: 'sublation',
      relation: 'contains',
      relatedTo: 'equilibriumIdentity',
    },
  ],
  invariants: [
    {
      id: 'mech-obj-1-inv-1',
      constraint: 'object is immediate totality of conceptive mediation',
      predicate: 'equals(object, immediateTotality(mediation))',
    },
    {
      id: 'mech-obj-1-inv-2',
      constraint:
        'object lacks property/accident externalization as essential form',
      predicate:
        'and(not(essential(properties(object))), not(essential(accidents(object))))',
    },
  ],
  forces: [
    {
      id: 'mech-obj-1-force-1',
      description:
        'Equilibrium object externalizes into plurality and relationless manifoldness',
      type: 'negation',
      trigger: 'internalDifference.notPosited = true',
      effect: 'plurality.emerges = true',
      targetState: 'mech-obj-3',
    },
  ],
  transitions: [
    {
      id: 'mech-obj-1-trans-1',
      from: 'mech-obj-1',
      to: 'mech-obj-3',
      mechanism: 'negation',
      description: 'From equilibrium totality to indeterminate plurality',
    },
  ],
  nextStates: ['mech-obj-3'],
  previousStates: ['syl-nec-11'],
  provenance: {
    topicMapId: 'mech-obj-1-syllogism-equilibrium',
    lineRange: { start: 2, end: 10 },
    section: 'The Mechanical Object',
    order: 1,
  },
  description: MECHANICAL_OBJECT_TOPIC_MAP.entries[0]?.description,
  keyPoints: MECHANICAL_OBJECT_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'mech-obj-3',
  title: 'Indeterminate plurality and external determinateness',
  concept: 'MechanicalPlurality',
  phase: 'object',
  moments: [
    {
      name: 'indeterminatePlurality',
      definition:
        'Object appears as composite/aggregate multiplicity of totalized singulars',
      type: 'determination',
    },
    {
      name: 'externalDeterminateness',
      definition:
        'Determinateness lies outside each object and recurs in infinite progression',
      type: 'externality',
      relation: 'contains',
      relatedTo: 'indeterminatePlurality',
    },
  ],
  invariants: [
    {
      id: 'mech-obj-3-inv-1',
      constraint: 'plurality remains relationless in itself',
      predicate: 'relationlessInItself(plurality)',
    },
    {
      id: 'mech-obj-3-inv-2',
      constraint: 'determinism lacks self-determining principle',
      predicate: 'not(hasSelfDeterminingPrinciple(determinism))',
    },
  ],
  forces: [
    {
      id: 'mech-obj-3-force-1',
      description:
        'External determinism collapses into contradiction of indifference and identity',
      type: 'contradiction',
      trigger: 'indifferenceAndIdentity.coexist = true',
      effect: 'mechanicalProcess.emerges = true',
      targetState: 'mech-obj-7',
    },
  ],
  transitions: [
    {
      id: 'mech-obj-3-trans-1',
      from: 'mech-obj-3',
      to: 'mech-obj-7',
      mechanism: 'contradiction',
      description: 'From plurality determinism to contradiction of mechanism',
    },
  ],
  nextStates: ['mech-obj-7'],
  previousStates: ['mech-obj-1'],
  provenance: {
    topicMapId: 'mech-obj-3-indeterminate-plurality',
    lineRange: { start: 36, end: 74 },
    section: 'The Mechanical Object',
    order: 3,
  },
  description: MECHANICAL_OBJECT_TOPIC_MAP.entries[2]?.description,
  keyPoints: MECHANICAL_OBJECT_TOPIC_MAP.entries[2]?.keyPoints,
};

const state3: DialecticState = {
  id: 'mech-obj-7',
  title: 'Contradiction yields the mechanical process',
  concept: 'TransitionToMechanicalProcess',
  phase: 'object',
  moments: [
    {
      name: 'negativeUnityOfPlurality',
      definition:
        'Objects are reciprocally repelling yet united by identical determinateness',
      type: 'contradiction',
    },
    {
      name: 'processPassover',
      definition: 'This negative unity passes into processual interaction',
      type: 'passover',
      relation: 'transitions',
      relatedTo: 'negativeUnityOfPlurality',
    },
  ],
  invariants: [
    {
      id: 'mech-obj-7-inv-1',
      constraint:
        'perfect indifference and identical determinateness are co-present',
      predicate:
        'and(perfectIndifference(objects), identicalDeterminateness(objects))',
    },
    {
      id: 'mech-obj-7-inv-2',
      constraint: 'contradiction is immanent, not externally imposed',
      predicate: 'immanent(contradiction(objects))',
    },
  ],
  forces: [
    {
      id: 'mech-obj-7-force-1',
      description: 'Negative unity unfolds as formal mechanical process',
      type: 'passover',
      trigger: 'contradiction.active = true',
      effect: 'formalProcess.emerges = true',
      targetState: 'mech-proc-1',
    },
  ],
  transitions: [
    {
      id: 'mech-obj-7-trans-1',
      from: 'mech-obj-7',
      to: 'mech-proc-1',
      mechanism: 'passover',
      description: 'From mechanical object contradiction to mechanical process',
    },
  ],
  nextStates: ['mech-proc-1'],
  previousStates: ['mech-obj-3'],
  provenance: {
    topicMapId: 'mech-obj-7-contradiction-mechanical-process',
    lineRange: { start: 132, end: 157 },
    section: 'The Mechanical Object',
    order: 7,
  },
  description: MECHANICAL_OBJECT_TOPIC_MAP.entries[6]?.description,
  keyPoints: MECHANICAL_OBJECT_TOPIC_MAP.entries[6]?.keyPoints,
};

export const mechanicalObjectIR: DialecticIR = {
  id: 'mechanical-object-ir',
  title: 'Mechanical Object IR: Equilibrium, Plurality, Process Transition',
  section: 'CONCEPT - OBJECTIVITY - A. Mechanism - A. The Mechanical Object',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'object.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'mech-obj-1': 'object',
      'mech-obj-3': 'object',
      'mech-obj-7': 'object',
    },
  },
};

export const mechanicalObjectStates = {
  'mech-obj-1': state1,
  'mech-obj-3': state2,
  'mech-obj-7': state3,
};
