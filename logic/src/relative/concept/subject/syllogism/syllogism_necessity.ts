import type { Chunk, LogicalOperation } from './index'

/**
 * Part 1 — The Categorical Syllogism (first section)
 * - chunkified and HLO-extracted for later EPA/truth mapping
 */

/* canonical textual chunks (small, editable units) */
export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'syll-nec-1-overview',
    title: 'A. The categorical syllogism — overview',
    text: `The categorical syllogism is the first syllogism of necessity; superficially a syllogism of inherence, but properly one where subject and predicate are conjoined through substance considered as universal.`
  },
  {
    id: 'syll-nec-2-middle-term-universality',
    title: 'Middle term as objective universality',
    text: `Associated is the more specific signification that the middle term is the objective universality — the universal posited in and for itself (not mere accident).`
  },
  {
    id: 'syll-nec-3-substance-as-universal',
    title: 'Substance elevated to universal (conceptual)',
    text: `When elevated to the concept, substance is the universal whose mode of being is the determination of the concept rather than accidental attributes; differences are extremes (universality and singularity).`
  },
  {
    id: 'syll-nec-4-universality-and-singularity',
    title: 'Universality (abstract determinateness) vs singularity (concrete unity)',
    text: `Universality is an abstract determinateness (the specific difference of substance); singularity is the concrete unity of genus and determinateness (immediate singularity as subsistence).`
  },
  {
    id: 'syll-nec-5-premises-categorical',
    title: 'Both premises are categorical judgments',
    text: `The connection of an extreme to the middle constitutes a categorical judgment; since the other extreme also expresses specific difference, both premises of the categorical syllogism are categorical.`
  },

  // appended: part 2 chunks
  {
    id: 'syll-nec-6-specific-difference-as-ground',
    title: 'Specific difference as ground of predication',
    text: `The specific difference (the abstract determinateness summed in the universal) functions as the ground in which predication rests: the predicate is tied to the subject through the substance's determinate identity.`
  },
  {
    id: 'syll-nec-7-major-minor-structure',
    title: 'Major and minor as categorical premises',
    text: `The major premise expresses the universal determinateness of substance; the minor connects the singular to that determinateness. Together they constitute a necessity-grounded chain rather than mere inherence.`
  },
  {
    id: 'syll-nec-8-necessity-and-substance',
    title: 'Necessity: predicate grounded in concept of substance',
    text: `Necessity here means the predicate follows from the subject insofar as the subject's concept (substance as universal) contains the predicate — the relation is conceptual, not contingent.`
  },

  // appended: part 3 chunks (categorical — subjective element, contingency, extremes, transition)
  {
    id: 'syll-nec-9-subjective-element',
    title: '3 — Subjective element in the categorical syllogism',
    text: `There remains a subjective element: the substantial identity is still contentual identity, not identity of form. The concept's identity is an inner bond and is necessity as connection, but the negativity of extremes is not yet posited.`
  },
  {
    id: 'syll-nec-10-singular-contingency',
    title: 'Singular as immediate; contingency of its subsumption',
    text: `The truly immediate element is the singular, subsumed under the genus (middle); many other singulars are likewise subsumed, so the choice of this singular is contingent. The singular's objectivity as universality also renders it a subjective actuality.`
  },
  {
    id: 'syll-nec-11-extremes-immediacy-and-indifference',
    title: 'Extremes: immediacy, contingency, and indifference',
    text: `Each extreme contains determinations not contained in the middle; they have a concrete existence indifferent to the middle and are contingent relative to it. Extremes may be self-subsistent actualities while also being sublated in identity.`
  },
  {
    id: 'syll-nec-12-formal-identity-and-transition',
    title: 'Formal inner identity → determination toward hypothetical syllogism',
    text: `The identity present is only formal, an inner identity; because of this, the syllogism of necessity determines itself into the hypothetical syllogism — the necessity-form transitions into a hypothetical shape.`
  }
]

