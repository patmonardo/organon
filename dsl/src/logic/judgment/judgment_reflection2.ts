import type { Chunk, LogicalOperation } from './index'

/* Judgment of Reflection — c. The universal judgment (part 1)
   - universality as "allness" of singulars via comparison (external universality)
   - distinction between mere plurality and true universality (method/rule as true universal)
   - warning against bad-infinity conception of universality
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-ref-8-universal-overview',
    title: 'Universal judgment — overview (external universality / allness)',
    text: `The universality of the subject in the universal judgment is the external universality of reflection — "allness" — i.e. the commonality of self‑subsisting singulars gathered by comparison. This allness leaves the singular itself unchanged.`
  },
  {
    id: 'j-ref-9-universality-as-commonality',
    title: 'Universality as commonality of singulars (comparison)',
    text: `This universality is a commonality arrived at by associating many singulars through comparison. That subjective association is the ordinary notion of universality: a determination is called universal because it fits many cases.`
  },
  {
    id: 'j-ref-10-method-rule-as-true-universal',
    title: 'Method / rule as the true universal (not mere plurality)',
    text: `The true universal is the underlying method or rule, not the mere multiplicity of instances. Increased plurality (more terms) does not increase universality if the same rule repeats; the rule (method) is the genuine universal content of the concept.`
  },
  {
    id: 'j-ref-11-bad-infinity-warning',
    title: 'Bad infinity and the deception of allness-as-totality',
    text: `Mistaking plurality (or an unattainable exhausted "all") for universality leads to bad infinity. True universality is the concept in-and-for-itself which transcends mere enumerative allness and the deceptive progression into infinite particularity.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'j-ref-op-8-declare-external-universal',
    chunkId: 'j-ref-8-universal-overview',
    label: 'Declare external universality (allness) and its character',
    clauses: [
      'universal.kind = externalAllness',
      'allness = associationOf(selfSubsistingSingulars)',
      'singular.remains = unchangedWithinAllness'
    ],
    predicates: [{ name: 'IsExternalUniversality', args: [] }],
    relations: [{ predicate: 'characterizes', from: 'universal', to: 'allness' }]
  },
  {
    id: 'j-ref-op-9-detect-comparison-based-universality',
    chunkId: 'j-ref-9-universality-as-commonality',
    label: 'Flag universality derived by comparison (plural instantiation)',
    clauses: [
      'if universality.derivedBy == comparison then tag(universal, "comparison-based")',
      'comparisonBased.universal -> mayBeOnlyParticularityDisguised'
    ],
    predicates: [{ name: 'DetectsComparisonUniversal', args: [] }],
    relations: [{ predicate: 'tags', from: 'detector', to: 'comparisonBasedUniversals' }]
  },
  {
    id: 'j-ref-op-10-elevate-method-rule',
    chunkId: 'j-ref-10-method-rule-as-true-universal',
    label: 'Promote method/rule as the genuine universal over plurality',
    clauses: [
      'if instances.share(sameMethod) then method = trueUniversal',
      'pluralityWithoutNewMethod -> remainsParticularity'
    ],
    predicates: [{ name: 'PromotesMethodAsUniversal', args: [] }],
    relations: [{ predicate: 'elevates', from: 'method', to: 'universalStatus' }]
  },
  {
    id: 'j-ref-op-11-flag-bad-infinity',
    chunkId: 'j-ref-11-bad-infinity-warning',
    label: 'Detect and flag bad-infinity / allness-deception',
    clauses: [
      'if universality.aimsAt(exhaustiveEnumeration) then tag(universal,"bad-infinity")',
      'badInfinity => recommend(reflectOnConceptAsInItself)'
    ],
    predicates: [{ name: 'FlagsBadInfinity', args: [] }],
    relations: [{ predicate: 'recommends', from: 'detector', to: 'conceptualReflection' }]
  }
]

/* appended: Judgment of Reflection — c. The universal judgment (part 2)
   - critique of empirical allness; distinction between plurality and true universality
   - subject already contains achieved universality; "all humans" -> "the human being"
   - reflection makes implicit universality explicit (objective universality)
*/
CANONICAL_CHUNKS.push(
  {
    id: 'j-ref-12-empirical-allness-critique',
    title: 'Empirical allness vs true universality',
    text: `Empirical allness (the "all" built from many singulars) is external to the singular and therefore cannot form true unity: plurality is still particularity. Empirical universality rests on an ought (if no counterexample then treat as all), and so requires deeper conceptual reflection.`
  },
  {
    id: 'j-ref-13-subject-already-contains-universality',
    title: 'Subject contains achieved universality (presupposition)',
    text: `The subject of the universal judgment already presupposes the achieved universality: "all humans" both posits the species and its singularizations. Through reflection the universality becomes determinate and equal to what was presupposed in the subject.`
  },
  {
    id: 'j-ref-14-reflection-explicit-objective-universality',
    title: 'Reflection renders implicit universality explicit — "the human being"',
    text: `Reflection that expands singularity into allness is not external: it makes explicit the implicit determinations. The result is objective universality — the subject sheds the form "all humans" and is properly expressed as "the human being."`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'j-ref-op-12-detect-empirical-allness',
    chunkId: 'j-ref-12-empirical-allness-critique',
    label: 'Detect empirical/plural-instantiated universality and flag for reflection',
    clauses: [
      'if universal.derivedBy == enumerationOrAbsenceOfCounterexample then tag(universal,"empirical-allness")',
      'empirical-allness -> recommend(conceptualReflection)'
    ],
    predicates: [{ name: 'DetectsEmpiricalAllness', args: [] }],
    relations: [{ predicate: 'recommends', from: 'detector', to: 'conceptualReflection' }]
  },
  {
    id: 'j-ref-op-13-formalize-subject-presupposition',
    chunkId: 'j-ref-13-subject-already-contains-universality',
    label: 'Formalize that subject already contains presupposed universality and singularizations',
    clauses: [
      'subject = {speciesUniversal, singularizations}',
      'allPhrase("all X") -> contains(presupposedUniversal, singularBasis)',
      'on(detectedPresupposition) => mark(node,"implicitUniversalPresent")'
    ],
    predicates: [{ name: 'MarksPresupposedUniversality', args: [] }],
    relations: [{ predicate: 'marks', from: 'analyzer', to: 'implicitUniversal' }]
  },
  {
    id: 'j-ref-op-14-promote-objective-universality',
    chunkId: 'j-ref-14-reflection-explicit-objective-universality',
    label: 'Promote reflected universality to objective form ("the X being")',
    clauses: [
      'if reflectionMakesImplicitExplicit then convert(subjectForm:"all X" -> "the X being")',
      'reclassification => subject.kind = objectiveUniversality',
      'preserve(origin:{singularBasis,particularForm})'
    ],
    predicates: [{ name: 'PromotesObjectiveUniversality', args: [] }],
    relations: [{ predicate: 'transforms', from: 'reflectionProcess', to: 'subjectReclassification' }]
  }
)

