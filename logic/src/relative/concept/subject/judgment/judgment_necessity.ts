import type { Chunk, LogicalOperation } from '../syllogism/index'

/* Judgment of Necessity — a. The categorical judgment
   - categorical judgment as first immediate form of necessity (genus/species structure)
   - distinguishes categorical predicates (essential/unity) from accidental predicates
   - copula here means necessity (not mere being)
   - determinateness is initially contingent but becomes inner-necessity -> passage to hypothetical
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-nes-1-categorical-overview',
    title: 'Categorical judgment — overview',
    text: `The categorical judgment predicates a universality (genus/species) that is the immanent nature of the subject. Its copula expresses necessity: the predicate is the subject's being-in-and-for-itself rather than an accidental quality.`
  },
  {
    id: 'j-nes-2-distinguish-accidental-vs-essential',
    title: 'Distinguish accidental predicates from categorical predicates',
    text: `Predicates like color are accidental; predicates that express the subject's substantial identity (e.g., "it is human") are categorical. Lumping accidental and categorical predicates together obscures necessity and the form of true judgment.`
  },
  {
    id: 'j-nes-3-determinateness-and-inner-necessity',
    title: 'Determinateness, contingency, and inner necessity',
    text: `The subject's determinateness may appear contingent externally, yet the categorical judgment posits an inner necessity: the subject's particularity is essentially connected to the predicate. This inner necessity grounds the transition to hypothetical judgments.`
  },
  {
    id: 'j-nes-4-categorical-to-hypothetical-transition',
    title: 'Transition: categorical judgment passing to hypothetical',
    text: `Because the objective universal determines itself in relation to repelled determinateness, the categorical judgment's necessity becomes explicit and thereby passes into the hypothetical form (necessity expressed as conditional connections among principles).`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'j-nes-op-1-detect-categorical',
    chunkId: 'j-nes-1-categorical-overview',
    label: 'Detect categorical predicate (necessity-candidate)',
    clauses: [
      'if predicate.role == "substantialIdentity" or predicate.reflects(subject.beingInItself) then tag(node,"categorical")',
      'copula.interpretation = "necessity"'
    ],
    predicates: [{ name: 'IsCategorical', args: [] }],
    relations: [{ predicate: 'tags', from: 'analyzer', to: 'categoricalNodes' }]
  },
  {
    id: 'j-nes-op-2-distinguish-accidental',
    chunkId: 'j-nes-2-distinguish-accidental-vs-essential',
    label: 'Differentiate accidental vs essential predicates',
    clauses: [
      'if predicate.isEmpiricalFeature then tag(predicate,"accidental")',
      'if predicate.expressesSpeciesOrGenus then tag(predicate,"essential")'
    ],
    predicates: [{ name: 'ClassifyPredicateKind', args: [] }],
    relations: [{ predicate: 'classifies', from: 'analyzer', to: 'predicateNodes' }]
  },
  {
    id: 'j-nes-op-3-assess-inner-necessity',
    chunkId: 'j-nes-3-determinateness-and-inner-necessity',
    label: 'Assess inner necessity of subject–predicate connection',
    clauses: [
      'compute(connectivityScore(subject,predicate))',
      'if connectivityScore >= threshold then tag(node,"inner-necessity")',
      'emit(metadata:{copula:"necessity",score:connectivityScore})'
    ],
    predicates: [{ name: 'AssessesInnerNecessity', args: [] }],
    relations: [{ predicate: 'evaluates', from: 'assessor', to: 'judgmentNode' }]
  },
  {
    id: 'j-nes-op-4-promote-to-hypothetical',
    chunkId: 'j-nes-4-categorical-to-hypothetical-transition',
    label: 'Promote categorical judgment into hypothetical schema when necessity explicit',
    clauses: [
      'on(tag(node,"inner-necessity")) => propose(hypotheticalForm {if:principleA, then:principleB})',
      'attach(provenance:{originalCategorical,connectivityScore})',
      'emit(event:"candidate-hypothetical", payload:{node})'
    ],
    predicates: [{ name: 'PromotesToHypothetical', args: [] }],
    relations: [{ predicate: 'proposes', from: 'system', to: 'hypotheticalCandidates' }]
  }
]

/* accessors */
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null
}

export function getLogicalOpsForChunk(oneBasedIndex: number) {
  const chunk = getChunk(oneBasedIndex)
  if (!chunk) return []
  return LOGICAL_OPERATIONS.filter(op => op.chunkId === chunk.id)
}
