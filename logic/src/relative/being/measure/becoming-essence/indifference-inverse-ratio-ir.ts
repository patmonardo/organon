import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { INDIFFERENCE_INVERSE_RATIO_TOPIC_MAP } from './sources/indifference-inverse-ratio-topic-map';

const irEntryById = (id: string) =>
  INDIFFERENCE_INVERSE_RATIO_TOPIC_MAP.entries.find((entry) => entry.id === id);

const state1: DialecticState = {
  id: 'becoming-essence-1-inverse-ratio',
  title: 'Indifference posited as inverse ratio',
  concept: 'IndifferenceInverseRatio',
  phase: 'quantity',
  moments: [
    {
      name: 'substrateAndOneMeasure',
      definition:
        'Indifference appears as one substrate whose measure is internally divisible yet indivisible as basis',
      type: 'determination',
    },
    {
      name: 'inverseRatioForm',
      definition:
        'Differences relate inversely as determinations of one underlying indifferent whole',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'substrateAndOneMeasure',
    },
    {
      name: 'twoQualitiesEmergence',
      definition:
        'The one relation unfolds into two qualities that maintain one connection through inversion',
      type: 'process',
    },
  ],
  invariants: [
    {
      id: 'becoming-essence-1-inverse-inv-1',
      constraint: 'inverse relation belongs to one substrate',
      predicate: 'oneSubstrateInverseRelation(indifference)',
    },
    {
      id: 'becoming-essence-1-inverse-inv-2',
      constraint: 'qualitative difference is mediated through inverse quantity',
      predicate: 'qualityMediatedByInverseQuantity(relation)',
    },
  ],
  forces: [
    {
      id: 'becoming-essence-1-inverse-force-1',
      description:
        'The inverse relation externalizes as determinate existence and equilibrium tension',
      type: 'mediation',
      trigger: 'inverseRelation.becomesExistential = true',
      effect: 'equilibriumContradiction.explicit = true',
      targetState: 'becoming-essence-2',
    },
  ],
  transitions: [
    {
      id: 'becoming-essence-1-inverse-trans-1',
      from: 'becoming-essence-1-inverse-ratio',
      to: 'becoming-essence-2',
      mechanism: 'mediation',
      description:
        'From inverse-ratio articulation to contradiction of equilibrium',
    },
  ],
  nextStates: ['becoming-essence-2'],
  previousStates: ['becoming-essence-1'],
  provenance: {
    topicMapId: 'be-b-1-inverse-ratio',
    lineRange: { start: 24, end: 52 },
    section: '1.',
    order: 3,
  },
  description: irEntryById('be-b-1-inverse-ratio')?.description,
  keyPoints: irEntryById('be-b-1-inverse-ratio')?.keyPoints,
};

const state2: DialecticState = {
  id: 'becoming-essence-2',
  title: 'Contradiction of indifference in existence',
  concept: 'IndifferenceContradiction',
  phase: 'quantity',
  moments: [
    {
      name: 'indifferenceAsExistence',
      definition:
        'Indifference gains existence as articulated determinacies that emerge from the substrate relation',
      type: 'determination',
    },
    {
      name: 'equilibriumAsTension',
      definition:
        'Qualitative equilibrium appears as internally unstable and contradiction-bearing',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'indifferenceAsExistence',
    },
    {
      name: 'essencePressure',
      definition:
        'Contradiction presses determinations toward a non-immediate reflective ground',
      type: 'process',
    },
  ],
  invariants: [
    {
      id: 'becoming-essence-2-inv-1',
      constraint: 'indifference and determinacy are in unresolved tension',
      predicate: 'tension(indifference, determinacy)',
    },
    {
      id: 'becoming-essence-2-inv-2',
      constraint: 'equilibrium is contradiction, not final repose',
      predicate: 'equilibriumAsContradiction(indifference)',
    },
  ],
  forces: [
    {
      id: 'becoming-essence-2-force-1',
      description:
        'Contradiction manifests as explicit self-sublation of measure-being',
      type: 'sublation',
      trigger: 'contradiction.matures = true',
      effect: 'selfSublation.explicit = true',
      targetState: 'becoming-essence-3',
    },
  ],
  transitions: [
    {
      id: 'becoming-essence-2-trans-1',
      from: 'becoming-essence-2',
      to: 'becoming-essence-3',
      mechanism: 'sublation',
      description: 'From contradiction to self-sublation manifested',
    },
  ],
  nextStates: ['becoming-essence-3'],
  previousStates: ['becoming-essence-1-inverse-ratio'],
  provenance: {
    topicMapId: 'be-b-3-contradiction-essence',
    lineRange: { start: 197, end: 229 },
    section: '3.',
    order: 11,
  },
  description: irEntryById('be-b-3-contradiction-essence')?.description,
  keyPoints: irEntryById('be-b-3-contradiction-essence')?.keyPoints,
};

export const indifferenceInverseRatioIR: DialecticIR = {
  id: 'indifference-inverse-ratio-ir',
  title: 'Indifference and Inverse Ratio IR',
  section:
    'BEING - MEASURE - C. Becoming of Essence - Indifference and Inverse Ratio',
  states: [state1, state2],
  metadata: {
    sourceFile: 'indifference-inverse-ratio.txt',
    totalStates: 2,
    cpuGpuMapping: {
      'becoming-essence-1-inverse-ratio': 'quantity',
      'becoming-essence-2': 'quantity',
    },
  },
};

export const indifferenceInverseRatioStates = {
  'becoming-essence-1-inverse-ratio': state1,
  'becoming-essence-2': state2,
};
