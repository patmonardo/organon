import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { absoluteIdeaTopicMap } from './sources/absolute-idea-topic-map';

const state1: DialecticState = {
  id: 'absolute-idea-1',
  title: 'Absolute idea as identity of true and good',
  concept: 'AbsoluteIdeaIdentity',
  phase: 'subject',
  moments: [
    {
      name: 'identityOfTheoreticalAndPractical',
      definition:
        'Absolute idea unifies theoretical and practical idea as self-rejoining rational concept',
      type: 'sublation',
    },
    {
      name: 'allTruthAsSelfKnowingLife',
      definition:
        'Absolute idea is imperishable self-knowing truth and the comprehensive content of philosophy',
      type: 'reflection',
      relation: 'contains',
      relatedTo: 'identityOfTheoreticalAndPractical',
    },
  ],
  invariants: [
    {
      id: 'absolute-idea-1-inv-1',
      constraint:
        'absolute idea preserves oppositions only as moments of self-identity',
      predicate: 'preservesAsMoments(opposition, absoluteIdentity)',
    },
    {
      id: 'absolute-idea-1-inv-2',
      constraint:
        'philosophical content is unified as concept apprehending itself',
      predicate: 'selfApprehendingConcept(philosophicalContent)',
    },
  ],
  forces: [
    {
      id: 'absolute-idea-1-force-1',
      description:
        'Identity differentiates into explicit logical form and universal method',
      type: 'mediation',
      trigger: 'identity.requiresExplicitForm = true',
      effect: 'logicalMethod.explicit = true',
      targetState: 'absolute-idea-3',
    },
  ],
  transitions: [
    {
      id: 'absolute-idea-1-trans-1',
      from: 'absolute-idea-1',
      to: 'absolute-idea-3',
      mechanism: 'mediation',
      description:
        'From identity of true and good to method as self-knowing concept',
    },
  ],
  nextStates: ['absolute-idea-3'],
  previousStates: ['idea-good-3'],
  provenance: {
    topicMapId: 'absolute-idea-1',
    lineRange: { start: 6, end: 72 },
    section: 'A. THE ABSOLUTE IDEA',
    order: 1,
  },
  description: absoluteIdeaTopicMap[0]?.description,
  keyPoints: absoluteIdeaTopicMap[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'absolute-idea-3',
  title: 'Method as absolute self-knowing movement',
  concept: 'AbsoluteMethod',
  phase: 'subject',
  moments: [
    {
      name: 'methodAsConceptMovement',
      definition:
        'Method is the concept’s own universal activity, both subjective cognition and objective substantiality',
      type: 'process',
    },
    {
      name: 'infiniteForceOfReason',
      definition:
        'As internal and external mode, method penetrates all determinacy and is the sole force of reason',
      type: 'immanence',
      relation: 'contains',
      relatedTo: 'methodAsConceptMovement',
    },
  ],
  invariants: [
    {
      id: 'absolute-idea-3-inv-1',
      constraint:
        'method is not external form but in-and-for-itself determinateness of concept',
      predicate: 'inAndForItselfDeterminateness(method, concept)',
    },
    {
      id: 'absolute-idea-3-inv-2',
      constraint:
        'objective determinacy is posited in identity with subjective concept',
      predicate: 'identity(subjectiveConcept, objectiveDeterminacy)',
    },
  ],
  forces: [
    {
      id: 'absolute-idea-3-force-1',
      description:
        'Methodic self-comprehension culminates in logic’s own closure and transition-shape',
      type: 'passover',
      trigger: 'concept.selfComprehension = complete',
      effect: 'transitionForm.explicit = true',
      targetState: 'absolute-idea-4',
    },
  ],
  transitions: [
    {
      id: 'absolute-idea-3-trans-1',
      from: 'absolute-idea-3',
      to: 'absolute-idea-4',
      mechanism: 'passover',
      description:
        'From method as absolute force to transition at logic’s completion',
    },
  ],
  nextStates: ['absolute-idea-4'],
  previousStates: ['absolute-idea-1'],
  provenance: {
    topicMapId: 'absolute-idea-3',
    lineRange: { start: 113, end: 221 },
    section: 'A. THE ABSOLUTE IDEA',
    order: 2,
  },
  description: absoluteIdeaTopicMap[2]?.description,
  keyPoints: absoluteIdeaTopicMap[2]?.keyPoints,
};

const state3: DialecticState = {
  id: 'absolute-idea-4',
  title: 'Logic completion and freely discharged transition-shape',
  concept: 'AbsoluteLiberation',
  phase: 'subject',
  moments: [
    {
      name: 'scienceSelfComprehension',
      definition:
        'Logic apprehends its own concept as pure idea running through and recollecting its full determinations',
      type: 'reflection',
    },
    {
      name: 'freeDischarge',
      definition:
        'Idea freely discharges itself while remaining with itself in transparent immediacy',
      type: 'passover',
      relation: 'passesOver',
      relatedTo: 'scienceSelfComprehension',
    },
  ],
  invariants: [
    {
      id: 'absolute-idea-4-inv-1',
      constraint:
        'transition is absolute liberation rather than external becoming',
      predicate: 'absoluteLiberation(transition)',
    },
    {
      id: 'absolute-idea-4-inv-2',
      constraint:
        'pure truth as result is equally beginning of further methodic articulation',
      predicate: 'resultEqualsBeginning(pureTruth)',
    },
  ],
  forces: [
    {
      id: 'absolute-idea-4-force-1',
      description:
        'Completed absolute idea opens into explicit treatment of method beginning',
      type: 'mediation',
      trigger: 'absoluteIdea.requiresMethodExposition = true',
      effect: 'methodBeginning.starts = true',
      targetState: 'method-beginning-1',
    },
  ],
  transitions: [
    {
      id: 'absolute-idea-4-trans-1',
      from: 'absolute-idea-4',
      to: 'method-beginning-1',
      mechanism: 'mediation',
      description:
        'From absolute idea closure to the beginning of method exposition',
    },
  ],
  nextStates: ['method-beginning-1'],
  previousStates: ['absolute-idea-3'],
  provenance: {
    topicMapId: 'absolute-idea-4',
    lineRange: { start: 223, end: 295 },
    section: 'A. THE ABSOLUTE IDEA',
    order: 3,
  },
  description: absoluteIdeaTopicMap[3]?.description,
  keyPoints: absoluteIdeaTopicMap[3]?.keyPoints,
};

export const absoluteIdeaIR: DialecticIR = {
  id: 'absolute-idea-ir',
  title: 'Absolute Idea IR: Identity, Method, Liberation',
  section: 'CONCEPT - IDEA - C. Speculation - A. The Absolute Idea',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'absolute-idea.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'absolute-idea-1': 'subject',
      'absolute-idea-3': 'subject',
      'absolute-idea-4': 'subject',
    },
  },
};

export const absoluteIdeaStates = {
  'absolute-idea-1': state1,
  'absolute-idea-3': state2,
  'absolute-idea-4': state3,
};
