import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { RATIO_TOPIC_MAP } from './sources/ratio-topic-map';

const ratioEntryById = (id: string) =>
  RATIO_TOPIC_MAP.entries.find((entry) => entry.id === id);

const state1: DialecticState = {
  id: 'ratio-intro-qualitative-moment',
  title: 'Ratio: quantum qualitatively determined',
  concept: 'QuantitativeRelation',
  phase: 'quantity',
  moments: [
    {
      name: 'infiniteQuantumAsRelation',
      definition:
        'Quantum is posited as relation: quantitative externality internally bears qualitative determinateness',
      type: 'determination',
    },
    {
      name: 'selfEnclosedTotality',
      definition:
        'Each quantum has determinateness through the other and returns into itself as infinite-within',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'infiniteQuantumAsRelation',
    },
  ],
  invariants: [
    {
      id: 'ratio-intro-qualitative-moment-inv-1',
      constraint: 'ratio unifies quantitative and qualitative moments',
      predicate: 'unity(quantitative, qualitative)',
    },
    {
      id: 'ratio-intro-qualitative-moment-inv-2',
      constraint: 'external reference is constitutive, not accidental',
      predicate: 'constitutiveReferenceToOther(quantum)',
    },
  ],
  forces: [
    {
      id: 'ratio-intro-qualitative-moment-force-1',
      description:
        'Relation must give itself an explicit shared determinateness as exponent',
      type: 'mediation',
      trigger: 'relation.requiresCommonLimit = true',
      effect: 'exponent.explicit = true',
      targetState: 'ratio-direct-exponent',
    },
  ],
  transitions: [
    {
      id: 'ratio-intro-qualitative-moment-trans-1',
      from: 'ratio-intro-qualitative-moment',
      to: 'ratio-direct-exponent',
      mechanism: 'mediation',
      description: 'From ratio in general to direct ratio exponent',
    },
  ],
  nextStates: ['ratio-direct-exponent'],
  previousStates: ['infinity-quantum-return-quality'],
  provenance: {
    topicMapId: 'ratio-intro-qualitative-moment',
    lineRange: { start: 6, end: 50 },
    section: 'Introduction',
    order: 1,
  },
  description: ratioEntryById('ratio-intro-qualitative-moment')?.description,
  keyPoints: ratioEntryById('ratio-intro-qualitative-moment')?.keyPoints,
};

const state2: DialecticState = {
  id: 'ratio-direct-exponent',
  title: 'Direct ratio: exponent as common determinateness',
  concept: 'DirectRatioExponent',
  phase: 'quantity',
  moments: [
    {
      name: 'commonLimit',
      definition:
        'The exponent is one determinateness of both quanta and so the direct ratio form',
      type: 'determination',
    },
    {
      name: 'unitAmountDifference',
      definition:
        'Within the exponent, difference appears as unit and amount, with qualitative determination',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'commonLimit',
    },
  ],
  invariants: [
    {
      id: 'ratio-direct-exponent-inv-1',
      constraint: 'each side is determined only through the other side',
      predicate: 'reciprocalDetermination(sides)',
    },
    {
      id: 'ratio-direct-exponent-inv-2',
      constraint: 'the exponent is still a quantum but qualitatively marked',
      predicate: 'qualitativelyMarkedQuantum(exponent)',
    },
  ],
  forces: [
    {
      id: 'ratio-direct-exponent-force-1',
      description:
        'Direct-ratio sides prove incomplete and negate their own isolated self-subsistence',
      type: 'contradiction',
      trigger: 'sides.onlyMomentaryCompleteness = true',
      effect: 'directRatioIncompleteness.explicit = true',
      targetState: 'ratio-direct-incompleteness',
    },
  ],
  transitions: [
    {
      id: 'ratio-direct-exponent-trans-1',
      from: 'ratio-direct-exponent',
      to: 'ratio-direct-incompleteness',
      mechanism: 'contradiction',
      description: 'From exponent form to incompleteness of direct ratio',
    },
  ],
  nextStates: ['ratio-direct-incompleteness'],
  previousStates: ['ratio-intro-qualitative-moment'],
  provenance: {
    topicMapId: 'ratio-direct-exponent-qualitative',
    lineRange: { start: 92, end: 106 },
    section: 'A. THE DIRECT RATIO',
    order: 4,
  },
  description: ratioEntryById('ratio-direct-exponent-qualitative')?.description,
  keyPoints: ratioEntryById('ratio-direct-exponent-qualitative')?.keyPoints,
};

const state3: DialecticState = {
  id: 'ratio-direct-incompleteness',
  title: 'Incompleteness of direct ratio',
  concept: 'DirectRatioSublation',
  phase: 'quantity',
  moments: [
    {
      name: 'momentarySides',
      definition:
        'Both sides belong to one quantum totality but each expresses only one moment of it',
      type: 'negation',
    },
    {
      name: 'determinedTransition',
      definition:
        'Negation is determined and qualitative, compelling a new ratio-form beyond direct immediacy',
      type: 'sublation',
      relation: 'transitions',
      relatedTo: 'momentarySides',
    },
  ],
  invariants: [
    {
      id: 'ratio-direct-incompleteness-inv-1',
      constraint: 'direct-ratio sides cannot each be complete quanta',
      predicate: 'not(eachCompleteQuantum(directRatioSides))',
    },
    {
      id: 'ratio-direct-incompleteness-inv-2',
      constraint:
        'the transition is determined negation, not arbitrary variation',
      predicate: 'determinedNegation(transition)',
    },
  ],
  forces: [
    {
      id: 'ratio-direct-incompleteness-force-1',
      description:
        'Sublation of direct incompleteness yields the inverse ratio structure',
      type: 'sublation',
      trigger: 'directRatio.incomplete = true',
      effect: 'inverseRatio.initiated = true',
      targetState: 'inverse-sublated-direct',
    },
  ],
  transitions: [
    {
      id: 'ratio-direct-incompleteness-trans-1',
      from: 'ratio-direct-incompleteness',
      to: 'inverse-sublated-direct',
      mechanism: 'sublation',
      description: 'From direct-ratio incompleteness to inverse ratio',
    },
  ],
  nextStates: ['inverse-sublated-direct'],
  previousStates: ['ratio-direct-exponent'],
  provenance: {
    topicMapId: 'ratio-direct-transition-inverse',
    lineRange: { start: 157, end: 191 },
    section: 'A. THE DIRECT RATIO',
    order: 7,
  },
  description: ratioEntryById('ratio-direct-transition-inverse')?.description,
  keyPoints: ratioEntryById('ratio-direct-transition-inverse')?.keyPoints,
};

export const ratioIR: DialecticIR = {
  id: 'ratio-ir',
  title: 'Ratio IR: Quantitative Relation',
  section: 'BEING - QUANTITY - D. Ratio',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'ratio.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'ratio-intro-qualitative-moment': 'quantity',
      'ratio-direct-exponent': 'quantity',
      'ratio-direct-incompleteness': 'quantity',
    },
  },
};

export const ratioStates = {
  'ratio-intro-qualitative-moment': state1,
  'ratio-direct-exponent': state2,
  'ratio-direct-incompleteness': state3,
};
