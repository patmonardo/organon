import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { QUANTUM_TOPIC_MAP } from './sources/quantum-topic-map';

const state1: DialecticState = {
  id: 'quantum-extensive-magnitude',
  title: 'Extensive magnitude: determinateness as amount',
  concept: 'ExtensiveMagnitude',
  phase: 'quantity',
  moments: [
    {
      name: 'limitAsPlurality',
      definition:
        'Quantum has its determinateness as limit in amount and explicit plurality',
      type: 'determination',
    },
    {
      name: 'amountAsOneUnitMany',
      definition:
        'Extensive quantum is amount as many ones gathered in one delimiting form',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'limitAsPlurality',
    },
  ],
  invariants: [
    {
      id: 'quantum-extensive-magnitude-inv-1',
      constraint: 'extensive quantum makes plurality explicit in the limit',
      predicate: 'explicitPlurality(extensiveQuantum)',
    },
    {
      id: 'quantum-extensive-magnitude-inv-2',
      constraint: 'extensive and intensive belong to one quantitative limit',
      predicate: 'sameLimit(extensiveMagnitude, intensiveMagnitude)',
    },
  ],
  forces: [
    {
      id: 'quantum-extensive-magnitude-force-1',
      description:
        'Plural limit reflected into itself posits degree as simple determinateness',
      type: 'mediation',
      trigger: 'pluralLimit.turnsInward = true',
      effect: 'degree.explicit = true',
      targetState: 'quantum-intensive-magnitude-degree',
    },
  ],
  transitions: [
    {
      id: 'quantum-extensive-magnitude-trans-1',
      from: 'quantum-extensive-magnitude',
      to: 'quantum-intensive-magnitude-degree',
      mechanism: 'mediation',
      description: 'From extensive amount to intensive degree',
    },
  ],
  nextStates: ['quantum-intensive-magnitude-degree'],
  previousStates: ['number-indifference-and-exteriority'],
  provenance: {
    topicMapId: 'quantum-extensive-magnitude',
    lineRange: { start: 6, end: 54 },
    section: 'a. Their difference',
    order: 1,
  },
  description: QUANTUM_TOPIC_MAP.entries[0]?.description,
  keyPoints: QUANTUM_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'quantum-intensive-magnitude-degree',
  title: 'Degree: intensive magnitude as simple determinateness',
  concept: 'IntensiveMagnitudeDegree',
  phase: 'quantity',
  moments: [
    {
      name: 'simpleDeterminateness',
      definition:
        'Degree is quantum as non-aggregate simplicity, with plurality sublated into one determination',
      type: 'sublation',
    },
    {
      name: 'externalityInDegree',
      definition:
        'Degree remains tied to externality through scale and comparative plurality',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'simpleDeterminateness',
    },
  ],
  invariants: [
    {
      id: 'quantum-intensive-magnitude-degree-inv-1',
      constraint: 'degree is intensive quantum, not aggregate amount',
      predicate: 'intensiveNotAggregate(degree)',
    },
    {
      id: 'quantum-intensive-magnitude-degree-inv-2',
      constraint:
        'degree remains quantitatively comparable through externality',
      predicate: 'comparableByExternality(degree)',
    },
  ],
  forces: [
    {
      id: 'quantum-intensive-magnitude-degree-force-1',
      description:
        'Degree reveals itself as identical with extensive determinateness',
      type: 'reflection',
      trigger: 'degree.requiresExternalScale = true',
      effect: 'identityOfSides.explicit = true',
      targetState: 'quantum-identity-two-sides',
    },
  ],
  transitions: [
    {
      id: 'quantum-intensive-magnitude-degree-trans-1',
      from: 'quantum-intensive-magnitude-degree',
      to: 'quantum-identity-two-sides',
      mechanism: 'reflection',
      description: 'From degree to identity of extensive and intensive',
    },
  ],
  nextStates: ['quantum-identity-two-sides'],
  previousStates: ['quantum-extensive-magnitude'],
  provenance: {
    topicMapId: 'quantum-intensive-magnitude-degree',
    lineRange: { start: 88, end: 117 },
    section: 'a. Their difference',
    order: 3,
  },
  description: QUANTUM_TOPIC_MAP.entries[2]?.description,
  keyPoints: QUANTUM_TOPIC_MAP.entries[2]?.keyPoints,
};

const state3: DialecticState = {
  id: 'quantum-identity-two-sides',
  title: 'Identity of sides and necessity of alteration',
  concept: 'ExtensiveIntensiveIdentity',
  phase: 'quantity',
  moments: [
    {
      name: 'identityOfDeterminateness',
      definition:
        'Extensive and intensive are the same quantitative determinateness under different placements of amount',
      type: 'determination',
    },
    {
      name: 'necessaryAlteration',
      definition:
        'As self-negating indifferent limit, quantum must alter and pass beyond itself',
      type: 'negation',
      relation: 'transitions',
      relatedTo: 'identityOfDeterminateness',
    },
  ],
  invariants: [
    {
      id: 'quantum-identity-two-sides-inv-1',
      constraint:
        'difference of extensive and intensive is only formal placement',
      predicate: 'formalDifference(extensive, intensive)',
    },
    {
      id: 'quantum-identity-two-sides-inv-2',
      constraint: 'quantum as indifferent limit is self-altering',
      predicate: 'mustAlter(quantum)',
    },
  ],
  forces: [
    {
      id: 'quantum-identity-two-sides-force-1',
      description:
        'Necessity of alteration pushes quantum into the quantitative infinite',
      type: 'passover',
      trigger: 'quantum.mustAlter = true',
      effect: 'quantitativeInfinity.initiated = true',
      targetState: 'infinity-concept-self-contradictory',
    },
  ],
  transitions: [
    {
      id: 'quantum-identity-two-sides-trans-1',
      from: 'quantum-identity-two-sides',
      to: 'infinity-concept-self-contradictory',
      mechanism: 'passover',
      description:
        'From identity and alteration to the concept of quantitative infinity',
    },
  ],
  nextStates: ['infinity-concept-self-contradictory'],
  previousStates: ['quantum-intensive-magnitude-degree'],
  provenance: {
    topicMapId: 'quantum-identity-two-sides',
    lineRange: { start: 199, end: 254 },
    section: 'b. Identity of extensive and intensive magnitude',
    order: 6,
  },
  description: QUANTUM_TOPIC_MAP.entries[5]?.description,
  keyPoints: QUANTUM_TOPIC_MAP.entries[5]?.keyPoints,
};

export const quantumIR: DialecticIR = {
  id: 'quantum-ir',
  title: 'Quantum IR: Extensive and Intensive Quantum',
  section: 'BEING - QUANTITY - C. Quantum',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'quantum.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'quantum-extensive-magnitude': 'quantity',
      'quantum-intensive-magnitude-degree': 'quantity',
      'quantum-identity-two-sides': 'quantity',
    },
  },
};

export const quantumStates = {
  'quantum-extensive-magnitude': state1,
  'quantum-intensive-magnitude-degree': state2,
  'quantum-identity-two-sides': state3,
};
