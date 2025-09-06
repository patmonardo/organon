import type { Chunk, LogicalOperation } from './index'

/**
 * Reflection — B. Difference : 3. Opposition (part 1)
 * Chunking + High-Level Operations for the passage on opposition:
 * - opposition = unity of identity and diversity
 * - positedness, likeness/unlikeness, positive/negative, mutual inclusion
 */

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'opp-1-overview',
    title: 'Opposition — overview',
    text: `Opposition completes determinate reflection (difference). It is the unity of identity and diversity: moments are diverse within one identity and thus stand as opposites.`
  },
  {
    id: 'opp-2-positedness-reflection',
    title: 'Positedness reflected into itself',
    text: `The moments of opposition are positedness reflected into itself. Likeness and unlikeness (external reflection) become determinations of opposition when reflected into themselves, each containing the other implicitly.`
  },
  {
    id: 'opp-3-positive-negative-definitions',
    title: 'Positive and negative as determinations',
    text: `Self‑likeness that contains reference to unlikeness is the positive; self‑unlikeness that contains reference to likeness is the negative. Both are positedness reflected in opposite modes (likeness vs unlikeness).`
  },
  {
    id: 'opp-4-mutual-inclusion',
    title: 'Mutual inclusion of opposites',
    text: `Each moment contains its other: positedness reflected into self‑likeness includes unlikeness, and positedness reflected into self‑unlikeness includes likeness. This mutual inclusion is essential to the determination of opposition.`
  },
  {
    id: 'opp-5-reflection-as-negation-of-negation',
    title: 'Negation as negation and immanent reflection',
    text: `What is reflected is positedness — negation as negation — so immanent reflection has reference to the other as its determination. Opposition thus rests on negation that refers to and includes its other.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'opp-op-1-declare-opposition',
    chunkId: 'opp-1-overview',
    label: 'Declare opposition as unity of identity + diversity',
    clauses: [
      'opposition = unity(identity, diversity)',
      'moments(opposition) = diverseWithinIdentity',
      'opposition.completes = determinateReflection'
    ],
    predicates: [{ name: 'IsOpposition', args: [] }],
    relations: [{ predicate: 'completes', from: 'opposition', to: 'determinateReflection' }]
  },
  {
    id: 'opp-op-2-positedness-reflects',
    chunkId: 'opp-2-positedness-reflection',
    label: 'Formalize positedness reflected into itself (likeness/unlikeness)',
    clauses: [
      'positedness.reflectedIntoItself = true',
      'likeness = positedIdentity',
      'unlikeness = positedDifference'
    ],
    predicates: [{ name: 'IsPositedReflection', args: [] }],
    relations: [{ predicate: 'constitutes', from: 'positedness', to: 'likeness/unlikeness' }]
  },
  {
    id: 'opp-op-3-define-pos-neg',
    chunkId: 'opp-3-positive-negative-definitions',
    label: 'Define positive and negative via internal reference to other',
    clauses: [
      'positive.contains = referenceTo(unlikeness)',
      'negative.contains = referenceTo(likeness)',
      'positive = self-likenessWithOther',
      'negative = self-unlikenessWithOther'
    ],
    predicates: [{ name: 'IsPositiveNegativePair', args: [] }],
    relations: [{ predicate: 'mutuallyIncludes', from: 'positive', to: 'negative' }]
  },
  {
    id: 'opp-op-4-mutual-inclusion-formalize',
    chunkId: 'opp-4-mutual-inclusion',
    label: 'Encode mutual inclusion: each moment contains the other',
    clauses: [
      'for each m in {positive, negative}: m.includes = other(m)',
      'mutualInclusion => whole(m) = true',
      'opposition.definedBy = mutualInclusion'
    ],
    predicates: [{ name: 'HasMutualInclusion', args: [] }],
    relations: [{ predicate: 'grounds', from: 'mutualInclusion', to: 'opposition' }]
  },
  {
    id: 'opp-op-5-negation-as-determination',
    chunkId: 'opp-5-reflection-as-negation-of-negation',
    label: 'Characterize negation-as-negation and its role',
    clauses: [
      'negation.asNegation => positedness',
      'immanentReflection.references = otherAsDetermination',
      'opposition.requires = negationReferentialStructure'
    ],
    predicates: [{ name: 'IsNegationAsNegation', args: [] }],
    relations: [{ predicate: 'enables', from: 'negation', to: 'opposition' }]
  }
]

/* appended: Opposition — part 2 (self-subsistence, mutual dependence, interchangeability, internalization) */
CANONICAL_CHUNKS.push(
  {
    id: 'opp-6-self-subsisting-sides',
    title: 'Self‑subsisting sides of opposition',
    text: `The positive and the negative become self‑subsisting because they are reflections of the whole into itself; as such the opposition they constitute is implicitly determinate. Each side is itself and its other: determinateness now lies within, not in an external other.`
  },
  {
    id: 'opp-7-mutual-dependence-and-positedness',
    title: 'Mutual dependence: each exists by virtue of the other',
    text: `The positive and negative are absolute moments of opposition; each is what it is by virtue of the other's non‑being. They are opposites in the simple sense: each is negative relative to the other and is posited only through that relation — posited moments of one mediation.`
  },
  {
    id: 'opp-8-indifference-and-interchangeability',
    title: 'Indifference, interchangeability, and reflected positedness',
    text: `As posited beings reflected into themselves, the sides show indifference to their first identity: their determinateness is general and interchangeable. Either side can be taken as positive or negative; the determinateness is not internally specific until further reflection.`
  },
  {
    id: 'opp-9-internalization-and-completion',
    title: 'Internalization: each contains positive and negative within itself',
    text: `The posited reference to the other is taken back into each side: the positive contains the negative within, and the negative contains the positive. Each becomes a self‑subsistent unity in which positedness is sublated — the opposition is completed by internalization of its other.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'opp-op-6-self-subsist-formalize',
    chunkId: 'opp-6-self-subsisting-sides',
    label: 'Formalize self-subsistence of positive/negative',
    clauses: [
      'positive.selfSubsists = true',
      'negative.selfSubsists = true',
      'opposition.implicitDeterminacy = true',
      'side.determinateness.located = within(side)'
    ],
    predicates: [{ name: 'IsSelfSubsistingSide', args: [] }],
    relations: [{ predicate: 'grounds', from: 'selfSubsistence', to: 'implicitDeterminacy' }]
  },
  {
    id: 'opp-op-7-mutual-dependence-formalize',
    chunkId: 'opp-7-mutual-dependence-and-positedness',
    label: 'Encode mutual dependence: each defined by the other\'s non-being',
    clauses: [
      'for each s in {positive,negative}: s.isByVirtueOf(other.nonBeing)',
      'both = oneMediationOfOpposition',
      'each.isNegativeWithRespectTo(other)'
    ],
    predicates: [{ name: 'IsMutualDependence', args: [] }],
    relations: [{ predicate: 'dependsOn', from: 'side', to: 'other.nonBeing' }]
  },
  {
    id: 'opp-op-8-interchangeability',
    chunkId: 'opp-8-indifference-and-interchangeability',
    label: 'Capture indifference and interchangeability of determinateness',
    clauses: [
      'positedness.reflected => sides.indifferentToFirstIdentity',
      'determinateness = general (canInterchangePositiveNegative)',
      'noInternalSpecificity => interchangeability = true'
    ],
    predicates: [{ name: 'IsInterchangeableDeterminateness', args: [] }],
    relations: [{ predicate: 'permits', from: 'interchangeability', to: 'roleSwap(positive,negative)' }]
  },
  {
    id: 'opp-op-9-internalization',
    chunkId: 'opp-9-internalization-and-completion',
    label: 'Formalize internalization: each side contains its other and sublation completes opposition',
    clauses: [
      'positive.includes = negative (internally)',
      'negative.includes = positive (internally)',
      'positedness.sublated => side.positedBeing = sublatedBeing',
      'result = opposition.completedByInternalization'
    ],
    predicates: [{ name: 'IsInternalizedOpposition', args: [] }],
    relations: [{ predicate: 'sublates', from: 'positedness', to: 'completedOpposition' }]
  }
)

/* appended: Opposition — part 3 (negative as sublated positedness; in‑itselfness of sides; exclusion & mutual determination) */
CANONICAL_CHUNKS.push(
  {
    id: 'opp-10-negative-sublated',
    title: 'Negative as sublated positedness',
    text: `The negative is not mere immediate negation but sublated positedness: the negative in and for itself positively rests upon itself. As immanent reflection it negates its reference to the other, excluding the positive from itself.`
  },
  {
    id: 'opp-11-independent-opposites',
    title: 'Independent opposite: negative vs positive',
    text: `The negative stands as an independently existing opposite over against the positive, which is the determination of the sublated opposition. The opposition rests upon itself and the negative excludes the positive as its non-being.`
  },
  {
    id: 'opp-12-in-itselfness-and-abstraction',
    title: 'In‑itselfness of sides; abstraction and determination',
    text: `Positive and negative can be considered in themselves (abstracted from excluding reference). This in‑itselfness is an immediate being/non‑being but remains a moment of opposition: their in‑itself is the form of their immanent reflectedness.`
  },
  {
    id: 'opp-13-exclusion-constitutes-in-itselfness',
    title: 'Exclusion constitutes in‑itselfness and mutual determination',
    text: `To be positive or negative in themselves implies that exclusion-reference to the other constitutes their determination. Their being in and for themselves is thus bound up with the exclusive reference that defines opposition.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'opp-op-10-negative-sublation',
    chunkId: 'opp-10-negative-sublated',
    label: 'Formalize negative as sublated positedness and exclusion',
    clauses: [
      'negative = sublatedPositedness',
      'negative.restsUponItself = true',
      'immanentReflection(negative) => excludes(positive)'
    ],
    predicates: [{ name: 'IsSublatedNegative', args: [] }],
    relations: [{ predicate: 'excludes', from: 'negative', to: 'positive' }]
  },
  {
    id: 'opp-op-11-independent-opposites',
    chunkId: 'opp-11-independent-opposites',
    label: 'Encode independent opposite relation and opposition self‑grounding',
    clauses: [
      'negative.independentExistence = true',
      'positive = determinationOfSublatedOpposition',
      'opposition.selfGrounded = true'
    ],
    predicates: [{ name: 'IsIndependentOppositePair', args: [] }],
    relations: [{ predicate: 'grounds', from: 'opposition', to: 'selfGrounding' }]
  },
  {
    id: 'opp-op-12-in-itselfness',
    chunkId: 'opp-12-in-itselfness-and-abstraction',
    label: 'Capture in‑itself abstraction and its status as a moment',
    clauses: [
      'inItself(positive|negative) = abstractImmediateBeingOrNonBeing',
      'inItself.form = immanentReflectedness',
      'inItselfness != removalOfOpposition (it remains a moment)'
    ],
    predicates: [{ name: 'IsInItselfMoment', args: [] }],
    relations: [{ predicate: 'qualifies', from: 'inItselfness', to: 'immanentReflectedness' }]
  },
  {
    id: 'opp-op-13-exclusion-constitutes',
    chunkId: 'opp-13-exclusion-constitutes-in-itselfness',
    label: 'Formalize that exclusive reference constitutes determination in and for itself',
    clauses: [
      'toBePositiveInItself => notMerelyContrastWithOther',
      'exclusiveReferenceToOther => constitutes(inItselfness)',
      'inAndForItself = determinationByExclusion'
    ],
    predicates: [{ name: 'ExclusionConstitutesDetermination', args: [] }],
    relations: [{ predicate: 'constitutes', from: 'exclusiveReference', to: 'inAndForItself' }]
  }
)
