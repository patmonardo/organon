import type { Chunk, LogicalOperation } from '../syllogism/index'

/* Judgment of Reflection — b. The particular judgment
   NOTE: chunk/operation numbering starts at 4 for this module.
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-ref-4-particular-overview',
    title: 'Particular judgment — overview',
    text: `Particularity is the non-singularity posited in reflection: the subject becomes "these ones" or "some singulars." The particular judgment expresses the extension of the singular into plurality while retaining the reflective determination that made the singular into a basis.`
  },
  {
    id: 'j-ref-5-indeterminate-dual-polarity',
    title: 'Particularity contains universality and indeterminacy',
    text: `The particular ("some") contains both universality and particularity: it is simultaneously positive and negative (e.g. "some are P" implies "some are not P"). This containment renders the particular judgment indeterminate.`
  },
  {
    id: 'j-ref-6-subject-anticipates-universal',
    title: 'Subject anticipates universal / transition toward universality',
    text: `The subject in the particular judgment combines singulars with a universal content (e.g. "humans"). This universality is not empirical filler but a reflection-produced species. Because the "some" extension is indeterminate relative to the singular, the particular judgment tends to pass over into the universal.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'j-ref-op-4-declare-particular',
    chunkId: 'j-ref-4-particular-overview',
    label: 'Declare particular judgment form',
    clauses: [
      'judgment.kind = reflection.particular',
      'subject.form = some(singulars)',
      'subject.includes = {singularBasis, reflectedExtension}'
    ],
    predicates: [{ name: 'IsParticularJudgment', args: [] }],
    relations: [{ predicate: 'contains', from: 'subject', to: 'singularBasis' }]
  },
  {
    id: 'j-ref-op-5-encode-indeterminacy',
    chunkId: 'j-ref-5-indeterminate-dual-polarity',
    label: 'Encode dual polarity and indeterminacy of the particular',
    clauses: [
      'if subject.form == some(...) then judgment.implies = {someAreP, someAreNotP}',
      'particular.indeterminate = true'
    ],
    predicates: [{ name: 'ParticularIsIndeterminate', args: [] }],
    relations: [{ predicate: 'implies', from: 'particular', to: 'dualPolarity' }]
  },
  {
    id: 'j-ref-op-6-promote-to-universal',
    chunkId: 'j-ref-6-subject-anticipates-universal',
    label: 'Detect anticipation of universality and trigger transition',
    clauses: [
      'if subject.contains(universalContent) && extensionNotCommensurateWithSingular then trigger(transitionToUniversal)',
      'on(transitionToUniversal) => reclassify(judgment.kind = universal)',
      'preserve(origins:{singularBasis,particularForm})'
    ],
    predicates: [{ name: 'PromotesParticularToUniversal', args: [] }],
    relations: [{ predicate: 'transitions', from: 'particularJudgment', to: 'universalJudgment' }]
  },
  /* appended logical ops: strengthen diagnostic + transition rules for the particular judgment */
  {
    id: 'j-ref-op-6b-detect-dual-polarity',
    chunkId: 'j-ref-5-indeterminate-dual-polarity',
    label: 'Detect dual polarity and tag particular as indeterminate',
    clauses: [
      'if subject.form == some(...) and judgment.implies contains both positive and negative then tag(subject,"indeterminate")',
      'tag(subject,"dual-polarity")',
      'emit(metadata:{origin:singularBasis,form:particular})'
    ],
    predicates: [{ name: 'DetectsDualPolarity', args: [] }],
    relations: [{ predicate: 'tags', from: 'analyzer', to: 'particularNodes' }]
  },
  {
    id: 'j-ref-op-7-trigger-universalization',
    chunkId: 'j-ref-6-subject-anticipates-universal',
    label: 'Robust trigger for particular -> universal transition',
    clauses: [
      'if subject.contains(universalContent) and coverageOf(universalContent) >= threshold then trigger(transitionToUniversal)',
      'on(transitionToUniversal) => reclassify(judgment.kind = universal) and preserve(origins:{singularBasis,particularForm})',
      'else => mark(node,"remains-particular-indeterminate")'
    ],
    predicates: [{ name: 'TriggersUniversalization', args: ['threshold'] }],
    relations: [{ predicate: 'transitions', from: 'particularJudgment', to: 'universalJudgment' }]
  }
]
