import type { Chunk, LogicalOperation } from './index'

/* c. The Infinite Judgment — part 1 (negative-infinite, nonsensical judgments; examples) */
export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'inf-1-overview',
    title: 'Infinite judgment — overview (negative infinite)',
    text: `The infinite judgment, as commonly presented, is the negative-infinite: a judgment where the form itself is sublated and the predicate's universality is denied altogether. Such judgments can be formally 'correct' yet nonsensical because the subject–predicate connection is effectively absent.`
  },
  {
    id: 'inf-2-examples-and-character',
    title: 'Examples and character of negative-infinite judgments',
    text: `Typical cases: "spirit is not red", "the rose is not an elephant", or negations that deny the universal sphere itself (e.g., crime negates 'right' as such). These are correct in the weak sense but lack meaningful judgment‑connection and so are fatuous.`
  },
  {
    id: 'inf-3-nonsensical-but-effective',
    title: 'Nonsensical but effective actions as infinite judgments',
    text: `Some infinite judgments operate as effective actions (e.g., denying property in litigation by denying right as such). They function, but because they negate the governing universal-sphere they are morally or conceptually nonsensical within that sphere.`
  },
  {
    id: 'inf-4-diagnostic-signs',
    title: 'Diagnostic signs of infinite judgment',
    text: `Signs: subject and predicate do not share a universal sphere; predicate denies the very category that would make predication meaningful; the assertion yields no conceptual mediation. Such nodes should be classified as negative-infinite / nonsensical in the KG.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'inf-op-1-declare-negative-infinite',
    chunkId: 'inf-1-overview',
    label: 'Declare negative-infinite and its paradoxical status',
    clauses: [
      'infiniteJudgment = negativeInfinite',
      'containsJudgmentForm? => yes, yet connectionAbsent -> nonsensical'
    ],
    predicates: [{ name: 'IsNegativeInfinite', args: [] }],
    relations: [{ predicate: 'flags', from: 'infiniteJudgment', to: 'paradoxStatus' }]
  },
  {
    id: 'inf-op-2-detect-nonsensical-mismatch',
    chunkId: 'inf-2-examples-and-character',
    label: 'Detect subject–predicate universal mismatch (nonsensical)',
    clauses: [
      'if not(sharedUniversal(subject,predicate)) then tag(node, infiniteNonsensical)',
      'examples = {"spirit is not red","rose is not an elephant"}'
    ],
    predicates: [{ name: 'DetectsUniversalMismatch', args: [] }],
    relations: [{ predicate: 'tags', from: 'detector', to: 'infiniteNonsensicalNodes' }]
  },
  {
    id: 'inf-op-3-classify-effective-but-nonsensical',
    chunkId: 'inf-3-nonsensical-but-effective',
    label: 'Classify effective actions that negate universality as infinite judgments',
    clauses: [
      'if action.negates(universalSphere) && action.effective then classify(as: infiniteJudgment, note: "practical but nonsensical")',
      'attachContextualEvidence(action)'
    ],
    predicates: [{ name: 'ClassifyEffectiveInfinite', args: [] }],
    relations: [{ predicate: 'attaches', from: 'classifier', to: 'contextualEvidence' }]
  },
  {
    id: 'inf-op-4-kg-tagging-and-curation',
    chunkId: 'inf-4-diagnostic-signs',
    label: 'KG tagging and curator workflow for infinite judgments',
    clauses: [
      'on(detect(infiniteNonsensical)) => tag(KG,node,"infinite-nonsensical")',
      'emit(curatorReview, payload:{node,examples,grounds})'
    ],
    predicates: [{ name: 'KGFlagsInfinite', args: [] }],
    relations: [{ predicate: 'escalates', from: 'KG', to: 'curatorReview' }]
  }
]

/* appended: c. The Infinite Judgment — part 2 (positive element; negation of negation; "singular is singular", "universal is universal") */
CANONICAL_CHUNKS.push(
  {
    id: 'inf-5-positive-element',
    title: 'Infinite judgment — positive element (negation of negation)',
    text: `The positive element of the infinite judgment (the negation of the negation) is the reflection of singularity into itself by which the singular is first posited as the determinate determinate. "The singular is singular" is the expression of this reflection.`
  },
  {
    id: 'inf-6-subject-posited-as-singular',
    title: 'Subject posited as singular via mediation',
    text: `In the judgment of existence the subject is immediate; through negative and infinite mediation it becomes posited as singular for the first time — the singular expands into a predicate identical with it, and universality becomes a summing of distincts.`
  },
  {
    id: 'inf-7-universal-reflection',
    title: 'Positive infinite: "the universal is universal"',
    text: `The positively infinite judgment likewise says "the universal is universal": the universal is posited as turning back into itself, purified and self-related through the mediation that negates limitation.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'inf-op-5-formalize-positive-element',
    chunkId: 'inf-5-positive-element',
    label: 'Formalize negation-of-negation as positive element',
    clauses: [
      'positiveElement = negationOfNegation(reflectionOfSingularity)',
      'on(positiveElement) => express("singular is singular")'
    ],
    predicates: [{ name: 'IsNegationOfNegation', args: [] }],
    relations: [{ predicate: 'expresses', from: 'positiveElement', to: '"singular is singular"' }]
  },
  {
    id: 'inf-op-6-posited-subject-and-universal',
    chunkId: 'inf-6-subject-posited-as-singular',
    label: 'Encode subject posited as singular and universal purified',
    clauses: [
      'mediate(negative, infinite) => subject.positedAs(singular)',
      'universality -> summingOfDistincts',
      'on(positiveInfinite) => express("universal is universal")'
    ],
    predicates: [{ name: 'PositsSubjectAndPurifiesUniversal', args: [] }],
    relations: [{ predicate: 'yields', from: 'mediation', to: 'positedSubject' }]
  }
)

