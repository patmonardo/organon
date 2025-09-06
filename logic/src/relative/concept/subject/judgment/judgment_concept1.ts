import type { Chunk, LogicalOperation } from './index'

/* Judgment of the Concept — b. The problematic judgment
   NOTE: chunk/operation numbering starts at 4 in this concept series.
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-con-4-problematic-overview',
    title: 'Problematic judgment — overview',
    text: `The problematic judgment is the assertoric taken both positively and negatively: the copula's indeterminateness places contingency on the subject's immediacy. Here the predicate already names the objective concept, but whether the subject conforms is unresolved, making the judgment problematic.`
  },
  {
    id: 'j-con-5-copula-indeterminacy',
    title: 'Indeterminacy falls on the copula / subject immediacy',
    text: `Indeterminateness primarily concerns the copula: the predicate is objective universality, so the contingent element lies in the subject's immediate constitution. The copula does not yet posit an internal connection; hence the judgment remains problematic.`
  },
  {
    id: 'j-con-6-subject-split-unity',
    title: 'Subject split: universal (ought) vs particular constitution',
    text: `The subject is divided into its universality (the ought, concept) and its particular constitution (the factual mode). This split provides the ground for either being or not being what the concept demands; in that sense the subject is already equated with the predicate as containing both moments.`
  },
  {
    id: 'j-con-7-subjectivity-duplicity-and-apodictic-transition',
    title: 'Duplicity of subjectivity and transition to apodictic',
    text: `The two senses of subjectivity (conceptual and constituted) are unified in the fact: when the problematic character is posited as characteristic of the fact itself, the judgment ceases to be problematic and becomes apodictic — the internal connection is thereby affirmed.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'j-con-op-4-detect-problematic-assertoric',
    chunkId: 'j-con-4-problematic-overview',
    label: 'Detect problematic assertoric judgments',
    clauses: [
      'if judgment.tag == "assertoric" and predicate.isObjectiveConcept and not judgement.innerConnectionPosited then tag(judgment,"problematic")',
      'emit(meta:{reason:"copula-indeterminate",subjectImmediacy:true})'
    ],
    predicates: [{ name: 'DetectsProblematic', args: [] }],
    relations: [{ predicate: 'flags', from: 'analyzer', to: 'problematicNodes' }]
  },
  {
    id: 'j-con-op-5-assess-copula-indeterminacy',
    chunkId: 'j-con-5-copula-indeterminacy',
    label: 'Assess indeterminacy locus (copula vs predicate)',
    clauses: [
      'if predicate.isConcreteUniversality then locusOfIndeterminacy = subject',
      'score = compute(contingencyScore(subject.constitution))',
      'attach(meta:{locus:locusOfIndeterminacy,score})'
    ],
    predicates: [{ name: 'AssessesCopulaIndeterminacy', args: [] }],
    relations: [{ predicate: 'annotates', from: 'assessor', to: 'judgmentNode' }]
  },
  {
    id: 'j-con-op-6-split-subject-and-equate',
    chunkId: 'j-con-6-subject-split-unity',
    label: 'Split subject into concept vs constitution and record equivalence with predicate',
    clauses: [
      'decompose(subject -> {universalConcept, particularConstitution})',
      'if particularConstitution.containsGroundFor(conformity) then annotate(judgment,"subject-equated-to-predicate-moment")',
      'record(meta:{universal:universalConcept,constitution:particularConstitution})'
    ],
    predicates: [{ name: 'SplitsSubject', args: [] }],
    relations: [{ predicate: 'records', from: 'decomposer', to: 'subjectNode' }]
  },
  {
    id: 'j-con-op-7-detect-apodictic-transition',
    chunkId: 'j-con-7-subjectivity-duplicity-and-apodictic-transition',
    label: 'Promote problematic → apodictic when problematicness posited as fact-characteristic',
    clauses: [
      'if judgment.tag == "problematic" and fact.meta.positsProblematicAsCharacteristic then reclassify(judgment,"apodictic")',
      'on(reclassify) => preserve(meta:{origin:"problematic-to-apodictic",evidence})'
    ],
    predicates: [{ name: 'PromotesApodictic', args: [] }],
    relations: [{ predicate: 'reclassifies', from: 'system', to: 'judgmentNode' }]
  }
]
