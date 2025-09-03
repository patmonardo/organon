import type { Chunk, LogicalOperation } from '../syllogism/index'

/* Judgment of Reflection — a. The singular judgment
   - immediate form: "the singular is universal" reinterpreted: "this is an essential universal"
   - predicate is implicit universal; negation applies to the subject (alterable), not directly to predicate
   - proximate truth of singular judgment resides in the particular judgment
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-ref-1-singular-judgment',
    title: 'Singular judgment — immediate reflective form',
    text: `The immediate judgment of reflection again appears as "the singular is universal", but now the subject and predicate carry reflective signification: the predicate is an implicit universal and the subject is that which requires determination.`
  },
  {
    id: 'j-ref-2-this-not-essential',
    title: '"This" is not an essential universal',
    text: `A mere "this" does not constitute an essential universal. In the judgment of reflection the positive form must be taken negatively: negation functions primarily on the subject (the "this") because the predicate is an implicit being rather than an inhering quality.`
  },
  {
    id: 'j-ref-3-proximate-truth-particular',
    title: 'Proximate truth in the particular judgment',
    text: `The singular judgment finds its proximate truth in the particular judgment: "'not a this' is a universal of reflection" — the in-itself has a more universal concrete existence in reflection than in immediate "this".`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'j-ref-op-1-declare-singular-reflection',
    chunkId: 'j-ref-1-singular-judgment',
    label: 'Declare singular judgment in reflection',
    clauses: [
      'judgment.kind = reflection.singular',
      'form = "singular is universal" (reflective signification)',
      'predicate.role = implicitUniversal'
    ],
    predicates: [{ name: 'IsSingularReflection', args: [] }],
    relations: [{ predicate: 'signifies', from: 'predicate', to: 'implicitUniversal' }]
  },
  {
    id: 'j-ref-op-2-negation-targets-subject',
    chunkId: 'j-ref-2-this-not-essential',
    label: 'Formalize negation applying to subject (alterability)',
    clauses: [
      'negation.target = subject (not predicate)',
      'subject.requiresDetermination = true',
      'predicate.inheres? = false (predicate = implicit being)'
    ],
    predicates: [{ name: 'NegationTargetsSubject', args: [] }],
    relations: [{ predicate: 'requires', from: 'subject', to: 'determinationProcess' }]
  },
  {
    id: 'j-ref-op-3-link-proximate-truth-particular',
    chunkId: 'j-ref-3-proximate-truth-particular',
    label: 'Link singular judgment to proximate truth in particular judgment',
    clauses: [
      'on(singularReflection) => propose(particularJudgment) as proximateTruth',
      '"not a this" => universalOfReflection',
      'reflection.increasesConcreteness(subject)'
    ],
    predicates: [{ name: 'LinksToParticularProximateTruth', args: [] }],
    relations: [{ predicate: 'proposes', from: 'singularReflection', to: 'particularJudgment' }]
  }
]