/* appended: c. The Infinite Judgment — part 3 (sublation of the judgment; transition to judgment of reflection) */
CANONICAL_CHUNKS.push(
  {
    id: 'inf-8-sublation-status',
    title: 'Sublation of the judgment — negative vs positive infinite',
    text: `Through reflection the judgment sublates itself. In the negative-infinite the difference is so great that subject and predicate have no positive connection; in the positive-infinite identity is total so that no judgment (distinct predication) remains.`
  },
  {
    id: 'inf-9-concept-unity-torn',
    title: 'Conceptual unity torn and reflected',
    text: `What the copula contains — unity in which qualitative extremes are sublated — is the concept. That unity is immediately torn and becomes a judgment whose terms are no longer immediate but are reflected into themselves.`
  },
  {
    id: 'inf-10-transition-to-reflection',
    title: 'Transition: from judgment of existence to judgment of reflection',
    text: `The judgment of existence has passed over into the judgment of reflection: the copula's contained unity becomes explicit concept, terms are reflected and mediated, and the determinate judgment is now a reflection rather than mere existence.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'inf-op-7-detect-sublation',
    chunkId: 'inf-8-sublation-status',
    label: 'Detect sublation status of infinite judgments',
    clauses: [
      'if subjectPredicate.connection == absent then mark(node, negativeInfiniteSublated)',
      'if subjectPredicate.identity == total then mark(node, positiveInfiniteSublated)'
    ],
    predicates: [{ name: 'DetectsSublationStatus', args: [] }],
    relations: [{ predicate: 'marks', from: 'detector', to: 'sublationState' }]
  },
  {
    id: 'inf-op-8-formalize-concept-torn',
    chunkId: 'inf-9-concept-unity-torn',
    label: 'Formalize concept-as-unity torn into reflected terms',
    clauses: [
      'concept = unity(qualitativeExtremes).sublated',
      'on(concept.torn) => convert(extremes -> reflectedTerms)',
      'reflectedTerms.notImmediate = true'
    ],
    predicates: [{ name: 'FormalizesTornConcept', args: [] }],
    relations: [{ predicate: 'converts', from: 'concept', to: 'reflectedTerms' }]
  },
  {
    id: 'inf-op-9-transition-to-reflection',
    chunkId: 'inf-10-transition-to-reflection',
    label: 'Reclassify judgment of existence into judgment of reflection',
    clauses: [
      'on(sublationComplete) => reclassify(judgment.kind = reflection)',
      'preserve(meta:{copula,origin,notes})',
      'emit(event:"transitionedToReflection", payload:{node})'
    ],
    predicates: [{ name: 'TransitionsToReflection', args: [] }],
    relations: [{ predicate: 'reclassifies', from: 'system', to: 'judgmentNode' }]
  }
)
