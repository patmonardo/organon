import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { LIVING_INDIVIDUAL_TOPIC_MAP } from './sources/living-individual-topic-map';

const state1: DialecticState = {
  id: 'idea-1',
  title: 'Life as immediate idea and creative presupposing',
  concept: 'LivingImmediateIdea',
  phase: 'subject',
  moments: [
    {
      name: 'creativePresupposing',
      definition:
        'Universal life presupposes itself, dirempts into judgment, and begins the syllogistic articulation of living individuality',
      type: 'mediation',
    },
    {
      name: 'negativeUnityOfExternality',
      definition:
        'Conceptual unity holds externality as its own moment rather than as a merely alien object',
      type: 'reflection',
      relation: 'contains',
      relatedTo: 'creativePresupposing',
    },
  ],
  invariants: [
    {
      id: 'idea-1-inv-1',
      constraint:
        'life begins as immediate identity of concept and objectivity',
      predicate: 'immediateIdentity(life, concept, objectivity)',
    },
    {
      id: 'idea-1-inv-2',
      constraint:
        'diremption preserves unity by generating mediating judgment form',
      predicate: 'preservedThroughDiremption(lifeUnity)',
    },
  ],
  forces: [
    {
      id: 'idea-1-force-1',
      description:
        'Immediate life must become determinate individual organism to actualize itself',
      type: 'negation',
      trigger: 'immediacy.requiresDeterminateCorporeity = true',
      effect: 'organismicIndividuality.emerges = true',
      targetState: 'idea-4',
    },
  ],
  transitions: [
    {
      id: 'idea-1-trans-1',
      from: 'idea-1',
      to: 'idea-4',
      mechanism: 'negation',
      description:
        'From immediate life to living individual as organism with articulated members',
    },
  ],
  nextStates: ['idea-4'],
  previousStates: ['realized-15'],
  provenance: {
    topicMapId: 'living-individual-1-creative-presupposing',
    lineRange: { start: 3, end: 74 },
    section: 'The Living Individual',
    order: 1,
  },
  description: LIVING_INDIVIDUAL_TOPIC_MAP.entries[0]?.description,
  keyPoints: LIVING_INDIVIDUAL_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'idea-4',
  title: 'Living individual as organism and internal differentiation',
  concept: 'OrganismicIndividuality',
  phase: 'subject',
  moments: [
    {
      name: 'organismicMembers',
      definition:
        'The individual is an organism whose members are moments of one living totality, not externally independent parts',
      type: 'determination',
    },
    {
      name: 'triplicityOfSensibilityIrritabilityReproduction',
      definition:
        'Universality, particularity, and singularity unfold as sensibility, irritability, and reproduction',
      type: 'process',
      relation: 'contains',
      relatedTo: 'organismicMembers',
    },
  ],
  invariants: [
    {
      id: 'idea-4-inv-1',
      constraint: 'members exist only as functions of the living individual',
      predicate: 'memberDependsOnIndividuality(member)',
    },
    {
      id: 'idea-4-inv-2',
      constraint:
        'reproduction gathers all moments into concrete self-related singularity',
      predicate:
        'totalizedInReproduction(universality, particularity, singularity)',
    },
  ],
  forces: [
    {
      id: 'idea-4-force-1',
      description:
        'Internal differentiation drives the organism to process itself as premise and conclusion',
      type: 'sublation',
      trigger: 'individuality.positsOwnExternalReference = true',
      effect: 'lifeProcess.emerges = true',
      targetState: 'idea-6',
    },
  ],
  transitions: [
    {
      id: 'idea-4-trans-1',
      from: 'idea-4',
      to: 'idea-6',
      mechanism: 'sublation',
      description:
        'From articulated organism to self-relating process of production and reproduction',
    },
  ],
  nextStates: ['idea-6'],
  previousStates: ['idea-1'],
  provenance: {
    topicMapId: 'living-individual-4-sensibility-irritability-reproduction',
    lineRange: { start: 206, end: 331 },
    section: 'The Living Individual',
    order: 2,
  },
  description: LIVING_INDIVIDUAL_TOPIC_MAP.entries[3]?.description,
  keyPoints: LIVING_INDIVIDUAL_TOPIC_MAP.entries[3]?.keyPoints,
};

const state3: DialecticState = {
  id: 'idea-6',
  title: 'Premise as conclusion in the individual life process',
  concept: 'SelfProducingIndividualLife',
  phase: 'subject',
  moments: [
    {
      name: 'premiseAsConclusion',
      definition:
        'The living process starts from what it also returns to, so production and product reciprocally generate one another',
      type: 'reflection',
    },
    {
      name: 'impulseAsConceptualNegativity',
      definition:
        'Impulse enacts the concept as negativity that transforms externality into self-reference',
      type: 'negation',
      relation: 'transitions',
      relatedTo: 'premiseAsConclusion',
    },
  ],
  invariants: [
    {
      id: 'idea-6-inv-1',
      constraint: 'product remains identical with the producing life activity',
      predicate: 'identical(product, producingActivity)',
    },
    {
      id: 'idea-6-inv-2',
      constraint:
        'externality is continuously reabsorbed into conceptual self-mediation',
      predicate: 'reabsorbedIntoConceptualMediation(externality)',
    },
  ],
  forces: [
    {
      id: 'idea-6-force-1',
      description:
        'The individual life process externalizes into confrontation with an objective world',
      type: 'passover',
      trigger: 'selfReproduction.encountersExternalObjectivity = true',
      effect: 'lifeProcessOpposition.emerges = true',
      targetState: 'life-process-1',
    },
  ],
  transitions: [
    {
      id: 'idea-6-trans-1',
      from: 'idea-6',
      to: 'life-process-1',
      mechanism: 'passover',
      description:
        'From internal self-production to explicit life-process against objective world',
    },
  ],
  nextStates: ['life-process-1'],
  previousStates: ['idea-4'],
  provenance: {
    topicMapId: 'living-individual-3-internal-process',
    lineRange: { start: 162, end: 204 },
    section: 'The Living Individual',
    order: 3,
  },
  description: LIVING_INDIVIDUAL_TOPIC_MAP.entries[2]?.description,
  keyPoints: LIVING_INDIVIDUAL_TOPIC_MAP.entries[2]?.keyPoints,
};

export const livingIndividualIR: DialecticIR = {
  id: 'living-individual-ir',
  title: 'Living Individual IR: Immediate Life, Organism, Internal Process',
  section: 'CONCEPT - IDEA - A. Life - A. The Living Individual',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'living-individual.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'idea-1': 'subject',
      'idea-4': 'subject',
      'idea-6': 'subject',
    },
  },
};

export const livingIndividualStates = {
  'idea-1': state1,
  'idea-4': state2,
  'idea-6': state3,
};
