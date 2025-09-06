import type { Chunk, LogicalOperation } from './index'

/* Judgment of Necessity — c. The disjunctive judgment (part 1)
   NOTE: continuation from previous necessity parts (chunks start at j-nes-9)
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-nes-9-disjunctive-overview',
    title: 'Disjunctive judgment — overview (genus in union with form)',
    text: `The disjunctive judgment posits objective universality united with its form: the genus appears as subject and its differentiated determinations (species) as totality. "A is either B or C" expresses the concept's necessity where extremes share identity yet differ by form.`
  },
  {
    id: 'j-nes-10-particularity-as-predicate',
    title: 'Particularity as predicate and the greater universal',
    text: `The particularity developed in species constitutes the predicate because it contains the universal sphere of the subject in articulated form; a properly particularized species can present itself as the larger universal with respect to specific application.`
  },
  {
    id: 'j-nes-11-unity-and-negative-connection',
    title: 'Unity of genus and the negative connection of species',
    text: `Species are both positively identical (share the genus) and negatively connected (mutually exclusive). The "either/or" is the negative unity: species exclude one another yet remain identical in the genus as a unity of determinate particulars.`
  },
  {
    id: 'j-nes-12-empirical-vs-conceptual-disjunction',
    title: 'Empirical disjunction vs necessary disjunction',
    text: `Empirical "either/or" (listing species) lacks necessity because completeness is subjective. True disjunctive necessity requires a concrete, immanent genus whose determinateness yields the principled "either/or" and connects species reciprocally by their specific differences.`
  },
  {
    id: 'j-nes-13-proximate-genus-and-completeness',
    title: 'Proximate genus: concreteness and completeness of species set',
    text: `When the genus is concrete (not an abstract comparison), it is the proximate genus: its immanent determinateness grounds the specific differences and ensures the species form a complete, necessary disjunction rather than a contingent plurality.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'j-nes-op-9-detect-disjunctive-form',
    chunkId: 'j-nes-9-disjunctive-overview',
    label: 'Detect disjunctive judgment form ("A is either B or C")',
    clauses: [
      'if clausePattern matches disjunctiveForm then tag(judgment,"disjunctive")',
      'annotate(judgment,{genus:subject, species:listOfAlternatives})'
    ],
    predicates: [{ name: 'IsDisjunctive', args: [] }],
    relations: [{ predicate: 'annotates', from: 'analyzer', to: 'disjunctiveNodes' }]
  },
  {
    id: 'j-nes-op-10-evaluate-species-as-predicate',
    chunkId: 'j-nes-10-particularity-as-predicate',
    label: 'Evaluate species particularity as predicate (contains universal sphere)',
    clauses: [
      'for each species in speciesList compute(containsUniversalSphere(species, genus))',
      'if contains then tag(species,"constitutesPredicate")'
    ],
    predicates: [{ name: 'EvaluatesSpeciesPredicateRole', args: [] }],
    relations: [{ predicate: 'tags', from: 'evaluator', to: 'speciesNodes' }]
  },
  {
    id: 'j-nes-op-11-check-negative-unity',
    chunkId: 'j-nes-11-unity-and-negative-connection',
    label: 'Check mutual exclusion and genus-identity (negative unity)',
    clauses: [
      'assert(all species share genusIdentity)',
      'verify(mutualExclusion(speciesPair) => true for canonicalDifferences)',
      'if both hold then tag(judgment,"negative-unity-validated")'
    ],
    predicates: [{ name: 'ValidatesNegativeUnity', args: [] }],
    relations: [{ predicate: 'validates', from: 'checker', to: 'disjunctiveNodes' }]
  },
  {
    id: 'j-nes-op-12-flag-empirical-disjunction',
    chunkId: 'j-nes-12-empirical-vs-conceptual-disjunction',
    label: 'Flag empirical disjunctions lacking immanent genus',
    clauses: [
      'if speciesListConstructedBy == enumeration or absenceOfCounterexample then tag(judgment,"empirical-disjunction")',
      'recommend(deepen:conceptualAnalysis)'
    ],
    predicates: [{ name: 'DetectsEmpiricalDisjunction', args: [] }],
    relations: [{ predicate: 'recommends', from: 'detector', to: 'analysisQueue' }]
  },
  {
    id: 'j-nes-op-13-promote-proximate-genus',
    chunkId: 'j-nes-13-proximate-genus-and-completeness',
    label: 'Promote a concrete genus to proximate-genus status and validate completeness',
    clauses: [
      'if genus.isConcrete and speciesSet.coversPrincipledDifferentiation then reclassify(genus,"proximate")',
      'on(reclassify) => mark(judgment,"necessary-disjunction") and preserve(metadata:{origin:genus, speciesSet})'
    ],
    predicates: [{ name: 'PromotesProximateGenus', args: [] }],
    relations: [{ predicate: 'reclassifies', from: 'system', to: 'genusNode' }]
  }
]

/* appended: Judgment of Necessity — c. The disjunctive judgment (continuation)
   - continued exposition: concept disjoins itself, species as differentiated determinateness,
     warnings about empirical lists, subject/predicate as members of disjunction,
     copula becomes concept -> judgment of the concept
*/
CANONICAL_CHUNKS.push(
  {
    id: 'j-nes-14-concept-disjoins-itself',
    title: 'Concept disjoins itself — determinateness and negative unity',
    text: `The disjunction reduces to the difference of the concept: the concept (universal) disjoins itself and thereby manifests its negative unity in differentiated determinateness — subject as simple determinateness and predicate as totality.`
  },
  {
    id: 'j-nes-15-species-as-conceptual-moments',
    title: 'Species as moments of the concept — principled particularity',
    text: `Species are the concept's own determinations; their reciprocal determination derives from the concept. Proper disjunction requires the genus' internal principle of differentiation, not a mere empirical list of alternatives.`
  },
  {
    id: 'j-nes-16-warning-empirical-disjunctions',
    title: 'Warning: empirical disjunctions and improper genera',
    text: `Listing heterogeneous or mixed members (e.g., arbitrary color shades) as species betrays lack of conceptual determinateness. Such empirical disjunctions lack principled necessity and must be refined by finding the genuine internal principle of differentiation.`
  },
  {
    id: 'j-nes-17-subject-and-predicate-as-disjoined-members',
    title: 'Predicate and subject as members of the disjunction',
    text: `In the disjunctive judgment the subject and predicate themselves are members of the disjunction: both are moments of the concept posited as determinateness and at the same time identical in objective universality and negative unity.`
  },
  {
    id: 'j-nes-18-copula-as-concept-and-judgment-of-concept',
    title: 'Copula becomes the concept — judgment of the concept',
    text: `The copula's unity, where extremes coincide through identity, is the concept itself as posited. Thus the judgment of necessity culminates in the judgment of the concept: necessity has risen to concepthood.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'j-nes-op-14-detect-concept-disjunction',
    chunkId: 'j-nes-14-concept-disjoins-itself',
    label: 'Detect whether disjunction arises from the concept (not enumeration)',
    clauses: [
      'if disjunction.membersArePrincipledBy(genus) and differentiationPrinciple.found then tag(judgment,"concept-disjunction")',
      'else tag(judgment,"suspect-empirical-disjunction")'
    ],
    predicates: [{ name: 'IsConceptDisjunction', args: [] }],
    relations: [{ predicate: 'flags', from: 'analyzer', to: 'disjunctionNodes' }]
  },
  {
    id: 'j-nes-op-15-validate-species-principle',
    chunkId: 'j-nes-15-species-as-conceptual-moments',
    label: 'Validate that species are moments of the concept (principled separation)',
    clauses: [
      'for each species compute(isImmanentTo(genus))',
      'if all true then tag(speciesSet,"principled-complete") else recommend(refine:principleExtraction)'
    ],
    predicates: [{ name: 'ValidatesSpeciesPrinciple', args: [] }],
    relations: [{ predicate: 'validates', from: 'validator', to: 'speciesSet' }]
  },
  {
    id: 'j-nes-op-16-flag-and-refine-empirical-lists',
    chunkId: 'j-nes-16-warning-empirical-disjunctions',
    label: 'Flag empirical disjunctions and suggest refinement workflow',
    clauses: [
      'if disjunction.tag == "suspect-empirical-disjunction" then emit(workflow:"refine-disjunction", payload:{node,examples})',
      'attach(recommendation:"seek internal principle of differentiation")'
    ],
    predicates: [{ name: 'HandlesEmpiricalDisjunctions', args: [] }],
    relations: [{ predicate: 'escalates', from: 'system', to: 'curationQueue' }]
  },
  {
    id: 'j-nes-op-17-link-subject-predicate-membership',
    chunkId: 'j-nes-17-subject-and-predicate-as-disjoined-members',
    label: 'Link and validate subject/predicate as disjunctive members',
    clauses: [
      'if subject and predicate both in speciesSet then annotate(judgment,"members-validated")',
      'record(meta:{roleAsMoment:true,universalIdentityConfirmed:true})'
    ],
    predicates: [{ name: 'LinksDisjunctiveMembers', args: [] }],
    relations: [{ predicate: 'annotates', from: 'linker', to: 'judgmentNode' }]
  },
  {
    id: 'j-nes-op-18-promote-to-judgment-of-concept',
    chunkId: 'j-nes-18-copula-as-concept-and-judgment-of-concept',
    label: 'Promote judgment to "judgment-of-concept" when copula identity is established',
    clauses: [
      'if judgment.tags includes "concept-disjunction" and copula.identityConfirmed then reclassify(judgment,"judgment-of-concept")',
      'on(reclassify) => preserve(metadata:{genus, speciesSet,principle}) and emit(event:"promotedToConceptJudgment", payload:{node})'
    ],
    predicates: [{ name: 'PromotesToConceptJudgment', args: [] }],
    relations: [{ predicate: 'reclassifies', from: 'system', to: 'judgmentNode' }]
  }
)
