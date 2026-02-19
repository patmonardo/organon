import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { EXISTENCE_SYLLOGISM_TOPIC_MAP } from './sources/existence-topic-map';

const state1: DialecticState = {
  id: 'syl-exist-1',
  title: 'Immediate formal syllogism S-P-U',
  concept: 'ImmediateFormalSyllogism',
  phase: 'subject',
  moments: [
    {
      name: 'particularMiddle',
      definition:
        'Particularity is immediate middle connecting singular and universal',
      type: 'determination',
    },
    {
      name: 'formalContingency',
      definition:
        'As merely formal arrangement, mediation remains contingent and externally arranged',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'particularMiddle',
    },
  ],
  invariants: [
    {
      id: 'syl-exist-1-inv-1',
      constraint: 'first schema is S-P-U',
      predicate: 'equals(schema, SPU)',
    },
    {
      id: 'syl-exist-1-inv-2',
      constraint: 'formal syllogism requires further mediating development',
      predicate: 'requiresFurtherMediation(formalSyllogism)',
    },
  ],
  forces: [
    {
      id: 'syl-exist-1-force-1',
      description:
        'Contingent formalism drives to negative mediation and inversion of figures',
      type: 'negation',
      trigger: 'middle.formal = true',
      effect: 'figureDevelopment.emerges = true',
      targetState: 'syl-exist-14',
    },
  ],
  transitions: [
    {
      id: 'syl-exist-1-trans-1',
      from: 'syl-exist-1',
      to: 'syl-exist-14',
      mechanism: 'negation',
      description: 'From first immediate figure to developed figure movement',
    },
  ],
  nextStates: ['syl-exist-14'],
  previousStates: ['conc-14'],
  provenance: {
    topicMapId: 'syl-exist-1-introduction-immediate',
    lineRange: { start: 4, end: 36 },
    section: 'The Syllogism of Existence',
    order: 1,
  },
  description: EXISTENCE_SYLLOGISM_TOPIC_MAP.entries[0]?.description,
  keyPoints: EXISTENCE_SYLLOGISM_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'syl-exist-14',
  title: 'Figure development and reciprocal presupposition',
  concept: 'FigureDevelopment',
  phase: 'subject',
  moments: [
    {
      name: 'negativeMediation',
      definition:
        'Second and third figures expose mediation as negative and reciprocally presupposing',
      type: 'mediation',
    },
    {
      name: 'formalTruthCrisis',
      definition:
        'No single figure secures total conceptual mediation on its own',
      type: 'contradiction',
      relation: 'contains',
      relatedTo: 'negativeMediation',
    },
  ],
  invariants: [
    {
      id: 'syl-exist-14-inv-1',
      constraint:
        'figure transitions expose insufficiency of isolated middle-term fixation',
      predicate: 'insufficient(isolatedMiddleFixation)',
    },
    {
      id: 'syl-exist-14-inv-2',
      constraint:
        'mediation must become self-referring rather than externally ordered',
      predicate: 'mustBecome(mediation, selfReferring)',
    },
  ],
  forces: [
    {
      id: 'syl-exist-14-force-1',
      description:
        'Reciprocal presupposition resolves toward reflected mediation',
      type: 'sublation',
      trigger: 'figures.reciprocallyPresuppose = true',
      effect: 'reflectionMediation.emerges = true',
      targetState: 'syl-exist-25',
    },
  ],
  transitions: [
    {
      id: 'syl-exist-14-trans-1',
      from: 'syl-exist-14',
      to: 'syl-exist-25',
      mechanism: 'sublation',
      description: 'From figure contradiction to reflected mediation',
    },
  ],
  nextStates: ['syl-exist-25'],
  previousStates: ['syl-exist-1'],
  provenance: {
    topicMapId: 'syl-exist-14-second-figure',
    lineRange: { start: 486, end: 670 },
    section: 'Second Figure',
    order: 14,
  },
  description: EXISTENCE_SYLLOGISM_TOPIC_MAP.entries[13]?.description,
  keyPoints: EXISTENCE_SYLLOGISM_TOPIC_MAP.entries[13]?.keyPoints,
};

const state3: DialecticState = {
  id: 'syl-exist-25',
  title: 'Existence syllogism passes to reflection syllogism',
  concept: 'TransitionToReflectionSyllogism',
  phase: 'subject',
  moments: [
    {
      name: 'mediationThroughConcreteIdentity',
      definition:
        'Mediation is grounded in concrete identity of determinacies, not isolated qualitative middle',
      type: 'reflection',
    },
    {
      name: 'selfReferringMediation',
      definition:
        'Mediation becomes mediation-of-mediation, a turning-back totality',
      type: 'passover',
      relation: 'transitions',
      relatedTo: 'mediationThroughConcreteIdentity',
    },
  ],
  invariants: [
    {
      id: 'syl-exist-25-inv-1',
      constraint: 'formalism of single middle determinateness is sublated',
      predicate: 'sublated(singleMiddleFormalism)',
    },
    {
      id: 'syl-exist-25-inv-2',
      constraint: 'result is positive reflection totality',
      predicate: 'equals(result, positiveReflectionTotality)',
    },
  ],
  forces: [
    {
      id: 'syl-exist-25-force-1',
      description:
        'Self-referring mediation hands off to syllogism of reflection',
      type: 'passover',
      trigger: 'reflectionMediation.totalized = true',
      effect: 'reflectionSyllogism.emerges = true',
      targetState: 'syl-refl-1',
    },
  ],
  transitions: [
    {
      id: 'syl-exist-25-trans-1',
      from: 'syl-exist-25',
      to: 'syl-refl-1',
      mechanism: 'passover',
      description: 'From syllogism of existence to syllogism of reflection',
    },
  ],
  nextStates: ['syl-refl-1'],
  previousStates: ['syl-exist-14'],
  provenance: {
    topicMapId: 'syl-exist-25-each-determination',
    lineRange: { start: 871, end: 896 },
    section: 'Fourth Figure',
    order: 25,
  },
  description: EXISTENCE_SYLLOGISM_TOPIC_MAP.entries[24]?.description,
  keyPoints: EXISTENCE_SYLLOGISM_TOPIC_MAP.entries[24]?.keyPoints,
};

export const existenceSyllogismIR: DialecticIR = {
  id: 'existence-syllogism-ir',
  title:
    'Existence Syllogism IR: Immediate Form, Figure Development, Reflection Handoff',
  section: 'CONCEPT - SUBJECTIVITY - C. Syllogism - A. Syllogism of Existence',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'existence.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'syl-exist-1': 'subject',
      'syl-exist-14': 'subject',
      'syl-exist-25': 'subject',
    },
  },
};

export const existenceSyllogismStates = {
  'syl-exist-1': state1,
  'syl-exist-14': state2,
  'syl-exist-25': state3,
};