/* logical operations (HLOs) extracted from each chunk */
export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'syll-op-nec-1-declare-categorical',
    chunkId: 'syll-nec-1-overview',
    label: 'Declare categorical syllogism as syllogism of necessity',
    clauses: [
      'syllogism.form = categorical',
      'syllogism.status = necessity',
      'superficialForm = inherence'
    ],
    predicates: [
      { name: 'IsForm', args: ['syllogism','categorical'] },
      { name: 'HasStatus', args: ['syllogism','necessity'] }
    ],
    relations: [
      { predicate: 'isTakenAs', from: 'categorical', to: 'inherence' }
    ]
  },

  {
    id: 'syll-op-nec-2-middle-is-universal',
    chunkId: 'syll-nec-2-middle-term-universality',
    label: 'Middle term = objective universality',
    clauses: [
      'middleTerm.is = objectiveUniversality',
      'objectiveUniversality.positedInAndForItself = true'
    ],
    predicates: [
      { name: 'IsObjectiveUniversality', args: ['middleTerm'] },
      { name: 'IsPositedInAndForItself', args: ['objectiveUniversality'] }
    ],
    relations: [
      { predicate: 'identifies', from: 'middleTerm', to: 'objectiveUniversality' }
    ]
  },

  {
    id: 'syll-op-nec-3-substance-as-universal',
    chunkId: 'syll-nec-3-substance-as-universal',
    label: 'Substance considered as universal (conceptual elevation)',
    clauses: [
      'substance.asConcept = universal',
      'modeOfBeing(substance) = determinationOfConcept (not accidentality)'
    ],
    predicates: [
      { name: 'IsUniversalAsConcept', args: ['substance'] },
      { name: 'HasModeOfBeing', args: ['substance','determinationOfConcept'] }
    ],
    relations: [
      { predicate: 'grounds', from: 'substance', to: 'determinationOfConcept' }
    ]
  },

  {
    id: 'syll-op-nec-4-univ-vs-sing',
    chunkId: 'syll-nec-4-universality-and-singularity',
    label: 'Differentiate universality (abstract) and singularity (concrete unity)',
    clauses: [
      'universality = abstractDeterminateness (specificDifference)',
      'singularity = concreteUnity(genus, determinateness)'
    ],
    predicates: [
      { name: 'IsAbstractDeterminateness', args: ['universality'] },
      { name: 'IsConcreteUnity', args: ['singularity'] }
    ],
    relations: [
      { predicate: 'contrastsWith', from: 'universality', to: 'singularity' }
    ]
  },

  {
    id: 'syll-op-nec-5-premises-are-categorical',
    chunkId: 'syll-nec-5-premises-categorical',
    label: 'Both premises are categorical judgments (connections to middle term)',
    clauses: [
      'extreme.connectsTo(middle) => categoricalJudgment',
      'bothExtremesExpressSpecificDifference => bothPremisesCategorical'
    ],
    predicates: [
      { name: 'IsCategoricalJudgment', args: ['proposition'] },
      { name: 'ExpressesSpecificDifference', args: ['extreme'] }
    ],
    relations: [
      { predicate: 'connects', from: 'extreme', to: 'middleTerm' },
      { predicate: 'makes', from: 'extremeExpression', to: 'premiseCategorical' }
    ]
  },

  // appended: part 2 HLOs
  {
    id: 'syll-op-nec-6-specific-difference-ground',
    chunkId: 'syll-nec-6-specific-difference-as-ground',
    label: 'Specific difference grounds predication',
    clauses: [
      'specificDifference = abstractDeterminatenessOfSubstance',
      'predicate.followsFrom(subject) iff subject.concept.includes(specificDifference)'
    ],
    predicates: [
      { name: 'IsSpecificDifference', args: ['difference'] },
      { name: 'GroundsPredication', args: ['specificDifference','predicate'] }
    ],
    relations: [
      { predicate: 'grounds', from: 'specificDifference', to: 'predication' }
    ]
  },

  {
    id: 'syll-op-nec-7-major-minor-categorical',
    chunkId: 'syll-nec-7-major-minor-structure',
    label: 'Major = universal determinateness; minor = singular connection',
    clauses: [
      'major.expresses = universalDeterminateness',
      'minor.connects = singular -> determinateness',
      'chain => necessity'
    ],
    predicates: [
      { name: 'ExpressesUniversal', args: ['major'] },
      { name: 'ConnectsSingular', args: ['minor'] }
    ],
    relations: [
      { predicate: 'yields', from: 'major+minor', to: 'consequenceNecessity' }
    ]
  },

  {
    id: 'syll-op-nec-8-necessity-grounded',
    chunkId: 'syll-nec-8-necessity-and-substance',
    label: 'Necessity: predicate grounded in subject notion (substance)',
    clauses: [
      'necessity = predicateGroundedInConcept(subject)',
      'if subject.concept.includes(predicate then predicate.necessary'
    ],
    predicates: [
      { name: 'IsNecessary', args: ['predicate'] },
      { name: 'IsGroundedInConcept', args: ['predicate','subject'] }
    ],
    relations: [
      { predicate: 'followsFrom', from: 'predicate', to: 'subject.concept' }
    ]
  },

  // appended: part 3 HLOs
  {
    id: 'syll-op-nec-9-subjective-element',
    chunkId: 'syll-nec-9-subjective-element',
    label: 'Subjective element: identity of content vs form',
    clauses: [
      'identity.contentual = true',
      'identity.ofForm = false',
      'necessity = connection(innerBond)'
    ],
    predicates: [
      { name: 'IsContentualIdentity', args: ['identity'] },
      { name: 'HasInnerBond', args: ['concept'] }
    ],
    relations: [
      { predicate: 'isNot', from: 'identity', to: 'identityOfForm' },
      { predicate: 'grounds', from: 'innerBond', to: 'necessity' }
    ]
  },

  {
    id: 'syll-op-nec-10-singular-contingency',
    chunkId: 'syll-nec-10-singular-contingency',
    label: 'Singular subsumed; contingency of selection among singulars',
    clauses: [
      'singular.isImmediate = true',
      'manySingulars.subsumedUnder(genus)',
      'choiceOf(singular) = contingent'
    ],
    predicates: [
      { name: 'IsImmediateSingular', args: ['singular'] },
      { name: 'IsContingentChoice', args: ['singular'] }
    ],
    relations: [
      { predicate: 'subsumes', from: 'genus', to: 'singulars' },
      { predicate: 'renders', from: 'objectivityUniversality', to: 'singularAsSubjectiveActuality' }
    ]
  },

  {
    id: 'syll-op-nec-11-extremes-immediacy-indifference',
    chunkId: 'syll-nec-11-extremes-immediacy-and-indifference',
    label: 'Extremes possess independent determinations and concrete existence',
    clauses: [
      'extreme.contains = determinationsNotInMiddle',
      'extreme.hasConcreteExistence = true',
      'extremes.indifferentToEachOther = true'
    ],
    predicates: [
      { name: 'HasIndependentDeterminations', args: ['extreme'] },
      { name: 'IsConcreteExistence', args: ['extreme'] }
    ],
    relations: [
      { predicate: 'isIndependentFrom', from: 'extreme', to: 'middleTerm' },
      { predicate: 'isIndifferentTo', from: 'extremeA', to: 'extremeB' }
    ]
  },

  {
    id: 'syll-op-nec-12-formal-identity-transition',
    chunkId: 'syll-nec-12-formal-identity-and-transition',
    label: 'Formal inner identity leads to determination toward hypothetical form',
    clauses: [
      'identity.isFormalInner = true',
      'if identity.onlyFormal then syllogism.determinesTo(hypothetical)'
    ],
    predicates: [
      { name: 'IsFormalInnerIdentity', args: ['identity'] },
      { name: 'DeterminesTo', args: ['syllogism','hypothetical'] }
    ],
    relations: [
      { predicate: 'determines', from: 'categoricalNecessity', to: 'hypotheticalSyllogism' }
    ]
  }
]
