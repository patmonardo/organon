import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { realizedTopicMap } from './souces/realized-topic-map';

const state1: DialecticState = {
  id: 'realized-2',
  title: 'Mechanism under dominance and cunning of reason',
  concept: 'MechanismUnderPurpose',
  phase: 'object',
  moments: [
    {
      name: 'purposeInMeansActivity',
      definition:
        'Purpose operates through means so mechanism/chemism run under purposive dominance',
      type: 'mediation',
    },
    {
      name: 'cunningOfReason',
      definition:
        'Purpose preserves itself by letting means bear external wear and mechanical violence',
      type: 'reflection',
      relation: 'contains',
      relatedTo: 'purposeInMeansActivity',
    },
  ],
  invariants: [
    {
      id: 'realized-2-inv-1',
      constraint: 'mechanical process returns to purpose as true middle',
      predicate: 'returnsToPurpose(mechanicalProcess)',
    },
    {
      id: 'realized-2-inv-2',
      constraint: 'means manifests higher durability than finite enjoyments',
      predicate: 'moreDurable(means, finiteEnjoyments)',
    },
  ],
  forces: [
    {
      id: 'realized-2-force-1',
      description:
        'Dominated mechanism reveals limit of external purposiveness',
      type: 'negation',
      trigger: 'product.remainsExternal = true',
      effect: 'externalPurposiveness.emerges = true',
      targetState: 'realized-10',
    },
  ],
  transitions: [
    {
      id: 'realized-2-trans-1',
      from: 'realized-2',
      to: 'realized-10',
      mechanism: 'negation',
      description: 'From dominated mechanism to external purposiveness',
    },
  ],
  nextStates: ['realized-10'],
  previousStates: ['means-6'],
  provenance: {
    topicMapId: 'realized-2',
    lineRange: { start: 27, end: 151 },
    section: 'C. THE REALIZED PURPOSE',
    order: 1,
  },
  description: realizedTopicMap[1]?.description,
  keyPoints: realizedTopicMap[1]?.keyPoints,
};

const state2: DialecticState = {
  id: 'realized-10',
  title: 'External purposiveness remains means-only',
  concept: 'ExternalPurposivenessLimit',
  phase: 'object',
  moments: [
    {
      name: 'infiniteMeansProgress',
      definition:
        'If taken externally, purposive production repeats as means and sets infinite mediation progress',
      type: 'negation',
    },
    {
      name: 'perishableObjectiveForm',
      definition:
        'Objects fulfill such purposes only by being used up, showing external purposiveness is merely relative',
      type: 'contradiction',
      relation: 'contains',
      relatedTo: 'infiniteMeansProgress',
    },
  ],
  invariants: [
    {
      id: 'realized-10-inv-1',
      constraint: 'external purposiveness does not yet yield objective purpose',
      predicate: 'not(yieldsObjectivePurpose(externalPurposiveness))',
    },
    {
      id: 'realized-10-inv-2',
      constraint: 'means/product equivalence persists under externality',
      predicate: 'equivalentUnderExternality(product, means)',
    },
  ],
  forces: [
    {
      id: 'realized-10-force-1',
      description: 'Externality collapses into inner purposive mediation',
      type: 'sublation',
      trigger: 'externalMediation.selfContradiction = explicit',
      effect: 'innerPurposiveIdentity.emerges = true',
      targetState: 'realized-15',
    },
  ],
  transitions: [
    {
      id: 'realized-10-trans-1',
      from: 'realized-10',
      to: 'realized-15',
      mechanism: 'sublation',
      description: 'From external purposiveness to realized purposive identity',
    },
  ],
  nextStates: ['realized-15'],
  previousStates: ['realized-2'],
  provenance: {
    topicMapId: 'realized-10',
    lineRange: { start: 320, end: 397 },
    section: 'C. THE REALIZED PURPOSE',
    order: 2,
  },
  description: realizedTopicMap[9]?.description,
  keyPoints: realizedTopicMap[9]?.keyPoints,
};

const state3: DialecticState = {
  id: 'realized-15',
  title: 'Concept identical with immediate objectivity',
  concept: 'TransitionToIdea',
  phase: 'object',
  moments: [
    {
      name: 'innerPurposiveIdentity',
      definition:
        'The concept attains identity with immediate objectivity as concrete purposive totality',
      type: 'sublation',
    },
    {
      name: 'mediatedImmediacy',
      definition: 'This identity is immediate only as self-sublating mediation',
      type: 'passover',
      relation: 'transitions',
      relatedTo: 'innerPurposiveIdentity',
    },
  ],
  invariants: [
    {
      id: 'realized-15-inv-1',
      constraint: 'purpose is no longer mere ought but objective identity',
      predicate: 'objectiveIdentity(purpose)',
    },
    {
      id: 'realized-15-inv-2',
      constraint:
        'concept distinguishes itself only to determine externality as its own',
      predicate: 'selfDeterminesExternality(concept)',
    },
  ],
  forces: [
    {
      id: 'realized-15-force-1',
      description: 'Realized purposive identity passes into the Idea',
      type: 'passover',
      trigger: 'concept.objectiveIdentity = complete',
      effect: 'idea.emerges = true',
      targetState: 'idea-1',
    },
  ],
  transitions: [
    {
      id: 'realized-15-trans-1',
      from: 'realized-15',
      to: 'idea-1',
      mechanism: 'passover',
      description: 'From realized purpose to Idea',
    },
  ],
  nextStates: ['idea-1'],
  previousStates: ['realized-10'],
  provenance: {
    topicMapId: 'realized-15',
    lineRange: { start: 578, end: 606 },
    section: 'C. THE REALIZED PURPOSE',
    order: 3,
  },
  description: realizedTopicMap[14]?.description,
  keyPoints: realizedTopicMap[14]?.keyPoints,
};

export const realizedPurposeIR: DialecticIR = {
  id: 'realized-purpose-ir',
  title:
    'Realized Purpose IR: Dominated Mechanism, External Limit, Objective Identity',
  section: 'CONCEPT - OBJECTIVITY - C. Teleology - C. The Realized Purpose',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'realized.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'realized-2': 'object',
      'realized-10': 'object',
      'realized-15': 'object',
    },
  },
};

export const realizedPurposeStates = {
  'realized-2': state1,
  'realized-10': state2,
  'realized-15': state3,
};
