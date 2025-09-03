import type { Chunk, LogicalOperation } from './index'

/**
 * Disjunctive syllogism — chunkified + HLO extraction
 * (third/last file for the syllogisms of necessity)
 */

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'syll-dis-1-schema',
    title: 'Disjunctive syllogism — schema (S-U-P)',
    text: `The disjunctive syllogism falls under the third figure S-U-P: the middle term is a universality developed into a totality that contains particularity and singularity (A is one of B,C,D).`
  },
  {
    id: 'syll-dis-2-middle-developed-universal',
    title: 'Middle term as developed objective universality',
    text: `The middle term is a developed universality: substantial identity of the genus that includes particularity and singularity as its species — a universal sphere containing its total particularity.`
  },
  {
    id: 'syll-dis-3-particularization-and-either-or',
    title: 'Particularization = differentiation = either-or (negative unity)',
    text: `Particularization is differentiation and takes the form of an either-or: B, C, D are mutually exclusive parts (negative unity). Each particular is self-referring singularity excluding the others.`
  },
  {
    id: 'syll-dis-4-syllogism-forms-examples',
    title: 'Form examples (two readings)',
    text: `Forms: "A is either B or C or D; A is B; therefore A is not C or D." Or: "A is either B or C or D; A is not C nor D; therefore A is B."`
  },
  {
    id: 'syll-dis-5-role-of-subject-across-premises',
    title: 'Subject A across premises and conclusion',
    text: `A appears as subject in both premises and the conclusion: as universal in the disjunction, as determinate species in the minor, and as excluding singularity in the conclusion (or vice versa when minor asserts exclusion).`
  },
  {
    id: 'syll-dis-6-exclusion-and-posited-singularity',
    title: 'Exclusion as negation; conclusion posits singular determinateness',
    text: `The conclusion performs an excluding operation (negation of alternatives) and thereby positively posits A's singular determinateness — the universal sphere is transformed into an exclusive singular determination.`
  }
]

// appended: transition — subjectivity → objectivity; unity of mediator & mediated
CANONICAL_CHUNKS.push(
  {
    id: 'syll-dis-7-transition-subjectivity-objectivity',
    title: 'Transition: from subjectivity to objectivity',
    text: `Consequently, what appears as mediated is the universality of A with the singularity. The mediating means is A as the universal sphere of its particularizations, determined as a singular.`
  },
  {
    id: 'syll-dis-8-mediator-unity',
    title: 'Mediator = unity of mediator and mediated',
    text: `The disjunctive syllogism posits the truth of the hypothetical: the unity of mediator and mediated. The middle term contains both extremes in complete determinateness, so the extremes lose autonomous determinateness.`
  },
  {
    id: 'syll-dis-9-middle-contains-extremes',
    title: 'Middle term contains extremes; extremes reduced to positedness',
    text: `The middle term, as totality of the concept, includes the extremes fully. The extremes, distinct from this middle, remain only as positedness without proper determinateness against the middle.`
  },
  {
    id: 'syll-dis-10-form-identity',
    title: 'Form identical with content — determinate difference + simple identity',
    text: `The disjunctive determination posits form and content as identical: the form-determination of the concept is present in determinate difference and in the simple identity of the concept; the external negative unity is assimilated into the solid content.`
  }
)

// appended: concluding paragraphs — formalism sublated; mediator/mediated unity
CANONICAL_CHUNKS.push(
  {
    id: 'syll-dis-11-formalism-sublated',
    title: 'Conclusion: formalism and subjectivity sublated',
    text: `The formalism of syllogistic inference — the subjectivity of the syllogism and of the concept — has sublated itself. Previously the middle stood as an abstract determination distinct from the terms; in completion that formal distinction is overcome.`
  },
  {
    id: 'syll-dis-12-distinction-fallen-away',
    title: 'Distinction of mediating and mediated has fallen away',
    text: `When objective universality is posited as the totality of form determinations, the distinction between mediating and mediated disappears: what is mediated is itself an essential moment of what mediates it.`
  },
  {
    id: 'syll-dis-13-each-moment-is-totality',
    title: 'Each moment contains the totality of what is mediated',
    text: `In the completed syllogism each moment (mediator or mediated) is the totality of the mediated content; mediation is unity immanent to the concept rather than an external formal operation.`
  }
)

