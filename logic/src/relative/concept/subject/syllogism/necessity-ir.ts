import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { NECESSITY_SYLLOGISM_TOPIC_MAP } from './sources/necessity-topic-map';

const state1: DialecticState = {
  id: 'syl-nec-1',
  title: 'Categorical syllogism with objective universality as middle',
  concept: 'CategoricalNecessitySyllogism',
  phase: 'subject',
  moments: [
    {
      name: 'objectiveUniversality',
      definition: 'Middle term is genus as completed determinate universality',
      type: 'determination',
    },
    {
      name: 'substantialConnection',
      definition:
        'Extremes are connected through substantial identity as first objective form',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'objectiveUniversality',
    },
  ],
  invariants: [
    {
      id: 'syl-nec-1-inv-1',
      constraint: 'objectivity begins in categorical syllogism',
      predicate: 'begins(objectivity, categoricalSyllogism)',
    },
    {
      id: 'syl-nec-1-inv-2',
      constraint: 'middle is concrete genus not accidental property',
      predicate: 'and(equals(middle, genus), not(accidental(middle)))',
    },
  ],
  forces: [
    {
      id: 'syl-nec-1-force-1',
      description:
        'Substantial identity formalizes itself as hypothetical mediation',
      type: 'mediation',
      trigger: 'identity.formStillInner = true',
      effect: 'hypotheticalSyllogism.emerges = true',
      targetState: 'syl-nec-5',
    },
  ],
  transitions: [
    {
      id: 'syl-nec-1-trans-1',
      from: 'syl-nec-1',
      to: 'syl-nec-5',
      mechanism: 'mediation',
      description: 'From categorical to hypothetical syllogism of necessity',
    },
  ],
  nextStates: ['syl-nec-5'],
  previousStates: ['syl-refl-16'],
  provenance: {
    topicMapId: 'syl-nec-1-introduction-objective',
    lineRange: { start: 5, end: 39 },
    section: 'The Syllogism of Necessity',
    order: 1,
  },
  description: NECESSITY_SYLLOGISM_TOPIC_MAP.entries[0]?.description,
  keyPoints: NECESSITY_SYLLOGISM_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'syl-nec-5',
  title: 'Hypothetical syllogism as negative unity of mediation',
  concept: 'HypotheticalNecessitySyllogism',
  phase: 'subject',
  moments: [
    {
      name: 'hypotheticalConnection',
      definition:
        'Necessary connection appears in the form if-A-then-B with immediacy added',
      type: 'determination',
    },
    {
      name: 'identityOfMediatorAndMediated',
      definition:
        'Mediator and mediated share identical content in self-referring negativity',
      type: 'reflection',
      relation: 'contains',
      relatedTo: 'hypotheticalConnection',
    },
  ],
  invariants: [
    {
      id: 'syl-nec-5-inv-1',
      constraint: 'being of one side is being of the other',
      predicate: 'equals(being(A), being(B))',
    },
    {
      id: 'syl-nec-5-inv-2',
      constraint: 'mediation is singularizing negative unity',
      predicate: 'equals(mediation, selfReferringNegativity)',
    },
  ],
  forces: [
    {
      id: 'syl-nec-5-force-1',
      description: 'Negative unity develops into disjunctive totality',
      type: 'passover',
      trigger: 'identityOfMediatorAndMediated.explicit = true',
      effect: 'disjunctiveTotality.emerges = true',
      targetState: 'syl-nec-11',
    },
  ],
  transitions: [
    {
      id: 'syl-nec-5-trans-1',
      from: 'syl-nec-5',
      to: 'syl-nec-11',
      mechanism: 'passover',
      description:
        'From hypothetical mediation to disjunctive completion and bridge',
    },
  ],
  nextStates: ['syl-nec-11'],
  previousStates: ['syl-nec-1'],
  provenance: {
    topicMapId: 'syl-nec-5-hypothetical-immediacy',
    lineRange: { start: 204, end: 226 },
    section: 'The Hypothetical Syllogism',
    order: 5,
  },
  description: NECESSITY_SYLLOGISM_TOPIC_MAP.entries[4]?.description,
  keyPoints: NECESSITY_SYLLOGISM_TOPIC_MAP.entries[4]?.keyPoints,
};

const state3: DialecticState = {
  id: 'syl-nec-11',
  title: 'Syllogism terminal bridge: concept realized as objectivity',
  concept: 'ObjectivityBridgeFromSyllogism',
  phase: 'subject',
  moments: [
    {
      name: 'disjunctiveCompletion',
      definition:
        'Middle as totality contains whole form-determinateness; formalism is sublated',
      type: 'sublation',
    },
    {
      name: 'ideaBridge',
      definition:
        'Concept restores immediacy through mediation and passes into differentiated objectivity',
      type: 'passover',
      relation: 'transitions',
      relatedTo: 'disjunctiveCompletion',
    },
  ],
  invariants: [
    {
      id: 'syl-nec-11-inv-1',
      constraint: 'concept is realized as objectivity',
      predicate: 'equals(concept.realized, objectivity)',
    },
    {
      id: 'syl-nec-11-inv-2',
      constraint: 'each syllogistic moment is totality only through the others',
      predicate: 'forall(moment, throughOthers(moment, totality))',
    },
  ],
  forces: [
    {
      id: 'syl-nec-11-force-1',
      description:
        'Realized concept hands off to mechanical object as first objective articulation',
      type: 'passover',
      trigger: 'objectivity.bridgeReady = true',
      effect: 'mechanicalObject.emerges = true',
      targetState: 'mech-obj-1',
    },
  ],
  transitions: [
    {
      id: 'syl-nec-11-trans-1',
      from: 'syl-nec-11',
      to: 'mech-obj-1',
      mechanism: 'passover',
      description: 'From syllogism of necessity to mechanical objectivity',
    },
  ],
  nextStates: ['mech-obj-1'],
  previousStates: ['syl-nec-5'],
  provenance: {
    topicMapId: 'syl-nec-11-review-objectivity',
    lineRange: { start: 475, end: 538 },
    section: 'The Disjunctive Syllogism',
    order: 11,
  },
  description: NECESSITY_SYLLOGISM_TOPIC_MAP.entries[10]?.description,
  keyPoints: NECESSITY_SYLLOGISM_TOPIC_MAP.entries[10]?.keyPoints,
};

export const necessitySyllogismIR: DialecticIR = {
  id: 'necessity-syllogism-ir',
  title:
    'Necessity Syllogism IR (Bridge): Categorical, Hypothetical, Objectivity Handoff',
  section:
    'CONCEPT - SUBJECTIVITY - C. Syllogism - C. Syllogism of Necessity (Objectivity Bridge)',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'necessity.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'syl-nec-1': 'subject',
      'syl-nec-5': 'subject',
      'syl-nec-11': 'subject',
    },
  },
};

export const necessitySyllogismStates = {
  'syl-nec-1': state1,
  'syl-nec-5': state2,
  'syl-nec-11': state3,
};
