import type { Chunk, LogicalOperation } from './index'

/**
 * Syllogism of Existence — Second Figure (part 1: P‑S‑U)
 * Conservative chunking and focused HLOs.
 */

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'syll-ex2-1-overview',
    title: 'Second figure — overview (P‑S‑U)',
    text: `P‑S‑U: the particular mediates between singularity and universality. The subject is not immediately united to the universal but through contingency in a singularity; the singular thus functions as the middle in truth.`
  },
  {
    id: 'syll-ex2-2-mediation-is-sublation',
    title: 'Mediation as sublation of immediacy',
    text: `The conclusion is not an immediate connection but a mediated (negative) unity; mediation therefore contains a negative moment rather than being mere immediacy.`
  },
  {
    id: 'syll-ex2-3-premises-status',
    title: 'Premises: one immediate, one mediated',
    text: `In this figure the premises are P‑S (immediate) and S‑U (already mediated via the first figure). Each syllogism presupposes the other; mediation now has a layered character.`
  },
  {
    id: 'syll-ex2-4-exchange-of-places',
    title: 'Exchange of places: particular ↔ singular ↔ universal',
    text: `The particular occupies the subject-place (singularity) while the singular is posited as particularity (middle). The extremes are no longer abstract immediacies but are externally posited into each other's place.`
  },
  {
    id: 'syll-ex2-5-determinate-meaning',
    title: 'Determinate meaning: universal as species via singularity',
    text: `Objectively: the universal is not itself a determinate particular but one species among particulars through mediation of the singular; the singular relates negatively to the particular (it is not the particular's predicate).`
  }
]

// appended: Second Figure — part 2 (conservative chunking + focused HLOs)
CANONICAL_CHUNKS.push(
  {
    id: 'syll-ex2-6-immediate-determinacies',
    title: 'Terms still immediate determinacies; form external',
    text: `The terms remain immediate determinacies and have not advanced to objective signification; the exchanged positions are a merely external form. Thus the terms are still contentual qualities linked by an accidental singularity.`
  },
  {
    id: 'syll-ex2-7-beginning-of-realization-and-alteration',
    title: 'Transition: beginning of realization; alteration of the pure form',
    text: `The move into the second figure begins the concept's realization by positing the negative moment within originally immediate determinateness, but this also alters the pure form so it no longer conforms wholly to S‑P‑U.`
  },
  {
    id: 'syll-ex2-8-subjective-species-and-relation-to-first',
    title: 'Second figure as subjective species; relation to the first',
    text: `Taken subjectively the second figure is a species of inference reducible to S‑P‑U; its truth, however, is not because of the syllogistic form but because the conclusion holds on its own — the first and second presuppose and implicate one another.`
  },
  {
    id: 'syll-ex2-9-particularity-of-conclusion',
    title: 'Conclusion restricted to particular judgment',
    text: `If the second figure's conclusion is correct without qualification, it can only be a particular judgment; particular judgments are limited (positive or negative) and thus the figure yields indeterminate, limited conclusions.`
  },
  {
    id: 'syll-ex2-10-indifference-and-interchangeability',
    title: 'Indifference of extremes; interchangeability of premises',
    text: `Particular and universal as extremes remain immediate and indifferent to one another; either can serve as major or minor, so premises and their roles can be interchanged without intrinsic preference.`
  }
)

