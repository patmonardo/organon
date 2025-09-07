import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

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
YS I.43 — Nirvitarka (without-marks): memory-purified, object-only shining
Vitarka’s second species: immediate determinacy without admixture (artha-mātra-nirbhāsa).
*/
export const CHUNKS_I_43_SUTRA = [
  {
    id: 'ys-i-43-nirvitarka',
    title: 'I.43 — Nirvitarka (without-marks): memory-purified, meaning-only shining',
    text:
      'When memory-impressions are purified (smṛti-pariśuddhi), the meaning shines as meaning-only (artha-mātra-nirbhāsa), as if devoid of its own form; the same immediate determinacy (vitarka) now without admixture of sign/ideation.'
  }
]

export const HLOS_I_43_SUTRA = [
  {
    id: 'ys-i-43-op-nirvitarka',
    chunkId: 'ys-i-43-nirvitarka',
    label: 'Nirvitarka Samāpatti — meaning-only shining (immediate-without-marks)',
    clauses: [
      // Method tags
      "tag('lens','yoga')",
      "tag('method','samapatti')",
      "tag('mode','vitarka')",
      "tag('phase','nirvitarka')",
      "tag('plane','dyadic')",
      "tag('role','judgment')",
      "tag('faculty','buddhi')",
      "tag('cycle','ys:vitarka:A.1')",
      "tag('order','2')",

      // Phenomenology (artha = meaning)
      'purify(smṛti) ⇒ drop(admixture: śabda, jñāna)',
      'shine(meaningOnly) ⇐ artha_mātra_nirbhāsa',
      'immediacy(withoutMarks) = vitarka − {śabda, jñāna}',
    ],
    predicates: [
      { name: 'Purify', args: ['smṛti'] },
      { name: 'ShinesAs', args: ['meaning', 'meaningOnly'] },
      { name: 'Immediate', args: ['nirvitarka'] }
    ],
    relations: [
      { predicate: 'removes', from: 'nirvitarka', to: 'śabda' },
      { predicate: 'removes', from: 'nirvitarka', to: 'jñāna' },
      { predicate: 'retains', from: 'nirvitarka', to: 'artha' } // artha = meaning
    ]
  }
]

export const YS_I_43_SUTRA_UNIT: DatasetUnit = {
  id: makeUnitId('i.43'),
  title: 'YS I.43 — Vitarka (Nirvitarka): Meaning-only shining',
  scope: 'being-only',
  logosMode: 'prajna',
  synthesis: 'pre-factum',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_I_43_SUTRA as any,
  hlos: HLOS_I_43_SUTRA as any,
}
