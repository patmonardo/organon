import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { POWERS_TOPIC_MAP } from './sources/powers-topic-map';

const powersEntryById = (id: string) =>
  POWERS_TOPIC_MAP.entries.find((entry) => entry.id === id);

const state1: DialecticState = {
  id: 'powers-being-for-itself',
  title: 'Ratio of powers as quantitative being-for-itself',
  concept: 'PowersBeingForItself',
  phase: 'quantity',
  moments: [
    {
      name: 'unitAsSelfDeterminingAmount',
      definition:
        'Quantum determines its own otherness: unit and amount no longer stand as externally fixed moments',
      type: 'determination',
    },
    {
      name: 'powerAsSelfReferentialAggregate',
      definition:
        'Power gathers units such that each moment reflects the whole determination',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'unitAsSelfDeterminingAmount',
    },
  ],
  invariants: [
    {
      id: 'powers-being-for-itself-inv-1',
      constraint: 'quantum is self-identical in its own otherness',
      predicate: 'selfIdenticalInOtherness(quantum)',
    },
    {
      id: 'powers-being-for-itself-inv-2',
      constraint: 'otherness is determined through quantum itself',
      predicate: 'selfDeterminedOtherness(quantum)',
    },
  ],
  forces: [
    {
      id: 'powers-being-for-itself-force-1',
      description:
        'Self-determining power makes the qualitative side of exponent explicit',
      type: 'reflection',
      trigger: 'power.selfReferenceExplicit = true',
      effect: 'returnToQuality.explicit = true',
      targetState: 'powers-return-quality',
    },
  ],
  transitions: [
    {
      id: 'powers-being-for-itself-trans-1',
      from: 'powers-being-for-itself',
      to: 'powers-return-quality',
      mechanism: 'reflection',
      description: 'From being-for-itself to explicit qualitative return',
    },
  ],
  nextStates: ['powers-return-quality'],
  previousStates: ['inverse-transition-powers'],
  provenance: {
    topicMapId: 'powers-being-for-itself',
    lineRange: { start: 4, end: 24 },
    section: 'C. THE RATIO OF POWERS',
    order: 1,
  },
  description: powersEntryById('powers-being-for-itself')?.description,
  keyPoints: powersEntryById('powers-being-for-itself')?.keyPoints,
};

const state2: DialecticState = {
  id: 'powers-return-quality',
  title: 'Quantity returned into quality',
  concept: 'QuantityAsQuality',
  phase: 'quantity',
  moments: [
    {
      name: 'externalityAsOwnMoment',
      definition:
        'Externality is no longer indifferent outside but a moment mediated by quantum itself',
      type: 'sublation',
    },
    {
      name: 'qualityTruthOfQuantity',
      definition:
        'Quantity appears as quality itself: determinateness returned into self-reference',
      type: 'quality',
      relation: 'contains',
      relatedTo: 'externalityAsOwnMoment',
    },
  ],
  invariants: [
    {
      id: 'powers-return-quality-inv-1',
      constraint: 'quantum refers to itself in externality',
      predicate: 'selfReferenceInExternality(quantum)',
    },
    {
      id: 'powers-return-quality-inv-2',
      constraint: 'quantity and quality are no longer externally opposed',
      predicate: 'unity(quantity, quality)',
    },
  ],
  forces: [
    {
      id: 'powers-return-quality-force-1',
      description:
        'Unity of quantity and quality demands a full double transition into measure',
      type: 'sublation',
      trigger: 'quantityAsQuality.explicit = true',
      effect: 'doubleTransition.required = true',
      targetState: 'powers-double-transition-measure',
    },
  ],
  transitions: [
    {
      id: 'powers-return-quality-trans-1',
      from: 'powers-return-quality',
      to: 'powers-double-transition-measure',
      mechanism: 'sublation',
      description: 'From qualitative return to double transition',
    },
  ],
  nextStates: ['powers-double-transition-measure'],
  previousStates: ['powers-being-for-itself'],
  provenance: {
    topicMapId: 'powers-return-quality',
    lineRange: { start: 85, end: 115 },
    section: 'C. THE RATIO OF POWERS',
    order: 5,
  },
  description: powersEntryById('powers-return-quality')?.description,
  keyPoints: powersEntryById('powers-return-quality')?.keyPoints,
};

const state3: DialecticState = {
  id: 'powers-double-transition-measure',
  title: 'Double transition into measure',
  concept: 'MeasureTransition',
  phase: 'quantity',
  moments: [
    {
      name: 'qualityToQuantityToQuality',
      definition:
        'Totality requires a twofold movement: quality into quantity and quantity into quality',
      type: 'mediation',
    },
    {
      name: 'measureAsTruth',
      definition:
        'The truth of quantum is measure, where determinateness is qualitative-quantitative unity',
      type: 'sublation',
      relation: 'transitions',
      relatedTo: 'qualityToQuantityToQuality',
    },
  ],
  invariants: [
    {
      id: 'powers-double-transition-measure-inv-1',
      constraint: 'totality requires both directional transitions',
      predicate: 'requiresDoubleTransition(totality)',
    },
    {
      id: 'powers-double-transition-measure-inv-2',
      constraint: 'quantum no longer stands as indifferent limit',
      predicate: 'not(indifferentLimit(quantum))',
    },
  ],
  forces: [
    {
      id: 'powers-double-transition-measure-force-1',
      description: 'Completion of the ratio movement opens the measure chapter',
      type: 'passover',
      trigger: 'doubleTransition.completed = true',
      effect: 'measureChapter.initiated = true',
      targetState: 'measure-1',
    },
  ],
  transitions: [
    {
      id: 'powers-double-transition-measure-trans-1',
      from: 'powers-double-transition-measure',
      to: 'measure-1',
      mechanism: 'passover',
      description: 'From ratio of powers to measure',
    },
  ],
  nextStates: ['measure-1'],
  previousStates: ['powers-return-quality'],
  provenance: {
    topicMapId: 'powers-double-transition-measure',
    lineRange: { start: 117, end: 140 },
    section: 'C. THE RATIO OF POWERS',
    order: 6,
  },
  description: powersEntryById('powers-double-transition-measure')?.description,
  keyPoints: powersEntryById('powers-double-transition-measure')?.keyPoints,
};

export const powersIR: DialecticIR = {
  id: 'powers-ir',
  title: 'Powers IR: Ratio of Powers and Transition to Measure',
  section: 'BEING - QUANTITY - D. Ratio of Powers',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'powers.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'powers-being-for-itself': 'quantity',
      'powers-return-quality': 'quantity',
      'powers-double-transition-measure': 'quantity',
    },
  },
};

export const powersStates = {
  'powers-being-for-itself': state1,
  'powers-return-quality': state2,
  'powers-double-transition-measure': state3,
};
