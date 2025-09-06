import type { Chunk, LogicalOperation } from './index'

/**
 * Reflection — B. Difference : 2. Diversity (part 1)
 * Chunking + logical operations for the passage on diversity as internalized difference.
 */

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'div-1-overview',
    title: 'Diversity — overview',
    text: `Identity internally breaks apart into diversity because absolute difference posits itself as the negative of itself. The two moments (itself and its negative) are reflections into themselves and remain identical with themselves, yielding diversity.`
  },
  {
    id: 'div-2-substance-of-diversity',
    title: 'Diversity as subsistence of the negative',
    text: `The different subsists as diverse, indifferent to others, because identity constitutes its base. The diverse remains itself even in its opposite; diversity constitutes the otherness proper to reflection rather than the otherness of mere existence.`
  },
  {
    id: 'div-3-moments-self-referring',
    title: 'Moments of difference are self‑referring',
    text: `The moments of difference — identity and difference itself — are diverse when each is reflected into itself and refers only to itself. In this structure the two moments are not differentiated with respect to each other; their difference becomes external.`
  },
  {
    id: 'div-4-externality-and-indifference',
    title: 'Externality, indifference, and mutual conduct',
    text: `Because the moments are self‑referring and indifferent, they conduct themselves as moments different in general. They are indifferent to each other and to their determinateness, and difference thus appears as external in relation to them.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'div-op-1-declare-breakup',
    chunkId: 'div-1-overview',
    label: 'Declare identity breaking into diversity',
    clauses: [
      'identity.sublates -> diversityOccurs',
      'diversity = negativeOfSelf ∧ selfReflected'
    ],
    predicates: [{ name: 'IsDiversityFromIdentity', args: [] }],
    relations: [{ predicate: 'emergesFrom', from: 'diversity', to: 'identitySublation' }]
  },
  {
    id: 'div-op-2-substance-formalize',
    chunkId: 'div-2-substance-of-diversity',
    label: 'Formalize diversity as subsistence of the negative with base in identity',
    clauses: [
      'diverse.subsists = true',
      'base(identity) => diverse.identityPreserved',
      'otherness.reflection != otherness.existence'
    ],
    predicates: [{ name: 'IsDiverseSubsistence', args: [] }],
    relations: [{ predicate: 'contrasts', from: 'reflectiveOtherness', to: 'existentialOtherness' }]
  },
  {
    id: 'div-op-3-self-referring-moments',
    chunkId: 'div-3-moments-self-referring',
    label: 'Encode the self‑referring nature of the moments (identity, difference)',
    clauses: [
      'for each moment in {identity,difference}: moment.refersTo = itself',
      'not(determinedWithRespectToOther) => externalDifference'
    ],
    predicates: [{ name: 'HasSelfReferringMoments', args: [] }],
    relations: [{ predicate: 'yields', from: 'selfReferringMoments', to: 'externalityOfDifference' }]
  },
  {
    id: 'div-op-4-externality-behavior',
    chunkId: 'div-4-externality-and-indifference',
    label: 'State behavior: indifference leads to external relation between moments',
    clauses: [
      'moments.indifferentToEachOther = true',
      'conduct(asGeneralMoments) => noInternalDifferentiation',
      'externalDifference => momentsInteractAsDiverse'
    ],
    predicates: [{ name: 'IsExternalDifferenceBehavior', args: [] }],
    relations: [{ predicate: 'explains', from: 'indifference', to: 'diverseConduct' }]
  }
]

/* appended: Diversity — part 2 (externality, immanent vs external reflection, likeness/unlikeness) */
CANONICAL_CHUNKS.push(
  {
    id: 'div-5-positedness-and-externality',
    title: 'Positedness and externality in diversity',
    text: `In diversity, reflection becomes generally external: difference is a positedness and yet encompasses the whole reflection. Identity and difference are reflections that are themselves wholes; determinateness as only identity or only difference is a sublated positedness.`
  },
  {
    id: 'div-6-duplicity-immanent-negation',
    title: 'Duplicity: immanent reflection and determinateness as negation',
    text: `There is a duplicity: immanent reflection as such, and determinateness as negation or positedness. Positedness is reflection external to itself — self‑referring implicitly, but referring to something external in its positedness.`
  },
  {
    id: 'div-7-immanent-vs-external-reflection',
    title: 'Immanent versus external reflection (moments of difference)',
    text: `Reflection-in-itself (immanent) and external reflection are the two determinations in which identity and difference are posited. Immanent reflection is identity made indifferent to difference (diversity); external reflection is determinate difference and likeness/unlikeness as posited relations dependent on a third.`
  },
  {
    id: 'div-8-likeness-and-unlikeness',
    title: 'Likeness and unlikeness as external determinations',
    text: `External identity is likeness and external difference is unlikeness: identity/unlikeness are posited relations, not properties in and for themselves. Whether something is like or unlike another depends on a third external standpoint.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'div-op-5-positedness',
    chunkId: 'div-5-positedness-and-externality',
    label: 'Formalize positedness and external reflection in diversity',
    clauses: [
      'diversity.reflection = externalInGeneral',
      'difference = positedness ∧ wholeReflection',
      'determinateness(identity|difference) = sublatedPositedness'
    ],
    predicates: [{ name: 'IsPositedDiversity', args: [] }],
    relations: [{ predicate: 'constitutes', from: 'positedness', to: 'diversity' }]
  },
  {
    id: 'div-op-6-duplicity',
    chunkId: 'div-6-duplicity-immanent-negation',
    label: 'Encode duplicity: immanent reflection vs determinateness as negation',
    clauses: [
      'duplicity = {immanentReflection, determinatenessAsNegation}',
      'positedness.refersTo = externalSomething',
      'immanentReflection.implicitSelfReference = true'
    ],
    predicates: [{ name: 'HasDuplicity', args: [] }],
    relations: [{ predicate: 'distinguishes', from: 'immanentReflection', to: 'positedness' }]
  },
  {
    id: 'div-op-7-immanent-vs-external',
    chunkId: 'div-7-immanent-vs-external-reflection',
    label: 'Differentiate immanent and external reflection and their roles',
    clauses: [
      'reflection.inItself = immanentIdentity',
      'externalReflection = determinateDifferenceOrLikeness',
      'identity.asOneReflection => containsDifferenceAsIndifferent'
    ],
    predicates: [{ name: 'DistinguishReflections', args: [] }],
    relations: [{ predicate: 'maps', from: 'immanentReflection', to: 'identityRole' }]
  },
  {
    id: 'div-op-8-likeness-unlikeness',
    chunkId: 'div-8-likeness-and-unlikeness',
    label: 'Formalize likeness/unlikeness as external, standpoint-relative relations',
    clauses: [
      'likeness = positedIdentity (not identityInAndForItself)',
      'unlikeness = positedDifference (not differenceInAndForItself)',
      'relationalTruth(like|unlike) => dependsOn(thirdStandpoint)'
    ],
    predicates: [{ name: 'IsStandpointRelativeRelation', args: ['likeness','unlikeness'] }],
    relations: [{ predicate: 'dependsOn', from: 'likeness/unlikeness', to: 'externalStandpoint' }]
  }
)

