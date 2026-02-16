import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { BEING_FOR_ITSELF_TOPIC_MAP } from './sources/being-for-itself-topic-map';

const bfiEntryById = (id: string) =>
  BEING_FOR_ITSELF_TOPIC_MAP.entries.find((entry) => entry.id === id);

const state1: DialecticState = {
  id: 'measure-c-immediate-specified-qualities',
  title: 'Immediate and specified qualities in measure',
  concept: 'MeasureQualityDifferentiation',
  phase: 'quantity',
  moments: [
    {
      name: 'immediateVsSpecified',
      definition:
        'Measure exhibits both immediate quality and quality specified through relation',
      type: 'determination',
    },
    {
      name: 'directRatioResidual',
      definition:
        'The relation retains a direct-ratio form while exceeding its immediacy',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'immediateVsSpecified',
    },
    {
      name: 'coefficientTendency',
      definition:
        'Empirical coefficients emerge as the concrete expression of measured relation',
      type: 'process',
    },
  ],
  invariants: [
    {
      id: 'measure-c-immediate-inv-1',
      constraint: 'immediate and specified quality remain one measure-process',
      predicate: 'unity(immediateQuality, specifiedQuality)',
    },
    {
      id: 'measure-c-immediate-inv-2',
      constraint: 'relation remains measure-determined despite variability',
      predicate: 'measureDeterminedRelation(qualityRelation)',
    },
  ],
  forces: [
    {
      id: 'measure-c-immediate-force-1',
      description:
        'The concrete relation seeks empirical coefficient determination',
      type: 'mediation',
      trigger: 'relation.requiresConcreteCoefficient = true',
      effect: 'empiricalCoefficient.explicit = true',
      targetState: 'measure-c-empirical-coefficient',
    },
  ],
  transitions: [
    {
      id: 'measure-c-immediate-trans-1',
      from: 'measure-c-immediate-specified-qualities',
      to: 'measure-c-empirical-coefficient',
      mechanism: 'mediation',
      description: 'From quality differentiation to empirical coefficient',
    },
  ],
  nextStates: ['measure-c-empirical-coefficient'],
  previousStates: ['measure-2-exponent'],
  provenance: {
    topicMapId: 'sq-c-1-qualities-immediate-vs-specified',
    lineRange: { start: 3, end: 30 },
    section: 'The Being-for-Itself in Measure',
    order: 1,
  },
  description: bfiEntryById('sq-c-1-qualities-immediate-vs-specified')
    ?.description,
  keyPoints: bfiEntryById('sq-c-1-qualities-immediate-vs-specified')?.keyPoints,
};

const state2: DialecticState = {
  id: 'measure-c-empirical-coefficient',
  title: 'Empirical coefficient and concrete measure relation',
  concept: 'EmpiricalMeasureCoefficient',
  phase: 'quantity',
  moments: [
    {
      name: 'empiricalCoefficient',
      definition:
        'Concrete measured relations appear as coefficients that encode determinate dependence',
      type: 'determination',
    },
    {
      name: 'measureComparability',
      definition:
        'Different measure-relations become comparable through coefficiental articulation',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'empiricalCoefficient',
    },
    {
      name: 'towardUnityOfMeasures',
      definition:
        'Coefficiental multiplicity drives toward a unifying measure-totality',
      type: 'process',
    },
  ],
  invariants: [
    {
      id: 'measure-c-empirical-inv-1',
      constraint:
        'coefficient remains qualitatively grounded, not merely numerical',
      predicate: 'qualitativelyGroundedCoefficient(measure)',
    },
    {
      id: 'measure-c-empirical-inv-2',
      constraint: 'comparability presupposes common measure-form',
      predicate: 'commonMeasureForm(comparableRelations)',
    },
  ],
  forces: [
    {
      id: 'measure-c-empirical-force-1',
      description: 'Coefficiental network sublates into unity of measures',
      type: 'sublation',
      trigger: 'measureNetwork.requiresUnity = true',
      effect: 'unityOfMeasures.explicit = true',
      targetState: 'measure-3',
    },
  ],
  transitions: [
    {
      id: 'measure-c-empirical-trans-1',
      from: 'measure-c-empirical-coefficient',
      to: 'measure-3',
      mechanism: 'sublation',
      description: 'From empirical coefficients to unity of measures',
    },
  ],
  nextStates: ['measure-3'],
  previousStates: ['measure-c-immediate-specified-qualities'],
  provenance: {
    topicMapId: 'sq-c-3-empirical-coefficient',
    lineRange: { start: 52, end: 141 },
    section: 'The Being-for-Itself in Measure',
    order: 3,
  },
  description: bfiEntryById('sq-c-3-empirical-coefficient')?.description,
  keyPoints: bfiEntryById('sq-c-3-empirical-coefficient')?.keyPoints,
};

const state3: DialecticState = {
  id: 'measure-3',
  title: 'Being-for-itself in measure: unity of measures',
  concept: 'MeasureBeingForItself',
  phase: 'quantity',
  moments: [
    {
      name: 'unityOfMeasures',
      definition:
        'Distinct measure-relations are gathered in a self-related totality of measure',
      type: 'sublation',
    },
    {
      name: 'immediacyAndSpecificationUnified',
      definition:
        'Immediate and specified qualities are unified in one measure-being-for-itself',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'unityOfMeasures',
    },
    {
      name: 'transitionToRealMeasure',
      definition:
        'Measure now demands self-subsistent measure-relations as real measure',
      type: 'process',
    },
  ],
  invariants: [
    {
      id: 'measure-3-inv-1',
      constraint:
        'being-for-itself in measure includes relation to other measures',
      predicate: 'selfReferenceThroughMeasureRelation(measure)',
    },
    {
      id: 'measure-3-inv-2',
      constraint: 'unity preserves differentiations as moments',
      predicate: 'momentPreservingUnity(measures)',
    },
  ],
  forces: [
    {
      id: 'measure-3-force-1',
      description:
        'Unified measure passes into the relation of independent real measures',
      type: 'passover',
      trigger: 'measure.unityRequiresExternalReality = true',
      effect: 'realMeasure.initiated = true',
      targetState: 'real-measure-1',
    },
  ],
  transitions: [
    {
      id: 'measure-3-trans-1',
      from: 'measure-3',
      to: 'real-measure-1',
      mechanism: 'passover',
      description: 'From specific quantity to real measure',
    },
  ],
  nextStates: ['real-measure-1'],
  previousStates: ['measure-c-empirical-coefficient'],
  provenance: {
    topicMapId: 'sq-c-4-unity-of-measures',
    lineRange: { start: 143, end: 190 },
    section: 'The Being-for-Itself in Measure',
    order: 4,
  },
  description: bfiEntryById('sq-c-4-unity-of-measures')?.description,
  keyPoints: bfiEntryById('sq-c-4-unity-of-measures')?.keyPoints,
};

export const specificQuantityIR: DialecticIR = {
  id: 'specific-quantity-ir',
  title: 'Specific Quantity IR: Being-for-itself in Measure',
  section: 'BEING - MEASURE - C. The Being-for-Itself in Measure',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'being-for-itself.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'measure-c-immediate-specified-qualities': 'quantity',
      'measure-c-empirical-coefficient': 'quantity',
      'measure-3': 'quantity',
    },
  },
};

export const specificQuantityStates = {
  'measure-c-immediate-specified-qualities': state1,
  'measure-c-empirical-coefficient': state2,
  'measure-3': state3,
};
