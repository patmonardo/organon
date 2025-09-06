import type { Chunk, LogicalOperation } from './index'

/**
 * Syllogism of Existence — Third Figure (S‑U‑P)
 * Complete module: chunkified + conservative HLO extraction.
 */

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'syll-ex3-1-overview',
    title: 'Third figure — overview (S‑U‑P)',
    text: `S‑U‑P presupposes the first and second figures (S‑U from the first; P‑U from the second) and is in turn presupposed by them; reciprocal mediation brings the determination of the syllogism to completion.`
  },
  {
    id: 'syll-ex3-2-reciprocal-mediation',
    title: 'Reciprocal mediation and incompleteness',
    text: `Each syllogism is a mediation but does not possess the totality of mediation; reciprocity means each is affected by an immediacy whose mediation lies outside it.`
  },
  {
    id: 'syll-ex3-3-formal-truth',
    title: 'S‑U‑P as the truth of the formal syllogism',
    text: `Considered in itself S‑U‑P expresses that its middle is the abstract universal and that the extremes are only contained in it as universals, not according to their essential determinateness.`
  },
  {
    id: 'syll-ex3-4-terms-immediate-content',
    title: 'Terms have immediate content indifferent to form',
    text: `Posited here is the formalism: terms have immediate content indifferent toward the form — they are form-determinations not yet reflected into content.`
  },
  {
    id: 'syll-ex3-5-middle-as-indeterminate-universal',
    title: 'Middle = indeterminate universal (abstraction from determinateness)',
    text: `The middle is the unity of the extremes but abstracts from their determinateness; as an indeterminate universal it subsumes but does not contain the specifics that were meant to be mediated.`
  },
  {
    id: 'syll-ex3-6-legitimacy-and-negative-conclusion',
    title: 'Legitimacy requires negative judgment; conclusion is negative',
    text: `For the figure to conform as a syllogism both S‑U and P‑U must hold in the same relation; this typically requires a negative judgment (indifferent subject/predicate relation), so the conclusion is necessarily negative.`
  },
  {
    id: 'syll-ex3-7-indifference-and-fourth-figure',
    title: 'Indifference of roles → fourth figure and its vacuity',
    text: `When subject/predicate distinction becomes indifferent, which premise is major/minor is indifferent — the origin of the customary fourth figure (a void distinction grounded in abstraction).`
  },
  {
    id: 'syll-ex3-8-objective-significance-limited',
    title: 'Objective significance: universality as middle is qualitative/abstract',
    text: `Objectively, the middle is essentially a universal; but when universality is only qualitative/abstract, determinateness of extremes is not contained and any conjunction still depends on mediation external to the figure.`
  },
  {
    id: 'syll-ex3-9-relationless-figure-uuu',
    title: 'Relationless figure (U‑U‑U) as extreme abstraction',
    text: `Abstracting from qualitative differentiation yields a relationless figure U‑U‑U, whose unity is merely external equality rather than concrete mediation.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'syll-ex3-op-1-declare-schema-and-reciprocity',
    chunkId: 'syll-ex3-1-overview',
    label: 'Declare S‑U‑P and reciprocal presupposition',
    clauses: [
      'schema = S-U-P',
      'presupposes = {firstFigure, secondFigure}',
      'isPresupposedBy = {firstFigure, secondFigure}'
    ],
    predicates: [{ name: 'IsSchema', args: ['S-U-P'] }],
    relations: [{ predicate: 'reciprocallyPresupposes', from: 'thirdFigure', to: 'firstAndSecond' }]
  },

  {
    id: 'syll-ex3-op-2-mediation-lacks-totality',
    chunkId: 'syll-ex3-2-reciprocal-mediation',
    label: 'Each mediation lacks totality; immediacy’s mediation lies outside',
    clauses: ['eachSyllogism.isMediation', 'eachSyllogism.lacksTotality', 'immediacy.mediation ∉ sameSyllogism'],
    predicates: [{ name: 'LacksTotality', args: ['syllogism'] }],
    relations: [{ predicate: 'isAffectedBy', from: 'syllogism', to: 'externalImmediacy' }]
  },

  {
    id: 'syll-ex3-op-3-middle-abstract-universal',
    chunkId: 'syll-ex3-3-formal-truth',
    label: 'Middle is abstract universal; extremes contained only as universals',
    clauses: ['middle.is = abstractUniversal', 'extremes.containedAs = universalityOnly', 'essentialDeterminateness.notContained'],
    predicates: [{ name: 'IsAbstractUniversal', args: ['middle'] }],
    relations: [{ predicate: 'containsOnlyAs', from: 'middle', to: 'extremes.universalAspect' }]
  },

  {
    id: 'syll-ex3-op-4-terms-immediate-content',
    chunkId: 'syll-ex3-4-terms-immediate-content',
    label: 'Terms possess immediate content indifferent to form',
    clauses: ['terms.haveImmediateContent', 'immediacy.isIndifferentTo(form)'],
    predicates: [{ name: 'HasImmediateContent', args: ['term'] }],
    relations: [{ predicate: 'isIndifferentTo', from: 'content', to: 'form' }]
  },

  {
    id: 'syll-ex3-op-5-indeterminate-universal',
    chunkId: 'syll-ex3-5-middle-as-indeterminate-universal',
    label: 'Middle unifies by abstraction; lacks the contained determinateness',
    clauses: ['middle.unifies = byAbstraction', 'abstraction => determinatenessExcluded', 'syllogism.relationToConcept = pending'],
    predicates: [{ name: 'IsIndeterminateUniversal', args: ['middle'] }],
    relations: [{ predicate: 'abstractsFrom', from: 'middle', to: 'extremes.determinateness' }]
  },

  {
    id: 'syll-ex3-op-6-negative-conclusion-condition',
    chunkId: 'syll-ex3-6-legitimacy-and-negative-conclusion',
    label: 'Legitimacy requires symmetrical relation → negative conclusion',
    clauses: ['if S-U and P-U share relation then conclusion.requires = negativeJudgment', 'negativeConclusion => subject/predicate roles indifferent'],
    predicates: [{ name: 'RequiresNegative', args: ['figure'] }],
    relations: [{ predicate: 'rendersIndifferent', from: 'negativeConclusion', to: 'roles' }]
  },

  {
    id: 'syll-ex3-op-7-fourth-figure-vacuity',
    chunkId: 'syll-ex3-7-indifference-and-fourth-figure',
    label: 'Role indifference motivates fourth figure; distinction is idle',
    clauses: ['roleIndifference => majorMinor.indifferent', 'fourthFigure.origin = abstraction', 'fourthFigure.value = void'],
    predicates: [{ name: 'IsIdleDistinction', args: ['fourthFigure'] }],
    relations: [{ predicate: 'derivesFrom', from: 'fourthFigure', to: 'roleIndifference' }]
  },

  {
    id: 'syll-ex3-op-8-objective-significance-limited',
    chunkId: 'syll-ex3-8-objective-significance-limited',
    label: 'Universality-as-middle is qualitative; conjunction depends on external mediation',
    clauses: ['middle.asUniversal = qualitative', 'extremes.determinateness.notContainedIn(middle)', 'conjunction.requires = externalMediation'],
    predicates: [{ name: 'IsQualitativeUniversal', args: ['middle'] }],
    relations: [{ predicate: 'dependsOn', from: 'conjunction', to: 'externalMediation' }]
  },

  {
    id: 'syll-ex3-op-9-relationless-figure-uuu',
    chunkId: 'syll-ex3-9-relationless-figure-uuu',
    label: 'Abstracting yields U‑U‑U: relationless equality, not concrete mediation',
    clauses: ['abstractFromQualities => figure = U-U-U', 'U-U-U = externalEqualityOnly'],
    predicates: [{ name: 'IsRelationlessFigure', args: ['U-U-U'] }],
    relations: [{ predicate: 'expresses', from: 'U-U-U', to: 'externalEquality' }]
  }
]

/* accessors */
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null
}

export function getLogicalOpsForChunk(oneBasedIndex: number) {
  const chunk = getChunk(oneBasedIndex)
  if (!chunk) return []
  return LOGICAL_OPERATIONS.filter(op => op.chunkId === chunk.id)
}
