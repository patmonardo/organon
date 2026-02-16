import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { SPECIFYING_MEASURE_TOPIC_MAP } from './sources/specifying-measure-topic-map';

const smEntryById = (id: string) =>
  SPECIFYING_MEASURE_TOPIC_MAP.entries.find((entry) => entry.id === id);

const state1: DialecticState = {
  id: 'measure-2',
  title: 'Specifying measure as lawful relation',
  concept: 'SpecifyingMeasure',
  phase: 'quantity',
  moments: [
    {
      name: 'rule',
      definition:
        'Specifying measure introduces a rule that determines quantitative alteration within qualitative bounds',
      type: 'determination',
    },
    {
      name: 'determinativeRelation',
      definition:
        'Measure now acts through determinate relation, not mere comparison',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'rule',
    },
    {
      name: 'variableMagnitude',
      definition:
        'Magnitude becomes variable while still governed by the qualitative measure-form',
      type: 'process',
    },
  ],
  invariants: [
    {
      id: 'measure-2-inv-1',
      constraint: 'specification is lawful and internal to measure',
      predicate: 'internalLawfulness(specifyingMeasure)',
    },
    {
      id: 'measure-2-inv-2',
      constraint: 'variation remains measure-determined',
      predicate: 'measureDeterminesVariation(magnitude)',
    },
  ],
  forces: [
    {
      id: 'measure-2-force-1',
      description:
        'Specifying relation crystallizes in exponent as qualitative moment',
      type: 'mediation',
      trigger: 'specifyingMeasure.requiresCentralMoment = true',
      effect: 'qualitativeExponent.explicit = true',
      targetState: 'measure-2-exponent',
    },
  ],
  transitions: [
    {
      id: 'measure-2-trans-1',
      from: 'measure-2',
      to: 'measure-2-exponent',
      mechanism: 'mediation',
      description: 'From specifying relation to exponental center',
    },
  ],
  nextStates: ['measure-2-exponent'],
  previousStates: ['measure-1-double-determination'],
  provenance: {
    topicMapId: 'sq-b-b-1-specifying-measure-determining',
    lineRange: { start: 34, end: 65 },
    section: 'Specifying Measure',
    order: 3,
  },
  description: smEntryById('sq-b-b-1-specifying-measure-determining')
    ?.description,
  keyPoints: smEntryById('sq-b-b-1-specifying-measure-determining')?.keyPoints,
};

const state2: DialecticState = {
  id: 'measure-2-exponent',
  title: 'Exponent as qualitative moment of specification',
  concept: 'ExponentQualitativeMoment',
  phase: 'quantity',
  moments: [
    {
      name: 'qualitativeExponent',
      definition:
        'Exponent operates as the qualitative center by which intensive and extensive sides are mediated',
      type: 'quality',
    },
    {
      name: 'powerDetermination',
      definition:
        'Genuine alteration shows itself as a power-determined relation, not external addition/subtraction',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'qualitativeExponent',
    },
    {
      name: 'relationOfQualities',
      definition:
        'The relation now exhibits itself as relation of qualities through variable magnitudes',
      type: 'process',
    },
  ],
  invariants: [
    {
      id: 'measure-2-exponent-inv-1',
      constraint: 'exponent is qualitative, not a merely numerical index',
      predicate: 'qualitativeExponent(exponent)',
    },
    {
      id: 'measure-2-exponent-inv-2',
      constraint: 'true alteration is power-determined',
      predicate: 'powerDeterminedAlteration(measure)',
    },
  ],
  forces: [
    {
      id: 'measure-2-exponent-force-1',
      description:
        'Exponental mediation passes into being-for-itself in measure',
      type: 'sublation',
      trigger: 'relation.ofQualitiesSelfUnifies = true',
      effect: 'measureBeingForItself.explicit = true',
      targetState: 'measure-c-immediate-specified-qualities',
    },
  ],
  transitions: [
    {
      id: 'measure-2-exponent-trans-1',
      from: 'measure-2-exponent',
      to: 'measure-c-immediate-specified-qualities',
      mechanism: 'sublation',
      description:
        'From exponental specification to being-for-itself in measure',
    },
  ],
  nextStates: ['measure-c-immediate-specified-qualities'],
  previousStates: ['measure-2'],
  provenance: {
    topicMapId: 'sq-b-b-3-exponent-as-qualitative-moment',
    lineRange: { start: 80, end: 104 },
    section: 'Specifying Measure',
    order: 5,
  },
  description: smEntryById('sq-b-b-3-exponent-as-qualitative-moment')
    ?.description,
  keyPoints: smEntryById('sq-b-b-3-exponent-as-qualitative-moment')?.keyPoints,
};

export const specifyingMeasureIR: DialecticIR = {
  id: 'specifying-measure-ir',
  title: 'Specifying Measure IR: Rule and Exponent',
  section: 'BEING - MEASURE - B. Specifying Measure',
  states: [state1, state2],
  metadata: {
    sourceFile: 'specifying-measure.txt',
    totalStates: 2,
    cpuGpuMapping: {
      'measure-2': 'quantity',
      'measure-2-exponent': 'quantity',
    },
  },
};

export const specifyingMeasureStates = {
  'measure-2': state1,
  'measure-2-exponent': state2,
};