/* appended: Diversity — part 3 (external reflection, likeness/unlikeness, oscillation, sublation → opposition) */
CANONICAL_CHUNKS.push(
  {
    id: 'div-9-external-connecting',
    title: 'External reflection connects diversity to likeness/unlikeness',
    text: `External reflection refers diversity to likeness and unlikeness by comparison. This comparing moves back and forth between likeness and unlikeness, but that movement is external to the determinations themselves: each is referred only to a third standpoint, not to each other.`
  },
  {
    id: 'div-10-oscillation-comparing',
    title: 'Oscillation of comparing (likeness ↔ unlikeness)',
    text: `The alternation of likeness and unlikeness makes each stand out independently; external reflection is external to itself. Determinate difference is negated absolute difference and has its reflection outside itself, so the moments come apart and refer externally to the immanent reflection confronting them.`
  },
  {
    id: 'div-11-separation-and-destruction',
    title: 'Separation of likeness/unlikeness and their self‑destruction',
    text: `When likeness and unlikeness are kept apart as external viewpoints, the very keeping-apart undermines them: each is meant to be a reference to the other, but their indifference makes them self-referential and so identical with themselves, dissolving real difference.`
  },
  {
    id: 'div-12-negative-unity',
    title: 'Negative unity of the comparing subject',
    text: `The comparing subject that oscillates between likeness and unlikeness is the negative unity of both; by letting one disappear into the other it is the subjective negative that transcends what is compared, and this negative unity becomes the nature of likeness/unlikeness themselves.`
  },
  {
    id: 'div-13-implicit-reflection-return',
    title: 'Implicit reflection and return into negative unity → opposition',
    text: `Likeness and unlikeness are implicitly reflected (self-reference without negation) and thus collapse back into a negative unity. The diverse, through this indifference and implicit reflection, passes into negative reflection: the result is opposition.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'div-op-9-external-connect',
    chunkId: 'div-9-external-connecting',
    label: 'Formalize external comparison linking diversity to likeness/unlikeness',
    clauses: [
      'externalReflection.compare(diverse) => {likeness, unlikeness}',
      'compare.operation = oscillation(likeness,unlikeness)',
      'likeness.refersTo = thirdStandpoint',
      'unlikeness.refersTo = thirdStandpoint'
    ],
    predicates: [{ name: 'IsExternalComparison', args: [] }],
    relations: [{ predicate: 'refersTo', from: 'likeness/unlikeness', to: 'thirdStandpoint' }]
  },
  {
    id: 'div-op-10-oscillation',
    chunkId: 'div-10-oscillation-comparing',
    label: 'Encode oscillation and externality of reflection',
    clauses: [
      'oscillation = move(likeness -> unlikeness -> likeness)',
      'oscillation.externalTo(determinations) = true',
      'determinateDifference = negatedAbsoluteDifference'
    ],
    predicates: [{ name: 'IsOscillatoryComparing', args: [] }],
    relations: [{ predicate: 'externalizes', from: 'oscillation', to: 'determinations' }]
  },
  {
    id: 'div-op-11-separation-sublates',
    chunkId: 'div-11-separation-and-destruction',
    label: 'Model separation causing self-referential collapse and sublation',
    clauses: [
      'if likeness.indifferentTo(unlikeness) then likeness -> selfReference',
      'selfReference => lossOfOpposition',
      'separation.subsumes -> sublationOfBoth'
    ],
    predicates: [{ name: 'SeparationCausesCollapse', args: [] }],
    relations: [{ predicate: 'yields', from: 'selfReference', to: 'lossOfOpposition' }]
  },
  {
    id: 'div-op-12-negative-unity-formalize',
    chunkId: 'div-12-negative-unity',
    label: 'Formalize comparing-subject as negative unity that transcends moments',
    clauses: [
      'comparingSubject.oscillates => negativeUnity',
      'negativeUnity.transforms(likeness,unlikeness) => mutualIdentity',
      'negativeUnity.initially = subjectiveOperationOutsideMoments'
    ],
    predicates: [{ name: 'IsNegativeUnity', args: [] }],
    relations: [{ predicate: 'transforms', from: 'negativeUnity', to: 'likeness/unlikeness' }]
  },
  {
    id: 'div-op-13-implicit-reflection-return',
    chunkId: 'div-13-implicit-reflection-return',
    label: 'Encode implicit reflection collapsing into negative unity → opposition',
    clauses: [
      'implicitReflection = selfReferenceWithoutNegation',
      'implicitReflection + indifference => collapseIntoNegativeUnity',
      'result = opposition (diversity as negative unity)'
    ],
    predicates: [{ name: 'IsImplicitReflectionCollapse', args: [] }],
    relations: [{ predicate: 'produces', from: 'implicitReflection', to: 'opposition' }]
  }
)
