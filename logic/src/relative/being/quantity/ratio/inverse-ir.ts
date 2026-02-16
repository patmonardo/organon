import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { INVERSE_TOPIC_MAP } from './sources/inverse-topic-map';

const inverseEntryById = (id: string) =>
  INVERSE_TOPIC_MAP.entries.find((entry) => entry.id === id);

const state1: DialecticState = {
  id: 'inverse-sublated-direct',
  title: 'Inverse ratio as sublated direct ratio',
  concept: 'InverseRatio',
  phase: 'quantity',
  moments: [
    {
      name: 'exponentAsProduct',
      definition:
        'Exponent now has the value of product and so unites unit and amount as qualitative limit',
      type: 'sublation',
    },
    {
      name: 'containedAlteration',
      definition:
        'Variation of sides is no longer arbitrary external change but enclosed by exponental limit',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'exponentAsProduct',
    },
  ],
  invariants: [
    {
      id: 'inverse-sublated-direct-inv-1',
      constraint:
        'inverse ratio sublates direct ratio while preserving its moments',
      predicate: 'sublatedPreservation(directRatio, inverseRatio)',
    },
    {
      id: 'inverse-sublated-direct-inv-2',
      constraint: 'exponent functions as negative qualitative boundary',
      predicate: 'qualitativeBoundary(exponent)',
    },
  ],
  forces: [
    {
      id: 'inverse-sublated-direct-force-1',
      description:
        'Enclosed alteration develops into explicit reciprocal limiting of the two sides',
      type: 'mediation',
      trigger: 'alteration.enclosedByExponent = true',
      effect: 'reciprocalLimiting.explicit = true',
      targetState: 'inverse-bad-infinity',
    },
  ],
  transitions: [
    {
      id: 'inverse-sublated-direct-trans-1',
      from: 'inverse-sublated-direct',
      to: 'inverse-bad-infinity',
      mechanism: 'mediation',
      description:
        'From inverse-ratio setup to contradiction of reciprocal limiting',
    },
  ],
  nextStates: ['inverse-bad-infinity'],
  previousStates: ['ratio-direct-incompleteness'],
  provenance: {
    topicMapId: 'inverse-sublated-direct',
    lineRange: { start: 4, end: 70 },
    section: 'B. THE INVERSE RATIO',
    order: 1,
  },
  description: inverseEntryById('inverse-sublated-direct')?.description,
  keyPoints: inverseEntryById('inverse-sublated-direct')?.keyPoints,
};

const state2: DialecticState = {
  id: 'inverse-bad-infinity',
  title: 'Inverse ratio bad infinity',
  concept: 'InverseBadInfinity',
  phase: 'quantity',
  moments: [
    {
      name: 'reciprocalLimiting',
      definition:
        'Each side is determined through the other and limits itself only by taking the other into itself',
      type: 'contradiction',
    },
    {
      name: 'approximationWithoutAttainment',
      definition:
        'Sides approximate the exponental unity but cannot attain it as fixed endpoint',
      type: 'process',
      relation: 'contains',
      relatedTo: 'reciprocalLimiting',
    },
  ],
  invariants: [
    {
      id: 'inverse-bad-infinity-inv-1',
      constraint: 'the in-itself of each side is the shared exponent',
      predicate: 'inItselfEachIsExponent(sides)',
    },
    {
      id: 'inverse-bad-infinity-inv-2',
      constraint: 'approximation reproduces the bad infinite process',
      predicate: 'badInfiniteApproximation(sides, exponent)',
    },
  ],
  forces: [
    {
      id: 'inverse-bad-infinity-force-1',
      description:
        'The exponent is affirmatively present in this contradiction and drives transition to powers',
      type: 'sublation',
      trigger: 'exponent.affirmativelyPresent = true',
      effect: 'ratioOfPowers.initiated = true',
      targetState: 'inverse-transition-powers',
    },
  ],
  transitions: [
    {
      id: 'inverse-bad-infinity-trans-1',
      from: 'inverse-bad-infinity',
      to: 'inverse-transition-powers',
      mechanism: 'sublation',
      description: 'From inverse bad infinity to powers transition',
    },
  ],
  nextStates: ['inverse-transition-powers'],
  previousStates: ['inverse-sublated-direct'],
  provenance: {
    topicMapId: 'inverse-bad-infinity',
    lineRange: { start: 125, end: 192 },
    section: 'B. THE INVERSE RATIO',
    order: 4,
  },
  description: inverseEntryById('inverse-bad-infinity')?.description,
  keyPoints: inverseEntryById('inverse-bad-infinity')?.keyPoints,
};

const state3: DialecticState = {
  id: 'inverse-transition-powers',
  title: 'Transition from inverse ratio to powers',
  concept: 'TransitionToPowers',
  phase: 'quantity',
  moments: [
    {
      name: 'selfMediatingExponent',
      definition:
        'Exponent becomes mediation of itself with itself in its finite otherness',
      type: 'mediation',
    },
    {
      name: 'affirmativeNegation',
      definition:
        'Negation of negation yields affirmative self-relation that externalizes without losing itself',
      type: 'sublation',
      relation: 'transitions',
      relatedTo: 'selfMediatingExponent',
    },
  ],
  invariants: [
    {
      id: 'inverse-transition-powers-inv-1',
      constraint: 'exponent encloses ratio moments implicitly as one totality',
      predicate: 'implicitEnclosure(exponent, ratioMoments)',
    },
    {
      id: 'inverse-transition-powers-inv-2',
      constraint:
        'qualitative self-relation is preserved in external existence',
      predicate: 'preservedInExternality(qualitativeSelfRelation)',
    },
  ],
  forces: [
    {
      id: 'inverse-transition-powers-force-1',
      description:
        'Affirmative self-mediation determines ratio explicitly as ratio of powers',
      type: 'passover',
      trigger: 'exponent.selfMediating = true',
      effect: 'powersRatio.explicit = true',
      targetState: 'powers-being-for-itself',
    },
  ],
  transitions: [
    {
      id: 'inverse-transition-powers-trans-1',
      from: 'inverse-transition-powers',
      to: 'powers-being-for-itself',
      mechanism: 'passover',
      description: 'From inverse ratio to ratio of powers',
    },
  ],
  nextStates: ['powers-being-for-itself'],
  previousStates: ['inverse-bad-infinity'],
  provenance: {
    topicMapId: 'inverse-transition-powers',
    lineRange: { start: 194, end: 268 },
    section: 'B. THE INVERSE RATIO',
    order: 5,
  },
  description: inverseEntryById('inverse-transition-powers')?.description,
  keyPoints: inverseEntryById('inverse-transition-powers')?.keyPoints,
};

export const inverseIR: DialecticIR = {
  id: 'inverse-ir',
  title: 'Inverse IR: The Inverse Ratio',
  section: 'BEING - QUANTITY - D. Inverse Ratio',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'inverse.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'inverse-sublated-direct': 'quantity',
      'inverse-bad-infinity': 'quantity',
      'inverse-transition-powers': 'quantity',
    },
  },
};

export const inverseStates = {
  'inverse-sublated-direct': state1,
  'inverse-bad-infinity': state2,
  'inverse-transition-powers': state3,
};