/* appended: Judgment of Reflection — c. The universal judgment (part 3)
   - genus as concrete universality; reversal of subsumption; copula -> necessity
*/
CANONICAL_CHUNKS.push(
  {
    id: 'j-ref-15-genus-concrete-universality',
    title: 'Genus — concrete universality (negative self‑identity)',
    text: `The universality arisen in reflection is the genus: a concrete universality that contains singular determinacies dissolved into substantial purity. Posited as negative self‑identity, the genus is essentially subject and no longer merely subsumed under a predicate.`
  },
  {
    id: 'j-ref-16-reversal-of-subsumption',
    title: 'Reversal of subsumption — predicate becomes particular',
    text: `When universality is determined objectively the relation of subsumption reverses: the predicate that once subsumed the subject becomes a particular, and the subject ceases to be merely an appearance under that relational grasp. Subject and predicate invert roles and the judgment is sublated.`
  },
  {
    id: 'j-ref-17-copula-sublation-and-necessity',
    title: 'Sublation of determinations into the copula — emergence of necessity',
    text: `The sublation of judgment determinations coincides with the copula's transformation: as the subject raises itself to universality it becomes identical with the predicate (in the copula). This identity is the genus/nature in itself; its inner division yields a necessary connection and grounds the new judgment — the judgment of necessity.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'j-ref-op-15-formalize-genus',
    chunkId: 'j-ref-15-genus-concrete-universality',
    label: 'Formalize genus as concrete universality (negative self‑identity)',
    clauses: [
      'genus = concreteUniversality(dissolvedSingularities)',
      'genus.role = subjectAndSelfIdentity',
      'genus.notPropertyOfSubject = true'
    ],
    predicates: [{ name: 'IsGenusConcrete', args: [] }],
    relations: [{ predicate: 'contains', from: 'genus', to: 'singularDeterminacies' }]
  },
  {
    id: 'j-ref-op-16-encode-reversal-subsumption',
    chunkId: 'j-ref-16-reversal-of-subsumption',
    label: 'Encode reversal of subsumption: predicate -> particular, subject -> universal',
    clauses: [
      'if universality.objectified then predicate.kind = particular',
      'subject.role -> becomes(universalIdentity)',
      'relation.reverse = true'
    ],
    predicates: [{ name: 'ReversesSubsumption', args: [] }],
    relations: [{ predicate: 'reverses', from: 'reflectionProcess', to: 'subsumptionRelation' }]
  },
  {
    id: 'j-ref-op-17-copula-necessity',
    chunkId: 'j-ref-17-copula-sublation-and-necessity',
    label: 'Formalize copula transformation and emergence of necessity',
    clauses: [
      'on(subject.raisedTo(universality)) => copula.role = identity',
      'identity.inCopula => termsAreUnessentialDistinctions',
      'identity.divide -> yields(innerNature) => necessityJudgmentForm'
    ],
    predicates: [{ name: 'CopulaBecomesNecessity', args: [] }],
    relations: [{ predicate: 'yields', from: 'copulaTransformation', to: 'necessityJudgment' }]
  }
)
