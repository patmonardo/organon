import type { Chunk, LogicalOperation } from './index'

/* Judgment of the Concept — c. The apodictic judgment
   - numbering continues from j-con-8
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-con-8-apodictic-overview',
    title: 'Apodictic judgment — overview',
    text: `The apodictic judgment predicates that the subject (as constituted) includes both its ought (universal) and its constitution; the copula now expresses a determinate correspondence of ought and being, making the judgment truly objective and the truth of judgment in general.`
  },
  {
    id: 'j-con-9-fact-constitution-and-soul',
    title: 'Fact constitution: the ought/being partition and the fact\'s soul',
    text: `The subject is internally fractured into ought and being; this partition is the fact's constitution. The inner connection (concrete identity) of these moments constitutes the fact's soul and grounds the apodictic judgment.`
  },
  {
    id: 'j-con-10-copula-determinateness-and-ground',
    title: 'Copula determinateness: ground in particular constitution',
    text: `The copula has progressed from abstract "is" to a determinate ground: the particular constitution of the subject manifests why the predicate pertains, so the copula is the determinate connection (ground) that binds subject and predicate.`
  },
  {
    id: 'j-con-11-syllogistic-transition',
    title: 'Copula replete — from apodictic judgment to syllogism',
    text: `Because subject and predicate now each embody the whole concept and the copula is replete with content (the connecting determinant), the judgment recovers the unity of the concept and has thereby become syllogism.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'j-con-op-8-declare-apodictic',
    chunkId: 'j-con-8-apodictic-overview',
    label: 'Declare apodictic judgment and annotate objective correspondence',
    clauses: [
      'if clausePattern == "the X, as so constituted, is Y" then tag(judgment,"apodictic")',
      'annotate(judgment,{subjectIncludes:["ought","constitution"],copula:"determinateCorrespondence"})'
    ],
    predicates: [{ name: 'IsApodictic', args: [] }],
    relations: [{ predicate: 'annotates', from: 'analyzer', to: 'apodicticNodes' }]
  },
  {
    id: 'j-con-op-9-evaluate-fact-soul',
    chunkId: 'j-con-9-fact-constitution-and-soul',
    label: 'Evaluate internal partition (ought vs being) and record fact-soul',
    clauses: [
      'decompose(subject -> {ought,being})',
      'if linkExists(ought,being) then tag(subject,"fact-with-soul")',
      'record(meta:{ought,being,linkEvidence})'
    ],
    predicates: [{ name: 'EvaluatesFactSoul', args: [] }],
    relations: [{ predicate: 'records', from: 'evaluator', to: 'subjectNode' }]
  },
  {
    id: 'j-con-op-10-assess-copula-grounding',
    chunkId: 'j-con-10-copula-determinateness-and-ground',
    label: 'Assess whether copula expresses determinate ground in subject constitution',
    clauses: [
      'if copula.expressesGround(subject.constitution, predicate.concept) then tag(judgment,"grounded-copula")',
      'emit(meta:{groundEvidence,conformityScore})'
    ],
    predicates: [{ name: 'AssessesCopulaGround', args: [] }],
    relations: [{ predicate: 'emits', from: 'assessor', to: 'evidenceStore' }]
  },
  {
    id: 'j-con-op-11-promote-to-syllogism',
    chunkId: 'j-con-11-syllogistic-transition',
    label: 'Promote apodictic judgment to syllogism when copula replete',
    clauses: [
      'if judgment.tag includes "apodictic" and copula.tag == "grounded-copula" and subject.predicateBoth == wholeConcept then reclassify(judgment,"syllogism")',
      'on(reclassify) => preserve(meta:{origin:"apodictic-to-syllogism",evidence})'
    ],
    predicates: [{ name: 'PromotesToSyllogism', args: [] }],
    relations: [{ predicate: 'reclassifies', from: 'system', to: 'judgmentNode' }]
  }
]
