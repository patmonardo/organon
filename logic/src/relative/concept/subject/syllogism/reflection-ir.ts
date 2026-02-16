import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { REFLECTION_SYLLOGISM_TOPIC_MAP } from './sources/reflection-topic-map';

const state1: DialecticState = {
  id: 'syl-refl-1',
  title: 'Allness as externally reflected universality',
  concept: 'SyllogismOfAllness',
  phase: 'subject',
  moments: [
    {
      name: 'allnessMiddle',
      definition:
        "Middle term appears as allness, understanding's perfected but external universality",
      type: 'determination',
    },
    {
      name: 'presuppositionalIllusion',
      definition: 'Major premise presupposes what conclusion should establish',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'allnessMiddle',
    },
  ],
  invariants: [
    {
      id: 'syl-refl-1-inv-1',
      constraint: 'allness remains external universality of reflection',
      predicate: 'equals(allness, externalUniversalityReflection)',
    },
    {
      id: 'syl-refl-1-inv-2',
      constraint: 'middle still relies on subjective singular basis',
      predicate: 'reliesOn(middle, subjectiveSingularity)',
    },
  ],
  forces: [
    {
      id: 'syl-refl-1-force-1',
      description:
        'Presuppositional form drives to inductive completion attempt',
      type: 'mediation',
      trigger: 'allness.illusionExposed = true',
      effect: 'induction.emerges = true',
      targetState: 'syl-refl-8',
    },
  ],
  transitions: [
    {
      id: 'syl-refl-1-trans-1',
      from: 'syl-refl-1',
      to: 'syl-refl-8',
      mechanism: 'mediation',
      description: 'From allness to induction',
    },
  ],
  nextStates: ['syl-refl-8'],
  previousStates: ['syl-exist-25'],
  provenance: {
    topicMapId: 'syl-refl-1-introduction-sublated',
    lineRange: { start: 4, end: 30 },
    section: 'The Syllogism of Reflection',
    order: 1,
  },
  description: REFLECTION_SYLLOGISM_TOPIC_MAP.entries[0]?.description,
  keyPoints: REFLECTION_SYLLOGISM_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'syl-refl-8',
  title: 'Induction and analogy as reflective completion',
  concept: 'InductionAnalogyMovement',
  phase: 'subject',
  moments: [
    {
      name: 'inductiveCompletion',
      definition:
        'Induction gathers singulars under universal but remains task-like and infinite',
      type: 'determination',
    },
    {
      name: 'analogicalEssentiality',
      definition:
        'Analogy raises middle toward essential universality while retaining reflective externality',
      type: 'reflection',
      relation: 'transforms',
      relatedTo: 'inductiveCompletion',
    },
  ],
  invariants: [
    {
      id: 'syl-refl-8-inv-1',
      constraint: 'induction presupposes genus/conclusion as in-itself true',
      predicate: 'presupposes(induction, genusConclusionTruth)',
    },
    {
      id: 'syl-refl-8-inv-2',
      constraint:
        'analogy expresses essential universality not mere empirical mark',
      predicate: 'not(mereEmpiricalMark(analogyMiddle))',
    },
  ],
  forces: [
    {
      id: 'syl-refl-8-force-1',
      description: 'Analogical form demands sublation of singular immediacy',
      type: 'sublation',
      trigger: 'analogy.presuppositionBecomesExplicit = true',
      effect: 'necessityBridge.emerges = true',
      targetState: 'syl-refl-16',
    },
  ],
  transitions: [
    {
      id: 'syl-refl-8-trans-1',
      from: 'syl-refl-8',
      to: 'syl-refl-16',
      mechanism: 'sublation',
      description: 'From induction/analogy to necessity transition',
    },
  ],
  nextStates: ['syl-refl-16'],
  previousStates: ['syl-refl-1'],
  provenance: {
    topicMapId: 'syl-refl-8-induction-schema',
    lineRange: { start: 189, end: 215 },
    section: 'The Syllogism of Induction',
    order: 8,
  },
  description: REFLECTION_SYLLOGISM_TOPIC_MAP.entries[7]?.description,
  keyPoints: REFLECTION_SYLLOGISM_TOPIC_MAP.entries[7]?.keyPoints,
};

const state3: DialecticState = {
  id: 'syl-refl-16',
  title: 'Reflection syllogism passes to necessity syllogism',
  concept: 'TransitionToNecessitySyllogism',
  phase: 'subject',
  moments: [
    {
      name: 'concreteMediationUnity',
      definition:
        'Mediation is posited as concrete unity of form-determinations',
      type: 'mediation',
    },
    {
      name: 'necessityPassover',
      definition:
        'Sublated immediacy yields objective universality as necessity-middle',
      type: 'passover',
      relation: 'transitions',
      relatedTo: 'concreteMediationUnity',
    },
  ],
  invariants: [
    {
      id: 'syl-refl-16-inv-1',
      constraint: 'singular immediacy is no longer immediate but posited',
      predicate: 'posited(notImmediate(singularity))',
    },
    {
      id: 'syl-refl-16-inv-2',
      constraint: 'higher universality re-joins presupposition and mediation',
      predicate: 'rejoins(higherUniversality, presuppositionMediation)',
    },
  ],
  forces: [
    {
      id: 'syl-refl-16-force-1',
      description:
        'Concrete reflective mediation passes over to syllogism of necessity',
      type: 'passover',
      trigger: 'mediatingUnity.concrete = true',
      effect: 'necessitySyllogism.emerges = true',
      targetState: 'syl-nec-1',
    },
  ],
  transitions: [
    {
      id: 'syl-refl-16-trans-1',
      from: 'syl-refl-16',
      to: 'syl-nec-1',
      mechanism: 'passover',
      description: 'From syllogism of reflection to syllogism of necessity',
    },
  ],
  nextStates: ['syl-nec-1'],
  previousStates: ['syl-refl-8'],
  provenance: {
    topicMapId: 'syl-refl-16-review-necessity',
    lineRange: { start: 514, end: 543 },
    section: 'The Syllogism of Reflection',
    order: 16,
  },
  description: REFLECTION_SYLLOGISM_TOPIC_MAP.entries[15]?.description,
  keyPoints: REFLECTION_SYLLOGISM_TOPIC_MAP.entries[15]?.keyPoints,
};

export const reflectionSyllogismIR: DialecticIR = {
  id: 'reflection-syllogism-ir',
  title:
    'Reflection Syllogism IR: Allness, Induction/Analogy, Necessity Handoff',
  section: 'CONCEPT - SUBJECTIVITY - C. Syllogism - B. Syllogism of Reflection',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'reflection.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'syl-refl-1': 'subject',
      'syl-refl-8': 'subject',
      'syl-refl-16': 'subject',
    },
  },
};

export const reflectionSyllogismStates = {
  'syl-refl-1': state1,
  'syl-refl-8': state2,
  'syl-refl-16': state3,
};