// appended: concluding transition — figures → objectivity; sublation and result
CANONICAL_CHUNKS.push(
  {
    id: 'syll-dis-14-figures-and-middle',
    title: 'Figures of the syllogism — middle as conceptual ought / totality',
    text: `The figures exhibit each determinateness of the concept singly as the middle term — the middle is the concept as an ought, i.e. the requirement that the mediating factor be the concept's totality. Different genera show stages in the repletion/concretion of the middle term.`
  },
  {
    id: 'syll-dis-15-repletion-and-concretion',
    title: 'Repletion/concretion of the middle across genera',
    text: `In the formal syllogism the middle is only posited as totality through all determinacies taken singly. In the syllogism of reflection the middle unites determinations externally. In the syllogism of necessity the middle is both developed/total and simple, sublating the form-distinction.`
  },
  {
    id: 'syll-dis-16-sublation-to-objectivity',
    title: 'Sublation of form/subjectivity → concept realized as objectivity',
    text: `With the sublation of the formal distinction the concept gains realized reality: the inwardness and externality of the concept become identical. Determinations return into the inner unity through mediation, so externality itself exhibits the concept.`
  },
  {
    id: 'syll-dis-17-syllogism-mediation-result-objectivity',
    title: 'Syllogism as mediation; result = objectivity',
    text: `The syllogism is mediation — the complete concept posited; its movement sublates mediation so nothing is in and for itself but only through mediation of another. The resulting immediacy, identical with mediation, is objectivity: a fact in and for itself.`
  }
)

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'syll-op-dis-1-declare-schema',
    chunkId: 'syll-dis-1-schema',
    label: 'Declare S-U-P schema and disjunctive shape',
    clauses: [
      'schema = S-U-P',
      'middleTerm.is = developedUniversal(includes particularity+singularity)'
    ],
    predicates: [
      { name: 'IsSchemaOf', args: ['disjunctive','S-U-P'] },
      { name: 'IsDevelopedUniversal', args: ['middleTerm'] }
    ],
    relations: [
      { predicate: 'contains', from: 'middleTerm', to: 'species(B,C,D)' }
    ]
  },

  {
    id: 'syll-op-dis-2-middle-as-totality',
    chunkId: 'syll-dis-2-middle-developed-universal',
    label: 'Middle term as totality (universal sphere containing species)',
    clauses: [
      'middleTerm.totality = genus + speciesSet',
      'speciesSet = {B,C,D} where each is a particularization'
    ],
    predicates: [
      { name: 'IsTotality', args: ['middleTerm'] },
      { name: 'HasSpeciesSet', args: ['middleTerm','{B,C,D}'] }
    ],
    relations: [
      { predicate: 'partitions', from: 'middleTerm', to: 'speciesSet' }
    ]
  },

  {
    id: 'syll-op-dis-3-particularization-either-or',
    chunkId: 'syll-dis-3-particularization-and-either-or',
    label: 'Particularization = differentiation = either-or (reciprocal exclusion)',
    clauses: [
      'particularization => differentiation',
      'differentiation => disjunction(either B or C or D)',
      'disjunction => mutualExclusion(B,C,D)'
    ],
    predicates: [
      { name: 'IsDifferentiation', args: ['particularization'] },
      { name: 'IsMutuallyExclusive', args: ['B','C','D'] }
    ],
    relations: [
      { predicate: 'excludes', from: 'B', to: 'C+D' },
      { predicate: 'excludes', from: 'C', to: 'B+D' }
    ]
  },

  {
    id: 'syll-op-dis-4-form-variants',
    chunkId: 'syll-dis-4-syllogism-forms-examples',
    label: 'Disjunctive inference variants (two standard forms)',
    clauses: [
      'formA: (A either B or C or D) && (A is B) => (A not C && A not D)',
      'formB: (A either B or C or D) && (A not C && A not D) => (A is B)'
    ],
    predicates: [
      { name: 'IsDisjunctiveForm', args: ['formA|formB'] },
      { name: 'InfersExclusion', args: ['minor','conclusion'] }
    ],
    relations: [
      { predicate: 'derives', from: 'minorAssertion', to: 'exclusionConclusion' }
    ]
  },

  {
    id: 'syll-op-dis-5-subject-roles',
    chunkId: 'syll-dis-5-role-of-subject-across-premises',
    label: 'A as universal (disjunction), species (minor) and excluded singular (conclusion)',
    clauses: [
      'A.inFirstPremise = universalDisjunction',
      'A.inMinor = determinateSpecies',
      'A.inConclusion = excludingSingularity'
    ],
    predicates: [
      { name: 'PlaysRole', args: ['A','universal|species|singular'] }
    ],
    relations: [
      { predicate: 'transformsRole', from: 'A', to: 'conclusionRole' }
    ]
  },

  {
    id: 'syll-op-dis-6-exclusion-posits-singularity',
    chunkId: 'syll-dis-6-exclusion-and-posited-singularity',
    label: 'Exclusion is negation that yields positive singular determinateness',
    clauses: [
      'exclusion = negationOf(alternatives)',
      'negationOf(alternatives) => positivePositingOf(A.asSingularity)'
    ],
    predicates: [
      { name: 'IsNegationOfAlternatives', args: ['exclusion'] },
      { name: 'PositsSingularity', args: ['A'] }
    ],
    relations: [
      { predicate: 'yields', from: 'exclusion', to: 'positiveSingularity' }
    ]
  },

  {
    id: 'syll-op-dis-7-transition-subjectivity-objectivity',
    chunkId: 'syll-dis-7-transition-subjectivity-objectivity',
    label: 'Transition: mediator A = universal sphere determined as singular',
    clauses: [
      'mediatedAppearance = universality(A) + singularity',
      'mediatingMeans = A.asUniversalSphere.determinedAsSingular'
    ],
    predicates: [
      { name: 'IsMediatingMeans', args: ['A'] },
      { name: 'Combines', args: ['universality','singularity'] }
    ],
    relations: [
      { predicate: 'mediatesAs', from: 'A', to: 'unityOfDeterminations' }
    ]
  },

  {
    id: 'syll-op-dis-8-mediator-unity',
    chunkId: 'syll-dis-8-mediator-unity',
    label: 'Disjunctive posits unity of mediator and mediated; extremes lose autonomy',
    clauses: [
      'disjunctive.posits = truthOf(hypothetical)',
      'middle.contains(extremes.completeDeterminateness) => extremes.autonomy = false'
    ],
    predicates: [
      { name: 'PositsTruthOf', args: ['disjunctive','hypothetical'] },
      { name: 'ContainsFully', args: ['middle','extremes'] }
    ],
    relations: [
      { predicate: 'renders', from: 'middleTerm', to: 'extremes.positedness' }
    ]
  },

  {
    id: 'syll-op-dis-9-middle-contains-extremes',
    chunkId: 'syll-dis-9-middle-contains-extremes',
    label: 'Middle as totality; extremes remain only as positedness',
    clauses: [
      'middle.totality = fullConceptualDetermination',
      'extremes.asDistinct = merePositednessWithoutProperDeterminateness'
    ],
    predicates: [
      { name: 'IsFullTotality', args: ['middle'] },
      { name: 'IsMerePositedness', args: ['extreme'] }
    ],
    relations: [
      { predicate: 'internalizes', from: 'middle', to: 'extremes' }
    ]
  },

  {
    id: 'syll-op-dis-10-form-identity',
    chunkId: 'syll-dis-10-form-identity',
    label: 'Form and content identical: determinate difference + simple identity',
    clauses: [
      'formDetermination = determinateDifference + simpleIdentity',
      'form.identityWithContent => externalNegativeUnity.assimilated'
    ],
    predicates: [
      { name: 'IsDeterminateDifference', args: ['form'] },
      { name: 'IsSimpleIdentity', args: ['concept'] }
    ],
    relations: [
      { predicate: 'identifies', from: 'form', to: 'content' },
      { predicate: 'assimilates', from: 'form', to: 'externalNegativeUnity' }
    ]
  },

  {
    id: 'syll-op-dis-11-formalism-sublated',
    chunkId: 'syll-dis-11-formalism-sublated',
    label: 'Formal/subjective factor is sublated in completion',
    clauses: [
      'formalism = middle.abstractAndDistinctFromTerms',
      'completion => formalism.sublated'
    ],
    predicates: [
      { name: 'IsFormalFactor', args: ['middle'] },
      { name: 'IsSublated', args: ['formalism'] }
    ],
    relations: [
      { predicate: 'isOvercomeBy', from: 'completionOfSyllogism', to: 'formalism' }
    ]
  },

  {
    id: 'syll-op-dis-12-distinction-fallen-away',
    chunkId: 'syll-dis-12-distinction-fallen-away',
    label: 'Mediator/mediated distinction collapses into unity',
    clauses: [
      'if objectiveUniversality = totalityOfForm then mediator/mediated.distinction = false',
      'mediated ∈ mediator.moments'
    ],
    predicates: [
      { name: 'IsTotality', args: ['objectiveUniversality'] },
      { name: 'IncludesAsMoment', args: ['mediator','mediated'] }
    ],
    relations: [
      { predicate: 'collapses', from: 'distinction', to: 'unity' }
    ]
  },

  {
    id: 'syll-op-dis-13-each-moment-totality',
    chunkId: 'syll-dis-13-each-moment-is-totality',
    label: 'Each moment bears the totality of what it mediates',
    clauses: [
      'for each moment m: m.contains = totalityOfMediatedContent',
      'mediation = immanentUnity(form, content)'
    ],
    predicates: [
      { name: 'ContainsTotality', args: ['moment'] },
      { name: 'IsImmanentUnity', args: ['mediation'] }
    ],
    relations: [
      { predicate: 'grounds', from: 'moment', to: 'totality' }
    ]
  },

  {
    id: 'syll-op-dis-14-figures-and-middle',
    chunkId: 'syll-dis-14-figures-and-middle',
    label: 'Figures expose determinateness as middle; middle = ought/totality',
    clauses: [
      'figure.exhibits(determinateness) => determinateness.asMiddle',
      'middle.is = conceptAsOught',
      'genera.reflect = stagesOfRepletion'
    ],
    predicates: [
      { name: 'ExhibitsAsMiddle', args: ['figure','determinateness'] },
      { name: 'IsConceptOught', args: ['middle'] }
    ],
    relations: [
      { predicate: 'stages', from: 'genera', to: 'repletion' }
    ]
  },

  {
    id: 'syll-op-dis-15-repletion-and-concretion',
    chunkId: 'syll-dis-15-repletion-and-concretion',
    label: 'Middle repletion across syllogism types; necessity sublates form-distinction',
    clauses: [
      'formal.middle.positedOnlyVia(allDeterminacies)',
      'reflection.middle.gathersExternally',
      'necessity.middle = developed + simple -> sublation(formDistinction)'
    ],
    predicates: [
      { name: 'IsPositedViaAll', args: ['middle','determinacies'] },
      { name: 'GathersExternally', args: ['middle'] }
    ],
    relations: [
      { predicate: 'sublates', from: 'necessityMiddle', to: 'formDistinction' }
    ]
  },

  {
    id: 'syll-op-dis-16-sublation-to-objectivity',
    chunkId: 'syll-dis-16-sublation-to-objectivity',
    label: 'Sublation yields identity of inwardness and externality; concept realized',
    clauses: [
      'sublation => inwardness == externality',
      'determinations.returnInto(innerUnity).via(mediation)'
    ],
    predicates: [
      { name: 'IsRealized', args: ['concept'] },
      { name: 'ReturnsInto', args: ['determinations','innerUnity'] }
    ],
    relations: [
      { predicate: 'exhibits', from: 'externality', to: 'concept' }
    ]
  },

  {
    id: 'syll-op-dis-17-syllogism-mediation-result-objectivity',
    chunkId: 'syll-dis-17-syllogism-mediation-result-objectivity',
    label: 'Syllogism = mediation; result is objectivity (fact in and for itself)',
    clauses: [
      'syllogism = completeConcept.posited',
      'movement = sublationOfMediation',
      'result = immediacyIdenticalWithMediation => objectivity'
    ],
    predicates: [
      { name: 'IsCompleteConcept', args: ['syllogism'] },
      { name: 'IsObjectivity', args: ['result'] }
    ],
    relations: [
      { predicate: 'yields', from: 'sublation', to: 'objectivity' }
    ]
  }
]
