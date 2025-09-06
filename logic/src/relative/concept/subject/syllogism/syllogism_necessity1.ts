import type { Chunk, LogicalOperation } from './index'

/**
 * Part 1 — The Hypothetical Syllogism (initial section)
 * Chunkified and HLO-extracted for later review and extension.
 */

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'syll-hyp-1-judgment-structure',
    title: 'Hypothetical judgment: necessary connection without immediacy',
    text: `The hypothetical judgment contains only the necessary connection without the immediacy of the connected terms: "If A is, so is B." It does not assert either A or B by itself.`
  },
  {
    id: 'syll-hyp-2-syllogism-adds-immediacy',
    title: 'Hypothetical syllogism adds immediacy (If A then B; A; therefore B)',
    text: `The hypothetical syllogism supplements the hypothetical judgment with the immediacy of being: the minor premise asserts A is, enabling the conclusion that B is. The conclusion is not a bare copula but the accomplished mediating unity.`
  },
  {
    id: 'syll-hyp-3-minor-as-middle',
    title: 'Minor premise and the being of A as mediating middle term',
    text: `The minor premise expresses the immediate being of A; this being of A is to be taken essentially as the middle term of the syllogism (needs closer examination).`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'syll-op-hyp-1-declare-hypothetical',
    chunkId: 'syll-hyp-1-judgment-structure',
    label: 'Declare hypothetical judgment: necessary connection without immediacy',
    clauses: [
      'judgment.form = hypothetical',
      'judgment.contains = necessaryConnection',
      'judgment.lacks = immediacyOfTerms'
    ],
    predicates: [
      { name: 'IsForm', args: ['judgment','hypothetical'] },
      { name: 'ContainsConnection', args: ['judgment','necessary'] }
    ],
    relations: [
      { predicate: 'lacks', from: 'hypotheticalJudgment', to: 'immediacyOfTerms' }
    ]
  },

  {
    id: 'syll-op-hyp-2-schema-and-conclusion',
    chunkId: 'syll-hyp-2-syllogism-adds-immediacy',
    label: 'Hypothetical syllogism schema and nature of conclusion',
    clauses: [
      'schema = {If A then B; A; therefore B}',
      'minor.asserts = immediacy(A)',
      'conclusion = mediatedUnity(A,B) (not empty copula)'
    ],
    predicates: [
      { name: 'IsSchemaOf', args: ['syllogism','hypothetical'] },
      { name: 'AssertsImmediacy', args: ['minor','A'] }
    ],
    relations: [
      { predicate: 'mediates', from: 'A', to: 'B' },
      { predicate: 'adds', from: 'minorPremise', to: 'immediacy' }
    ]
  },

  {
    id: 'syll-op-hyp-3-being-as-middle',
    chunkId: 'syll-hyp-3-minor-as-middle',
    label: 'Being of A taken as middle term (functional role)',
    clauses: [
      'being(A) := middleTermOfSyllogism',
      'conclusion.expresses = accomplishedMediatingUnity',
      'role(being(A)) = mediatingConnector(A,B)'
    ],
    predicates: [
      { name: 'IsMiddleTerm', args: ['being(A)'] },
      { name: 'ExpressesUnity', args: ['conclusion'] }
    ],
    relations: [
      { predicate: 'functionsAs', from: 'being(A)', to: 'middleTerm' },
      { predicate: 'grounds', from: 'being(A)', to: 'conclusionUnity' }
    ]
  }
]

// appended: part 2 chunks and logical operations

