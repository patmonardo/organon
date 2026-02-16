import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { NUMBER_TOPIC_MAP } from './sources/number-topic-map';

const state1: DialecticState = {
  id: 'number-quantum-as-number',
  title: 'Quantum as number: complete positedness',
  concept: 'QuantumAsNumber',
  phase: 'quantity',
  moments: [
    {
      name: 'completePositedness',
      definition:
        'Quantum is posited as number, with limit explicit as plurality in unity',
      type: 'determination',
    },
    {
      name: 'oneAsPrinciple',
      definition:
        'The one is self-referring, enclosing, and excluding while preserving continuity',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'completePositedness',
    },
  ],
  invariants: [
    {
      id: 'number-quantum-as-number-inv-1',
      constraint: 'number is the explicit quantitative limit',
      predicate: 'explicitLimit(number)',
    },
    {
      id: 'number-quantum-as-number-inv-2',
      constraint: 'discreteness and continuity remain united in number',
      predicate: 'unity(discreteness, continuity)',
    },
  ],
  forces: [
    {
      id: 'number-quantum-as-number-force-1',
      description:
        'Complete positedness differentiates into the moments of amount and unit in concrete delimitation',
      type: 'mediation',
      trigger: 'number.momentsRequireConcreteShape = true',
      effect: 'amountInLimit.explicit = true',
      targetState: 'number-amount-in-limit',
    },
  ],
  transitions: [
    {
      id: 'number-quantum-as-number-trans-1',
      from: 'number-quantum-as-number',
      to: 'number-amount-in-limit',
      mechanism: 'mediation',
      description:
        'From abstract complete positedness to amount and unit as determinate limit',
    },
  ],
  nextStates: ['number-amount-in-limit'],
  previousStates: ['quantity-limiting-quantum'],
  provenance: {
    topicMapId: 'number-quantum-as-number',
    lineRange: { start: 4, end: 52 },
    section: 'A. NUMBER',
    order: 1,
  },
  description: NUMBER_TOPIC_MAP.entries[0]?.description,
  keyPoints: NUMBER_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'number-amount-in-limit',
  title: 'Amount in the limit: delimitation by many ones',
  concept: 'AmountUnitDelimitation',
  phase: 'quantity',
  moments: [
    {
      name: 'amountAndUnit',
      definition:
        'Number divides into amount as how-many and unit as continuity of that amount',
      type: 'determination',
    },
    {
      name: 'manyOnesAsLimit',
      definition:
        'The many ones are present and equal, and constitute delimitation itself',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'amountAndUnit',
    },
  ],
  invariants: [
    {
      id: 'number-amount-in-limit-inv-1',
      constraint: 'amount and unit are inseparable moments of number',
      predicate: 'inseparable(amount, unit)',
    },
    {
      id: 'number-amount-in-limit-inv-2',
      constraint: 'the many ones delimit from within, not as external residue',
      predicate: 'internalDelimitation(manyOnes)',
    },
  ],
  forces: [
    {
      id: 'number-amount-in-limit-force-1',
      description:
        'Internal delimitation reveals the contradiction of indifference and exteriority in number',
      type: 'contradiction',
      trigger: 'delimitation.internalizesExternality = true',
      effect: 'numberContradiction.explicit = true',
      targetState: 'number-indifference-and-exteriority',
    },
  ],
  transitions: [
    {
      id: 'number-amount-in-limit-trans-1',
      from: 'number-amount-in-limit',
      to: 'number-indifference-and-exteriority',
      mechanism: 'contradiction',
      description: 'From delimitation to indifference and exteriority',
    },
  ],
  nextStates: ['number-indifference-and-exteriority'],
  previousStates: ['number-quantum-as-number'],
  provenance: {
    topicMapId: 'number-amount-in-limit',
    lineRange: { start: 67, end: 109 },
    section: 'A. NUMBER',
    order: 3,
  },
  description: NUMBER_TOPIC_MAP.entries[2]?.description,
  keyPoints: NUMBER_TOPIC_MAP.entries[2]?.keyPoints,
};

const state3: DialecticState = {
  id: 'number-indifference-and-exteriority',
  title: 'Number contradiction: indifference and exteriority',
  concept: 'NumberContradiction',
  phase: 'quantity',
  moments: [
    {
      name: 'indifference',
      definition:
        'Number is determined in itself yet indifferent in relation to other numbers',
      type: 'quality',
    },
    {
      name: 'exteriorityInOne',
      definition:
        'External reference is not merely outside but lodged in the one itself',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'indifference',
    },
  ],
  invariants: [
    {
      id: 'number-indifference-and-exteriority-inv-1',
      constraint:
        'number keeps absolute determinateness only as external reference',
      predicate: 'determinatenessThroughExternality(number)',
    },
    {
      id: 'number-indifference-and-exteriority-inv-2',
      constraint: 'the contradiction of number is the quality of quantum',
      predicate: 'qualityOfQuantum(numberContradiction)',
    },
  ],
  forces: [
    {
      id: 'number-indifference-and-exteriority-force-1',
      description:
        'Contradiction of number drives the transition to extensive and intensive quantum',
      type: 'passover',
      trigger: 'numberContradiction.develops = true',
      effect: 'quantumDifferentiation.initiated = true',
      targetState: 'quantum-extensive-magnitude',
    },
  ],
  transitions: [
    {
      id: 'number-indifference-and-exteriority-trans-1',
      from: 'number-indifference-and-exteriority',
      to: 'quantum-extensive-magnitude',
      mechanism: 'passover',
      description: 'From number contradiction to quantum differentiation',
    },
  ],
  nextStates: ['quantum-extensive-magnitude'],
  previousStates: ['number-amount-in-limit'],
  provenance: {
    topicMapId: 'number-indifference-and-exteriority',
    lineRange: { start: 111, end: 136 },
    section: 'A. NUMBER',
    order: 4,
  },
  description: NUMBER_TOPIC_MAP.entries[3]?.description,
  keyPoints: NUMBER_TOPIC_MAP.entries[3]?.keyPoints,
};

export const numberIR: DialecticIR = {
  id: 'number-ir',
  title: 'Number IR: Quantum as Number',
  section: 'BEING - QUANTITY - A. Number',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'number.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'number-quantum-as-number': 'quantity',
      'number-amount-in-limit': 'quantity',
      'number-indifference-and-exteriority': 'quantity',
    },
  },
};

export const numberStates = {
  'number-quantum-as-number': state1,
  'number-amount-in-limit': state2,
  'number-indifference-and-exteriority': state3,
};
