import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { SINGULAR_TOPIC_MAP } from './sources/singular-topic-map';

const state1: DialecticState = {
  id: 'sing-1',
  title: 'Singularity as determinate universality',
  concept: 'SingularityPosited',
  phase: 'subject',
  moments: [
    {
      name: 'singularity',
      definition:
        'Self-referring determinateness posited through particularity',
      type: 'determination',
    },
    {
      name: 'determinateDeterminate',
      definition:
        'The determinate is posited as the determinate determinateness',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'singularity',
    },
  ],
  invariants: [
    {
      id: 'sing-1-inv-1',
      constraint: 'singularity is determinate universality',
      predicate: 'equals(singularity, determinateUniversality)',
    },
    {
      id: 'sing-1-inv-2',
      constraint: 'singularity is self-referring determinateness',
      predicate: 'selfReferring(singularity)',
    },
  ],
  forces: [
    {
      id: 'sing-1-force-1',
      description: 'Singularity reflects concept back into itself',
      type: 'reflection',
      trigger: 'singularity.posited = true',
      effect: 'selfMediation.emerges = true',
      targetState: 'sing-5',
    },
  ],
  transitions: [
    {
      id: 'sing-1-trans-1',
      from: 'sing-1',
      to: 'sing-5',
      mechanism: 'reflection',
      description: 'From posited singularity to indissoluble conceptual unity',
    },
  ],
  nextStates: ['sing-5'],
  previousStates: ['part-12'],
  provenance: {
    topicMapId: 'sing-1-posited-through-particularity',
    lineRange: { start: 4, end: 8 },
    section: 'The Singular Concept',
    order: 1,
  },
  description: SINGULAR_TOPIC_MAP.entries[0]?.description,
  keyPoints: SINGULAR_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'sing-5',
  title: 'Indissoluble unity of concept and abstraction products',
  concept: 'IndissolubleUnity',
  phase: 'subject',
  moments: [
    {
      name: 'indissolubleUnity',
      definition:
        'Conceptual unity remains in products abstraction claims to isolate',
      type: 'determination',
    },
    {
      name: 'abstractionReturnsToSingularity',
      definition:
        'Abstraction grasps universality only as determinate universality, hence singular',
      type: 'sublation',
      relation: 'contains',
      relatedTo: 'indissolubleUnity',
    },
  ],
  invariants: [
    {
      id: 'sing-5-inv-1',
      constraint: 'products of abstraction are singulars',
      predicate: 'equals(productsOfAbstraction, singulars)',
    },
    {
      id: 'sing-5-inv-2',
      constraint: 'conceptual singularity is totality of form',
      predicate: 'equals(conceptualSingularity, totalityOfForm)',
    },
  ],
  forces: [
    {
      id: 'sing-5-force-1',
      description: 'Dissolution of rigid determinations posits inseparability',
      type: 'sublation',
      trigger: 'moments.rigidity = dissolved',
      effect: 'inseparability.emerges = true',
      targetState: 'sing-8',
    },
  ],
  transitions: [
    {
      id: 'sing-5-trans-1',
      from: 'sing-5',
      to: 'sing-8',
      mechanism: 'sublation',
      description: 'From indissoluble unity to inseparable conceptual totality',
    },
  ],
  nextStates: ['sing-8'],
  previousStates: ['sing-1'],
  provenance: {
    topicMapId: 'sing-5-unity-indissoluble',
    lineRange: { start: 75, end: 101 },
    section: 'The Singular Concept',
    order: 5,
  },
  description: SINGULAR_TOPIC_MAP.entries[4]?.description,
  keyPoints: SINGULAR_TOPIC_MAP.entries[4]?.keyPoints,
};

const state3: DialecticState = {
  id: 'sing-13',
  title: 'Concept posited as judgment',
  concept: 'JudgmentTransition',
  phase: 'subject',
  moments: [
    {
      name: 'absolutePartition',
      definition:
        'Concept partitions itself originatively in singular determinateness',
      type: 'process',
    },
    {
      name: 'selfSubsistingDeterminations',
      definition:
        'Determinations stand as self-subsisting and concept is posited as judgment',
      type: 'determination',
      relation: 'transitions',
      relatedTo: 'absolutePartition',
    },
  ],
  invariants: [
    {
      id: 'sing-13-inv-1',
      constraint: 'determinate has become totality',
      predicate: 'equals(determinate, totality)',
    },
    {
      id: 'sing-13-inv-2',
      constraint: 'turning back is posited as judgment',
      predicate: 'equals(turningBack(concept), judgment)',
    },
  ],
  forces: [
    {
      id: 'sing-13-force-1',
      description: 'Originative partition passes over to judgment of existence',
      type: 'passover',
      trigger: 'concept.partitioned = true',
      effect: 'judgmentOfExistence.emerges = true',
      targetState: 'exist-1',
    },
  ],
  transitions: [
    {
      id: 'sing-13-trans-1',
      from: 'sing-13',
      to: 'exist-1',
      mechanism: 'passover',
      description: 'From singular concept to judgment of existence',
    },
  ],
  nextStates: ['exist-1'],
  previousStates: ['sing-8'],
  provenance: {
    topicMapId: 'sing-13-posited-as-judgment',
    lineRange: { start: 268, end: 283 },
    section: 'The Singular Concept',
    order: 13,
  },
  description: SINGULAR_TOPIC_MAP.entries[12]?.description,
  keyPoints: SINGULAR_TOPIC_MAP.entries[12]?.keyPoints,
};

export const singularIR: DialecticIR = {
  id: 'singular-ir',
  title:
    'Singular IR: Posited Singularity, Indissoluble Unity, Judgment Transition',
  section: 'CONCEPT - SUBJECTIVITY - A. The Concept - 3. The Singular',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'singular.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'sing-1': 'subject',
      'sing-5': 'subject',
      'sing-13': 'subject',
    },
  },
};

export const singularStates = {
  'sing-1': state1,
  'sing-5': state2,
  'sing-13': state3,
};