CANONICAL_CHUNKS.push(
  {
    id: 'syll-hyp-4-connection-necessity',
    title: 'Hypothetical — connection as necessity / inner substantial identity',
    text: `The hypothetical judgment expresses an inner substantial identity that underlies external diversity: an identical content held in necessity. The two sides are universality and singularity, not immediate beings but being held in necessity (appearances). It is indifferent which side is taken as universality or singularity; conditions are inner abstract elements that gain actuality in singularity.`
  },
  {
    id: 'syll-hyp-5-condition-vs-cause',
    title: 'Condition vs cause/ground — indifference and universality of condition',
    text: `The relation between condition and conditioned can be read as cause/effect or ground/consequence; condition is more universal because it comprehends both sides: effect can be condition of cause as much as vice versa.`
  },
  {
    id: 'syll-hyp-6-a-mediating-being',
    title: 'A as mediating being: immediacy and self-sublation',
    text: `A is mediating insofar as it is immediate actuality and insofar as it is contingent, self-sublating being. Conditions are dispersed material requiring application; their negativity (self-mediation) is the active middle term that contradicts objective universality and immediate indifference.`
  },
  {
    id: 'syll-hyp-7-conclusion-identity',
    title: 'Conclusion: mediated immediacy and identity of mediating & mediated',
    text: `The conclusion 'therefore B is' expresses B's immediate existence and mediated character: B and A share the same absolute content; the mediating term and the mediated are identical in concept, distinguished superficially by singularity vs universality.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'syll-op-hyp-4-connection-necessity',
    chunkId: 'syll-hyp-4-connection-necessity',
    label: 'Connection = inner substantial identity; sides = universality & singularity (appearance)',
    clauses: [
      'judgment.connection = innerSubstantialIdentity',
      'sides = {universality, singularity}',
      'sides.areBeingHeldInNecessity = true'
    ],
    predicates: [
      { name: 'HasInnerIdentity', args: ['judgment'] },
      { name: 'AreSides', args: ['judgment','universality|singularity'] }
    ],
    relations: [
      { predicate: 'holds', from: 'identity', to: 'appearance' },
      { predicate: 'correspondsTo', from: 'conditions', to: 'actuality' }
    ]
  },

  {
    id: 'syll-op-hyp-5-condition-vs-cause',
    chunkId: 'syll-hyp-5-condition-vs-cause',
    label: 'Condition is more universal; cause/ground are transitions (indifferent reading)',
    clauses: [
      'relation.condition <-> {ground,cause} (indifferent)',
      'condition.comprehends = bothSidesOfRelation'
    ],
    predicates: [
      { name: 'IsMoreUniversal', args: ['condition'] },
      { name: 'CanBeReadAs', args: ['relation','cause|ground'] }
    ],
    relations: [
      { predicate: 'comprehends', from: 'condition', to: 'sides' }
    ]
  },

  {
    id: 'syll-op-hyp-6-mediating-being',
    chunkId: 'syll-hyp-6-a-mediating-being',
    label: 'A as mediating being: immediacy + self-sublation; negativity as mediator',
    clauses: [
      'A.isImmediateActuality = true',
      'A.isContingentAndSelfSublating = true',
      'negativity = mediatingMeans(activity)'
    ],
    predicates: [
      { name: 'IsMediatingBeing', args: ['A'] },
      { name: 'IsSelfSublating', args: ['A'] }
    ],
    relations: [
      { predicate: 'translates', from: 'conditions', to: 'actuality' },
      { predicate: 'isContradictionOf', from: 'middleTerm', to: 'objectiveUniversality' }
    ]
  },

  {
    id: 'syll-op-hyp-7-conclusion-identity',
    chunkId: 'syll-hyp-7-conclusion-identity',
    label: 'Conclusion frames mediated immediacy; mediating term identical with mediated in concept',
    clauses: [
      'conclusion = B.isImmediate && B.isMediated',
      'absoluteContent(A) = absoluteContent(B)',
      'mediatingTerm.identity = mediated.identity (in concept)'
    ],
    predicates: [
      { name: 'IsMediatedImmediacy', args: ['B'] },
      { name: 'HasSameAbsoluteContent', args: ['A','B'] }
    ],
    relations: [
      { predicate: 'expresses', from: 'conclusion', to: 'mediatedImmediacy' },
      { predicate: 'identifies', from: 'mediatingTerm', to: 'mediated' }
    ]
  }
)

// appended: part 3 chunks and logical operations
CANONICAL_CHUNKS.push(
  {
    id: 'syll-hyp-8-form-negative-unity',
    title: 'Hypothetical part 3 — form as negative unity',
    text: `The hypothetical syllogism displays necessary connection as a connectedness through form or negative unity: the conditioning actuality is translated into the conditioned and determinacies of opposition are sublated; difference of A and B becomes an empty name.`
  },
  {
    id: 'syll-hyp-9-unity-posited-and-identity',
    title: 'Unity posited: mediated immediacy and identical content',
    text: `The unity is reflected into itself and thereby posited: the being of A is also the being of B and vice versa. Their immediate determinateness is mediated; externality is sublated and unity withdrawn into itself.`
  },
  {
    id: 'syll-hyp-10-mediation-as-differentiating-identity',
    title: 'Mediation determined as self-referring negativity / absolute form',
    text: `Mediation becomes singularity, immediacy, and self-referring negativity — a differentiating identity that retrieves itself into itself, i.e., absolute form and objective universality.`
  },
  {
    id: 'syll-hyp-11-transition-to-disjunctive',
    title: 'Transition: this determination yields the disjunctive syllogism',
    text: `In this determination the hypothetical syllogism is the disjunctive syllogism — mediation as differentiating identity issues in the disjunctive form.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'syll-op-hyp-8-form-negative-unity',
    chunkId: 'syll-hyp-8-form-negative-unity',
    label: 'Form-connectedness = negative unity; sublation of differences',
    clauses: [
      'necessity = connectednessThroughForm(negativeUnity)',
      'conditioningActuality => translatedInto(conditioned)',
      'oppositionalDeterminacies => sublated'
    ],
    predicates: [
      { name: 'IsNegativeUnity', args: ['form'] },
      { name: 'Sublates', args: ['determinacies','opposition'] }
    ],
    relations: [
      { predicate: 'translates', from: 'conditioningActuality', to: 'conditioned' },
      { predicate: 'rendersEmpty', from: 'difference(A,B)', to: 'name' }
    ]
  },

  {
    id: 'syll-op-hyp-9-unity-posited-identity',
    chunkId: 'syll-hyp-9-unity-posited-and-identity',
    label: 'Posited unity: A and B share identical absolute content',
    clauses: [
      'unity = reflectedIntoItself',
      'being(A) = being(B) (in concept)',
      'immediacyOf(A/B) := mediated'
    ],
    predicates: [
      { name: 'IsReflectedUnity', args: ['unity'] },
      { name: 'HasSameContent', args: ['A','B'] }
    ],
    relations: [
      { predicate: 'identifies', from: 'A', to: 'B' },
      { predicate: 'sublates', from: 'externality', to: 'withdrawnUnity' }
    ]
  },

  {
    id: 'syll-op-hyp-10-mediation-differentiating-identity',
    chunkId: 'syll-hyp-10-mediation-as-differentiating-identity',
    label: 'Mediation as self-referring negativity → absolute form / objective universality',
    clauses: [
      'mediation = singularity + immediacy + selfReferringNegativity',
      'mediation.retrievesDifferentiationIntoIdentity',
      'result = absoluteForm (objectiveUniversality)'
    ],
    predicates: [
      { name: 'IsMediationAsNegativity', args: ['mediation'] },
      { name: 'Becomes', args: ['mediation','absoluteForm'] }
    ],
    relations: [
      { predicate: 'retrieves', from: 'mediation', to: 'identity' },
      { predicate: 'determines', from: 'mediation', to: 'objectiveUniversality' }
    ]
  },

  {
    id: 'syll-op-hyp-11-disjunctive-transition',
    chunkId: 'syll-hyp-11-transition-to-disjunctive',
    label: 'Determination issues in disjunctive syllogism',
    clauses: [
      'if mediation.isDifferentiatingIdentity then syllogism.form -> disjunctive',
      'disjunctiveForm = wayOfExpressingWithheldUnity'
    ],
    predicates: [
      { name: 'DeterminesForm', args: ['mediation','disjunctive'] },
      { name: 'IsWayOfExpressing', args: ['disjunctive','withheldUnity'] }
    ],
    relations: [
      { predicate: 'transitionsTo', from: 'hypotheticalSyllogism', to: 'disjunctiveSyllogism' }
    ]
  }
)
