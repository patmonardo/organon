import type { Chunk, LogicalOperation } from '../syllogism/index'

/* Judgment of Necessity — b. The hypothetical judgment
   - "If A is, then B is": posits necessary connectedness (ground ↔ consequence)
   - externals A and B are immediate; what is essential is their connection, not the extremes
   - maps causality/condition/ground as moments of a single identity (concrete universality)
   - form is indeterminate; content becomes universality-as-concrete-identity (disjunctive judgment)
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-nes-5-hypothetical-overview',
    title: 'Hypothetical judgment — overview ("If A then B")',
    text: `The hypothetical judgment posits the necessary connectedness of immediate determinacies: not that A or B simply are, but that if A is then B is. The being of finite things is their being-for-an-other; the connection (ground/consequence) is the essential posited content.`
  },
  {
    id: 'j-nes-6-relations-as-moments',
    title: 'Ground, consequence, causality as reflexive moments',
    text: `Relations of reflection (ground/consequence, condition/conditioned, causality) occur as moments of one identity in the hypothetical judgment. These relations are present as interconnected moments rather than as isolated determinations.`
  },
  {
    id: 'j-nes-7-indeterminate-form',
    title: 'Indeterminate form and content of the hypothetical',
    text: `The hypothetical has an indeterminate propositional form initially: extremes are externally contingent and indifferent. Its content does not conform to simple subject–predicate shape; the form expresses a unity of self and other (concrete self‑identity).`
  },
  {
    id: 'j-nes-8-concrete-universality-disjunction',
    title: 'Concrete universality and the disjunctive judgment',
    text: `What is posited is universality as concrete identity: determinations lack independent subsistence and are particularities within that identity. The hypothetical is thus a disjunctive judgement that grounds particularity in a unified concept.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'j-nes-op-5-detect-hypothetical-shape',
    chunkId: 'j-nes-5-hypothetical-overview',
    label: 'Detect hypothetical form (if→then) and mark ground/consequence',
    clauses: [
      'if clausePattern == "if A then B" then tag(judgment,"hypothetical")',
      'annotate(judgment,{ground:A,consequence:B})'
    ],
    predicates: [{ name: 'IsHypothetical', args: [] }],
    relations: [{ predicate: 'annotates', from: 'analyzer', to: 'judgmentNode' }]
  },
  {
    id: 'j-nes-op-6-map-reflective-relations',
    chunkId: 'j-nes-6-relations-as-moments',
    label: 'Map ground/consequence/causality as unified reflective moments',
    clauses: [
      'mapRelations({ground,condition,cause} -> unifiedIdentityMoment)',
      'if relationsDetected then linkAs(momentsOf:unifiedIdentity)'
    ],
    predicates: [{ name: 'MapsReflectiveRelations', args: [] }],
    relations: [{ predicate: 'links', from: 'relationDetector', to: 'unifiedMoment' }]
  },
  {
    id: 'j-nes-op-7-flag-indeterminate-form',
    chunkId: 'j-nes-7-indeterminate-form',
    label: 'Flag indeterminate hypothetical form and recommend specification',
    clauses: [
      'if judgment.kind == hypothetical and extremes.indifferent then tag(judgment,"indeterminate-form")',
      'recommend(specify:{groundEvidence,conditionarity,scope})'
    ],
    predicates: [{ name: 'FlagsIndeterminateHypothetical', args: [] }],
    relations: [{ predicate: 'recommends', from: 'system', to: 'curation' }]
  },
  {
    id: 'j-nes-op-8-classify-disjunctive-concrete-universal',
    chunkId: 'j-nes-8-concrete-universality-disjunction',
    label: 'Classify hypothetical as disjunctive/concrete-universal when moments unified',
    clauses: [
      'if moments.linkedAsUnifiedIdentity then reclassify(judgment,"disjunctive-concrete-universal")',
      'attach(meta:{conceptualUnity:true,particularitiesBound:true})'
    ],
    predicates: [{ name: 'ClassifiesAsConcreteUniversal', args: [] }],
    relations: [{ predicate: 'reclassifies', from: 'classifier', to: 'judgmentNode' }]
  }
]

/* accessors */
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 5] ?? null
}

export function getLogicalOpsForChunk(oneBasedIndex: number) {
  const chunk = getChunk(oneBasedIndex)
  if (!chunk) return []
  return LOGICAL_OPERATIONS.filter(op => op.chunkId === chunk.id)
}

//