// appended: Second Figure — part 3 (conservative chunks + focused HLOs)
CANONICAL_CHUNKS.push(
  {
    id: 'syll-ex2-11-universal-connection',
    title: 'Conclusion as universal connection (positive & negative)',
    text: `Because the conclusion is both positive and negative it is indifferent to those determinations and thus presents as a universal connection; contingency implicit in the first syllogism is here made explicit.`
  },
  {
    id: 'syll-ex2-12-mediation-self-sublating',
    title: 'Mediation self-sublating; singularity as immediate manifold',
    text: `The mediation posited by the second syllogism has the determination of singularity and immediacy and thereby self-sublates: the middle is an external, infinitely manifold singularity and so points beyond itself to a higher mediation.`
  },
  {
    id: 'syll-ex2-13-immediacy-pointing-to-abstract-universal',
    title: 'Immediacy of this figure points to the opposite (abstract universal)',
    text: `The immediacy on which the second figure rests is in fact the sublated first immediacy — the immediacy reflected into itself, i.e. the abstract universal existing for itself.`
  },
  {
    id: 'syll-ex2-14-transition-to-new-form',
    title: 'Transition: qualitative base → another form of syllogism',
    text: `Viewed conceptually the shift here is like an alteration; when the middle is posited in its truth (as the determination of mediation), the figure assumes another form of the syllogism rather than remaining a mere contingent linkage.`
  }
)

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'syll-ex2-op-1-declare-schema',
    chunkId: 'syll-ex2-1-overview',
    label: 'Declare P‑S‑U schema and role of singular as middle-in-truth',
    clauses: ['schema = P-S-U', 'subject.unionToUniversal via contingency(S)', 'singular.functionsAsMiddleInTruth'],
    predicates: [{ name: 'IsSchema', args: ['P-S-U'] }],
    relations: [{ predicate: 'mediates', from: 'singular', to: 'particular+universal' }]
  },
  {
    id: 'syll-ex2-op-2-mediation-negative',
    chunkId: 'syll-ex2-2-mediation-is-sublation',
    label: 'Mediation contains negative moment (sublation of immediacy)',
    clauses: ['conclusion.isMediated = true', 'mediation.includes = negativeMoment'],
    predicates: [{ name: 'IsSublation', args: ['mediation'] }],
    relations: [{ predicate: 'contains', from: 'mediation', to: 'negativeMoment' }]
  },
  {
    id: 'syll-ex2-op-3-premises-immediate-vs-mediated',
    chunkId: 'syll-ex2-3-premises-status',
    label: 'Premises: P‑S immediate; S‑U mediated (presupposes first figure)',
    clauses: ['premise1 = P-S (immediate)', 'premise2 = S-U (mediated)'],
    predicates: [{ name: 'IsImmediate', args: ['P-S'] }, { name: 'IsMediated', args: ['S-U'] }],
    relations: [{ predicate: 'presupposes', from: 'secondFigure', to: 'firstFigure' }]
  },
  {
    id: 'syll-ex2-op-4-exchange-places',
    chunkId: 'syll-ex2-4-exchange-of-places',
    label: 'Particular and singular exchange functional places',
    clauses: ['particular.becomes = subjectAsSingularity', 'singular.assigned = middleAsParticularity'],
    predicates: [{ name: 'ExchangesPlace', args: ['particular','singular'] }],
    relations: [{ predicate: 'positsExternally', from: 'extreme', to: 'otherPlace' }]
  },
  {
    id: 'syll-ex2-op-5-universal-as-species',
    chunkId: 'syll-ex2-5-determinate-meaning',
    label: 'Universal as one species among particulars via singular mediation',
    clauses: ['universal != immediateDeterminateParticular', 'universal = speciesVia(singularity)', 'singularity.relatesNegativelyTo(particular)'],
    predicates: [{ name: 'IsSpeciesVia', args: ['universal','singularity'] }],
    relations: [{ predicate: 'excludes', from: 'singularity', to: 'otherSpecies' }]
  },
  {
    id: 'syll-ex2-op-6-terms-immediate',
    chunkId: 'syll-ex2-6-immediate-determinacies',
    label: 'Terms are immediate; exchanged positions remain external form',
    clauses: [
      'terms.mode = immediateDeterminacies',
      'exchangedPositions = externalForm',
      'linkage.via = accidentalSingularity'
    ],
    predicates: [
      { name: 'IsImmediateDeterminate', args: ['term'] },
      { name: 'HasExternalForm', args: ['exchangedPositions'] }
    ],
    relations: [
      { predicate: 'linkedBy', from: 'termPair', to: 'accidentalSingularity' }
    ]
  },

  {
    id: 'syll-ex2-op-7-form-transition',
    chunkId: 'syll-ex2-7-beginning-of-realization-and-alteration',
    label: 'Transition posits negative moment; pure form altered and nonconformant',
    clauses: [
      'transition => negativeMoment.positedInImmediate',
      'pureForm.altered = true',
      'conformityTo(S-P-U) = false (in this moment)'
    ],
    predicates: [
      { name: 'PositsNegativeMoment', args: ['transition'] },
      { name: 'AltersForm', args: ['pureForm'] }
    ],
    relations: [
      { predicate: 'resultsIn', from: 'transition', to: 'nonconformity' }
    ]
  },

  {
    id: 'syll-ex2-op-8-indifference-of-extremes',
    chunkId: 'syll-ex2-10-indifference-and-interchangeability',
    label: 'Extremes are indifferent; premises interchangeable; conclusion particular',
    clauses: [
      'extremes.indifferent = true',
      'eitherCanBeMajorOrMinor = true',
      'conclusion.form = particular (limited)'
    ],
    predicates: [
      { name: 'IsInterchangeable', args: ['extreme'] },
      { name: 'YieldsParticularConclusion', args: ['figure'] }
    ],
    relations: [
      { predicate: 'permits', from: 'form', to: 'roleInterchange' }
    ]
  },
  {
    id: 'syll-ex2-op-11-universal-connection',
    chunkId: 'syll-ex2-11-universal-connection',
    label: 'Conclusion indifferent to positive/negative → reads as universal',
    clauses: [
      'conclusion.isPositive && conclusion.isNegative => conclusion.isUniversal',
      'contingency(firstSyllogism) => positedIn(secondFigure)'
    ],
    predicates: [
      { name: 'IsUniversalConnection', args: ['conclusion'] }
    ],
    relations: [
      { predicate: 'makesExplicit', from: 'secondFigure', to: 'firstFigure.contingency' }
    ]
  },

  {
    id: 'syll-ex2-op-12-self-sublating-mediation',
    chunkId: 'syll-ex2-12-mediation-self-sublating',
    label: 'Mediation contains singularity/immediacy and thereby self-sublates',
    clauses: [
      'mediation.has = {singularity, immediacy}',
      'singularity.isInfinitelyManifold = true',
      'mediation.pointsBeyondItself = true'
    ],
    predicates: [
      { name: 'IsSelfSublating', args: ['mediation'] }
    ],
    relations: [
      { predicate: 'pointsTo', from: 'mediation', to: 'higherMediation(universal)' }
    ]
  },

  {
    id: 'syll-ex2-op-13-immediacy-to-abstract-universal',
    chunkId: 'syll-ex2-13-immediacy-pointing-to-abstract-universal',
    label: 'Immediacy in this figure corresponds to the reflected abstract universal',
    clauses: [
      'immediacy.thisFigure = reflectedFirstImmediacy',
      'reflectedImmediacy = abstractUniversal'
    ],
    predicates: [
      { name: 'IsReflectedImmediacy', args: ['immediacy'] }
    ],
    relations: [
      { predicate: 'equates', from: 'reflectedImmediacy', to: 'abstractUniversal' }
    ]
  },

  {
    id: 'syll-ex2-op-14-transition-new-form',
    chunkId: 'syll-ex2-14-transition-to-new-form',
    label: 'When middle is posited in its truth the figure becomes another form',
    clauses: [
      'if middle.positedInTruth then figure.form -> otherSyllogismForm',
      'contingentLinkage => transformedInto(determinedMediation)'
    ],
    predicates: [
      { name: 'TransformsForm', args: ['figure','newForm'] }
    ],
    relations: [
      { predicate: 'yields', from: 'positedMiddle', to: 'newSyllogisticForm' }
    ]
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
