import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { CHEMICAL_OBJECT_TOPIC_MAP } from './sources/object-topic-map';

const state1: DialecticState = {
  id: 'chem-obj-1',
  title: 'Chemical object distinguished from mechanism',
  concept: 'ChemicalObjectNonIndifference',
  phase: 'object',
  moments: [
    {
      name: 'nonIndifferentDeterminateness',
      definition:
        'Determinateness belongs to nature itself and is not accidental externality',
      type: 'determination',
    },
    {
      name: 'particularizationAsPrinciple',
      definition:
        'Particular determinateness is raised into universality as a principle',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'nonIndifferentDeterminateness',
    },
  ],
  invariants: [
    {
      id: 'chem-obj-1-inv-1',
      constraint: 'determinateness is intrinsic to object nature',
      predicate: 'intrinsicToNature(determinateness, object)',
    },
    {
      id: 'chem-obj-1-inv-2',
      constraint: 'chemical object is non-indifferent in relation',
      predicate: 'nonIndifferentInRelation(object)',
    },
  ],
  forces: [
    {
      id: 'chem-obj-1-force-1',
      description:
        'Intrinsic determinateness reflects the object into self-subsistent totality',
      type: 'reflection',
      trigger: 'determinateness.internalized = true',
      effect: 'selfSubsistentTotality.emerges = true',
      targetState: 'chem-obj-3',
    },
  ],
  transitions: [
    {
      id: 'chem-obj-1-trans-1',
      from: 'chem-obj-1',
      to: 'chem-obj-3',
      mechanism: 'reflection',
      description: 'From non-indifference to reflected self-subsistence',
    },
  ],
  nextStates: ['chem-obj-3'],
  previousStates: ['mech-abs-4'],
  provenance: {
    topicMapId: 'chem-obj-1-distinguished-mechanical',
    lineRange: { start: 2, end: 42 },
    section: 'The Chemical Object',
    order: 1,
  },
  description: CHEMICAL_OBJECT_TOPIC_MAP.entries[0]?.description,
  keyPoints: CHEMICAL_OBJECT_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'chem-obj-3',
  title: 'Self-subsistent totality in negative relation',
  concept: 'ChemicalSelfSubsistentTotality',
  phase: 'object',
  moments: [
    {
      name: 'reflectedTotality',
      definition:
        'Chemical object is reflected into itself while remaining essentially referred to other',
      type: 'determination',
    },
    {
      name: 'negativeUnityOfOpposedObjects',
      definition:
        'Its unity appears only across opposed particulars whose being is in one another',
      type: 'contradiction',
      relation: 'contains',
      relatedTo: 'reflectedTotality',
    },
  ],
  invariants: [
    {
      id: 'chem-obj-3-inv-1',
      constraint: 'chemical object cannot be comprehended in isolation',
      predicate: 'not(comprehensibleInIsolation(chemicalObject))',
    },
    {
      id: 'chem-obj-3-inv-2',
      constraint: 'object being is mediated through the other object',
      predicate: 'mediatedThroughOther(being(object))',
    },
  ],
  forces: [
    {
      id: 'chem-obj-3-force-1',
      description:
        'Negative unity sharpens into contradiction and active striving',
      type: 'contradiction',
      trigger: 'unity.onlyExternalized = true',
      effect: 'strivingContradiction.emerges = true',
      targetState: 'chem-obj-4',
    },
  ],
  transitions: [
    {
      id: 'chem-obj-3-trans-1',
      from: 'chem-obj-3',
      to: 'chem-obj-4',
      mechanism: 'contradiction',
      description: 'From reflected totality to explicit contradiction',
    },
  ],
  nextStates: ['chem-obj-4'],
  previousStates: ['chem-obj-1'],
  provenance: {
    topicMapId: 'chem-obj-3-self-subsistent-totality',
    lineRange: { start: 44, end: 66 },
    section: 'The Chemical Object',
    order: 2,
  },
  description: CHEMICAL_OBJECT_TOPIC_MAP.entries[2]?.description,
  keyPoints: CHEMICAL_OBJECT_TOPIC_MAP.entries[2]?.keyPoints,
};

const state3: DialecticState = {
  id: 'chem-obj-4',
  title: 'Contradiction and striving initiate process',
  concept: 'TransitionToChemicalProcess',
  phase: 'object',
  moments: [
    {
      name: 'absoluteReflection',
      definition:
        'Determinateness is absolutely reflected as real genus in the object',
      type: 'reflection',
    },
    {
      name: 'selfDeterminingStriving',
      definition:
        'Object strives to sublate one-sided immediacy and realize conceptual totality',
      type: 'passover',
      relation: 'transitions',
      relatedTo: 'absoluteReflection',
    },
  ],
  invariants: [
    {
      id: 'chem-obj-4-inv-1',
      constraint: 'object is contradiction of posited immediacy and concept',
      predicate:
        'contradiction(immediatePositedness(object), immanentConcept(object))',
    },
    {
      id: 'chem-obj-4-inv-2',
      constraint: 'striving is immanent, not externally imposed movement',
      predicate: 'immanent(striving(object))',
    },
  ],
  forces: [
    {
      id: 'chem-obj-4-force-1',
      description: 'Immanent striving unfolds as the chemical process itself',
      type: 'passover',
      trigger: 'striving.selfDetermining = true',
      effect: 'chemicalProcess.emerges = true',
      targetState: 'chem-proc-1',
    },
  ],
  transitions: [
    {
      id: 'chem-obj-4-trans-1',
      from: 'chem-obj-4',
      to: 'chem-proc-1',
      mechanism: 'passover',
      description: 'From chemical contradiction to chemical process',
    },
  ],
  nextStates: ['chem-proc-1'],
  previousStates: ['chem-obj-3'],
  provenance: {
    topicMapId: 'chem-obj-4-contradiction-striving',
    lineRange: { start: 67, end: 84 },
    section: 'The Chemical Object',
    order: 3,
  },
  description: CHEMICAL_OBJECT_TOPIC_MAP.entries[3]?.description,
  keyPoints: CHEMICAL_OBJECT_TOPIC_MAP.entries[3]?.keyPoints,
};

export const chemicalObjectIR: DialecticIR = {
  id: 'chemical-object-ir',
  title: 'Chemical Object IR: Non-Indifference, Totality, Process Bridge',
  section: 'CONCEPT - OBJECTIVITY - B. Chemism - A. The Chemical Object',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'object.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'chem-obj-1': 'object',
      'chem-obj-3': 'object',
      'chem-obj-4': 'object',
    },
  },
};

export const chemicalObjectStates = {
  'chem-obj-1': state1,
  'chem-obj-3': state2,
  'chem-obj-4': state3,
};
