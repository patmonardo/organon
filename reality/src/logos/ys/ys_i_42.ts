import { DatasetUnit, makeUnitId } from '../registry/canon'

export type Chunk = { id: string; title: string; text: string }
export type LogicalOperation = {
  id: string
  chunkId: string
  label: string
  clauses: string[]
  predicates?: Array<{ name: string; args: string[] }>
  relations?: Array<{ predicate: string; from: string; to: string }>
}

/*
YS I.42 — Savitarka (with-marks): word–meaning–knowledge mixed
Vitarka’s first species: immediate determinacy with admixture (śabda–artha–jñāna).
Note: artha = meaning.
*/
export const CHUNKS_I_42_SUTRA = [
  {
    id: 'ys-i-42-savitarka',
    title: 'I.42 — Savitarka (with-marks): word–meaning–knowledge mixed',
    text:
      'Immediate determinacy (vitarka) “with-marks”: cognition interwoven of word (śabda), meaning (artha), and knowledge/ideation (jñāna/kalpanā); mixture yields a factical immediacy of Being through signs.'
  }
]

export const HLOS_I_42_SUTRA = [
  {
    id: 'ys-i-42-op-savitarka',
    chunkId: 'ys-i-42-savitarka',
    label: 'Savitarka Samāpatti — mixed word–meaning–knowledge (immediate-with-marks)',
    clauses: [
      // Method tags
      "tag('lens','yoga')",
      "tag('method','samapatti')",
      "tag('mode','vitarka')",
      "tag('phase','savitarka')",
      "tag('plane','dyadic')",
      "tag('role','original')",
      "tag('faculty','manas')",
      "tag('cycle','ys:vitarka:A.1')",
      "tag('order','1')",

      // Phenomenology
      'mix(śabda, artha, jñāna) ⇒ immediacy(withMarks)',
      'determine(Being) via signs(śabda) anchoredIn(artha) under(jñāna)',
      'factical(immediacy) because admixture(śabda–artha–jñāna)'
    ]
  }
]

export const YS_I_42_SUTRA_UNIT: DatasetUnit = {
  id: makeUnitId('i.42'),
  title: 'YS I.42 — Vitarka (Savitarka): Light as First Principle',
  scope: 'being-only',
  logosMode: 'prajna',
  synthesis: 'pre-factum',
  faculty: 'manas',
  lens: 'fichte',
  chunks: CHUNKS_I_42_SUTRA as any,
  hlos: HLOS_I_42_SUTRA as any,
}
