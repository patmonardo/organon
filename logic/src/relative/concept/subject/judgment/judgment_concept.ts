import type { Chunk, LogicalOperation } from './index'

/* Judgment of the Concept — a. The assertoric judgment
   Full-text broken into explicit chunks (preserves original source paragraphs)
   and HLOs (logical operations) that capture detection, decomposition,
   conformity assessment, subjective-flagging, conflict handling, and promotion rules.
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-con-1-assertoric-intro',
    title: 'Assertoric judgment — introduction',
    text: `The judgment of the concept is at first immediate; as such, it is the assertoric judgment. The subject is a concrete singular in general, and the predicate expresses this same singular as the connection of its actuality, its determinateness or constitution, to its concept. ("This house is bad," "this action is good.")`
  },
  {
    id: 'j-con-2-assertoric-ought',
    title: 'Assertoric judgment — the "ought" (universal nature)',
    text: `More closely considered, it contains, therefore, (a) that the subject ought to be something; its universal nature has posited itself as the self-subsistent concept.`
  },
  {
    id: 'j-con-3-assertoric-particularity',
    title: 'Assertoric judgment — particularity and constitution',
    text: `More closely considered, it contains, therefore, (b) that particularity is something constituted or an external concrete existence, not only because of its immediacy, but because it expressly differs from its self-subsisting universal nature; its external concrete existence, for its part, because of this self-subsistence of the concept, is also indifferent with respect to the universal and may or may not conform to it.`
  },
  {
    id: 'j-con-4-assertoric-constitution-link',
    title: 'Assertoric judgment — constitution, singularity, and the disjunctive link',
    text: `This constitution is the singularity which in the disjunctive judgment escapes the necessary determination of the universal, a determination that exists only as the particularization of the species and as the negative principle of the genus. Thus the concrete universality that has come out of the disjunctive judgment divides in the assertoric judgment into the form of extremes to which the concept itself, as the posited unity connecting them, is still lacking.`
  },
  {
    id: 'j-con-5-assertoric-subjective-assurance',
    title: 'Assertoric judgment — subjective assurance and the abstract copula',
    text: `For this reason the judgment is so far only assertoric; its credential is only a subjective assurance. That something is good or bad, right, suitable or not, hangs on an external third. But to say that the connectedness is thus externally posited is the same as saying that it is still only in itself or internal. When we say that something is good or bad, etc., we certainly do not mean to say that it is good only in a subjective consciousness but may perhaps be bad in itself, or that "good and bad," "right," "suitable," etc. may not be predicates of the object itself. The merely subjective character of the assertion of this judgment consists, therefore, in the fact that the implicitly present connectedness of subject and predicate has not been posited yet, or, what amounts to the same thing, that it is only external; the copula still is an immediate abstract being.`
  },
  {
    id: 'j-con-6-assertoric-conflict-and-problematic',
    title: 'Assertoric judgment — conflict, contingency, and problematic status',
    text: `Thus the assurance of the assertoric judgment can with right be confronted by an opposing one. When the assurance is given that "this action is good," the opposite, "this action is bad," has equal justification. Or, considering the judgment in itself, since its subject is an immediate singular, in this abstraction it still does not have, posited in it, the determinateness that would contain its connection with the universal concept; it still is a contingent matter, therefore, whether there is or there is not conformity to the concept. Essentially, therefore, the judgment is problematic.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'j-con-op-1-detect-assertoric',
    chunkId: 'j-con-1-assertoric-intro',
    label: 'Detect assertoric pattern and annotate subject/predicate roles',
    clauses: [
      'if clausePattern matches ("This X is Y" | "The X, as so constituted, is Y") then tag(judgment,"assertoric")',
      'annotate(judgment,{subjectKind:"concrete-singular", predicateRole:"relationOfActualityToConcept"})'
    ],
    predicates: [{ name: 'IsAssertoric', args: [] }],
    relations: [{ predicate: 'annotates', from: 'analyzer', to: 'assertoricNodes' }]
  },
  {
    id: 'j-con-op-2-decompose-ought-and-concept',
    chunkId: 'j-con-2-assertoric-ought',
    label: 'Decompose subject into ought (concept) and register universal nature',
    clauses: [
      'decompose(subject -> {ought:universalConcept, constitution:particularity})',
      'if universalConcept.found then tag(subject,"includes-ought")',
      'emit(provenance:{sourceChunk:chunkId, sourceOp:"j-con-op-2-decompose-ought-and-concept", extractor:"hlo-extract-v1", ts:now()})'
    ],
    predicates: [{ name: 'DecomposesOught', args: [] }],
    relations: [{ predicate: 'decomposes', from: 'decomposer', to: 'subjectNode' }]
  },
  {
    id: 'j-con-op-3-evaluate-particularity-conformity',
    chunkId: 'j-con-3-assertoric-particularity',
    label: 'Assess whether particular constitution conforms to the concept',
    clauses: [
      'evidence = collectEvidenceItems(subject.constitution, subject.ought) // traces used for score',
      'provenance = {sourceChunk:chunkId, sourceOp:"j-con-op-3-evaluate-particularity-conformity", deps:[\"j-con-op-2-decompose-ought-and-concept\"]}',
      'compute(conformityScore = conformityScore(subject.constitution, subject.ought, evidence))',
      'threshold = params?.conformityThreshold ?? 0.7',
      'if conformityScore >= threshold then tag(judgment,"conforms") else tag(judgment,"diverges")',
      'record(meta:{conformityScore, threshold, evidence, provenance})'
    ],
    predicates: [{ name: 'AssessesConformity', args: [] }],
    relations: [{ predicate: 'evaluates', from: 'assessor', to: 'judgmentNode' }]
  },
  {
    id: 'j-con-op-4-link-to-disjunctive-origin',
    chunkId: 'j-con-4-assertoric-constitution-link',
    label: 'Link assertoric constitution back to disjunctive origin and note missing unity',
    clauses: [
      'if subject.constitution.origin == "disjunctive" then annotate(judgment,"origin-disjunctive")',
      'if conceptUnityNotPosited then tag(judgment,"unity-missing")'
    ],
    predicates: [{ name: 'LinksDisjunctiveOrigin', args: [] }],
    relations: [{ predicate: 'links', from: 'analyzer', to: 'originNodes' }]
  },
  {
    id: 'j-con-op-5-flag-subjective-assurance',
    chunkId: 'j-con-5-assertoric-subjective-assurance',
    label: 'Flag subjective assurance and abstract copula; recommend review',
    clauses: [
      'if judgment.tag == "assertoric" and not innerConnectionPosited then tag(judgment,"subjective-assurance")',
      'if conflictingAssertionsExist(subject) then emit(reviewRequest:{judgment,conflicts})'
    ],
    predicates: [{ name: 'FlagsSubjectiveAssurance', args: [] }],
    relations: [{ predicate: 'escalates', from: 'system', to: 'curationQueue' }]
  },
  {
    id: 'j-con-op-6-promote-to-problematic',
    chunkId: 'j-con-6-assertoric-conflict-and-problematic',
    label: 'Classify as problematic when contingency remains and conflict possible',
    clauses: [
      'if judgment.tag == "assertoric" and (conflicts.exists || conformityScore < threshold) then reclassify(judgment,"problematic")',
      'attach(meta:{reason:"contingency-or-conflict",conformityScore})'
    ],
    predicates: [{ name: 'PromotesToProblematic', args: [] }],
    relations: [{ predicate: 'reclassifies', from: 'system', to: 'judgmentNode' }]
  }
]

