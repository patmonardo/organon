import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { SPECIFIC_QUANTUM_TOPIC_MAP } from './sources/specific-quantum-topic-map';

const sqEntryById = (id: string) =>
  SPECIFIC_QUANTUM_TOPIC_MAP.entries.find((entry) => entry.id === id);

const state1: DialecticState = {
  id: 'measure-1',
  title: 'Measure as self-reference of quantum',
  concept: 'ImmediateMeasure',
  phase: 'quantity',
  moments: [
    {
      name: 'selfReferencedQuantum',
      definition:
        'Measure begins as quantum that refers itself to itself and thereby bears qualitative significance',
      type: 'determination',
    },
    {
      name: 'standardPresupposition',
      definition:
        'Every existent is determined through a measure-standard that gives its quantitative being determinate form',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'selfReferencedQuantum',
    },
    {
      name: 'qualityQuantityUnity',
      definition:
        'Quality is not external to quantity but present as the determinative meaning of quantum',
      type: 'quality',
    },
  ],
  invariants: [
    {
      id: 'measure-1-inv-1',
      constraint: 'whatever is has measure',
      predicate: 'hasMeasure(anything)',
    },
    {
      id: 'measure-1-inv-2',
      constraint: 'measure is unity of quality and quantity',
      predicate: 'unity(quality, quantity)',
    },
    {
      id: 'measure-1-inv-3',
      constraint: 'measure requires a standard relation',
      predicate: 'requiresStandard(measure)',
    },
  ],
  forces: [
    {
      id: 'measure-1-force-1',
      description:
        'Immediate measure articulates itself as a doubled quantitative determination',
      type: 'mediation',
      trigger: 'measure.immediacyNeedsExplicitStructure = true',
      effect: 'doubleDetermination.explicit = true',
      targetState: 'measure-1-double-determination',
    },
  ],
  transitions: [
    {
      id: 'measure-1-trans-1',
      from: 'measure-1',
      to: 'measure-1-double-determination',
      mechanism: 'mediation',
      description: 'From immediate measure to doubled determination',
    },
  ],
  nextStates: ['measure-1-double-determination'],
  previousStates: ['powers-double-transition-measure'],
  provenance: {
    topicMapId: 'sq-a-1-measure-as-self-reference',
    lineRange: { start: 3, end: 19 },
    section: 'The Specific Quantum',
    order: 1,
  },
  description: sqEntryById('sq-a-1-measure-as-self-reference')?.description,
  keyPoints: sqEntryById('sq-a-1-measure-as-self-reference')?.keyPoints,
};

const state2: DialecticState = {
  id: 'measure-1-double-determination',
  title: 'Immediate measure as double determination',
  concept: 'DoubleDetermination',
  phase: 'quantity',
  moments: [
    {
      name: 'intensiveAndExtensiveSides',
      definition:
        'Immediate measure contains intensive and extensive determination that belong together yet are distinguishable',
      type: 'determination',
    },
    {
      name: 'baldHeapProblematic',
      definition:
        'The threshold problem of bald/heap shows the instability of merely immediate delimitation',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'intensiveAndExtensiveSides',
    },
    {
      name: 'cunningOfConcept',
      definition:
        'Conceptual mediation silently governs quantitative drift toward determinate specification',
      type: 'process',
    },
  ],
  invariants: [
    {
      id: 'measure-1-double-inv-1',
      constraint: 'double determination is required for immediate measure',
      predicate: 'requiredDoubleDetermination(measure)',
    },
    {
      id: 'measure-1-double-inv-2',
      constraint:
        'indeterminate thresholds expose inadequacy of pure immediacy',
      predicate: 'inadequatePureImmediacy(thresholdCases)',
    },
  ],
  forces: [
    {
      id: 'measure-1-double-force-1',
      description:
        'Immediate ambiguity compels the explicit form of specifying measure',
      type: 'sublation',
      trigger: 'immediateMeasure.instability = true',
      effect: 'specifyingMeasure.introduced = true',
      targetState: 'measure-2',
    },
  ],
  transitions: [
    {
      id: 'measure-1-double-trans-1',
      from: 'measure-1-double-determination',
      to: 'measure-2',
      mechanism: 'sublation',
      description: 'From double determination to specifying measure',
    },
  ],
  nextStates: ['measure-2'],
  previousStates: ['measure-1'],
  provenance: {
    topicMapId: 'sq-a-4-immediate-measure-double-determination',
    lineRange: { start: 72, end: 108 },
    section: 'The Specific Quantum',
    order: 4,
  },
  description: sqEntryById('sq-a-4-immediate-measure-double-determination')
    ?.description,
  keyPoints: sqEntryById('sq-a-4-immediate-measure-double-determination')
    ?.keyPoints,
};

export const specificQuantumIR: DialecticIR = {
  id: 'specific-quantum-ir',
  title: 'Specific Quantum IR: Immediate Measure Dynamics',
  section: 'BEING - MEASURE - A. The Specific Quantum',
  states: [state1, state2],
  metadata: {
    sourceFile: 'specific-quantum.txt',
    totalStates: 2,
    cpuGpuMapping: {
      'measure-1': 'quantity',
      'measure-1-double-determination': 'quantity',
    },
  },
};

export const specificQuantumStates = {
  'measure-1': state1,
  'measure-1-double-determination': state2,
};
