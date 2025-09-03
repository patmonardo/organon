import type { Provenance } from './index'

export type BeingMoment = 'Sat' | 'Sunya' | 'Bhava'
export type ConsciousnessMoment = 'Cit' | 'Citi' | 'Citta'

export interface LensTag {
  id: string
  appliesTo: 'chunk' | 'op'
  targetId: string
  tags: string[]
  note?: string
  provenance?: Provenance
  confidence?: 'low' | 'medium' | 'high'
}

export const LENS_TAGS: LensTag[] = [
  // Bhava ↔ Citta — Sublation of Becoming (confirmed)
  {
    id: 'lens-ref2-sublation-of-becoming',
    appliesTo: 'op',
    targetId: 'ess-ref2-op-12-sublation-of-becoming',
    tags: [
      'Theme:SublationOfBecoming',
      'BeingMoment:Bhava',
      'ConsciousnessMoment:Citta',
      'Pattern:InternalReferenceToOtherness',
      'Pattern:SublatedPositedness',
      'Pattern:InfiniteSelfReference'
    ],
    note: 'Citta as Truth of Bhava via sublation of positedness and internalized otherness.',
    confidence: 'high'
  },

  // Sunya ↔ Citi — Shine (proposed; verify on review)
  {
    id: 'lens-shine-sunya-citi',
    appliesTo: 'chunk',
    targetId: 'ess-sh-1-being-is-shine',
    tags: [
      'BeingMoment:Sunya',
      'ConsciousnessMoment:Citi',
      'Theme:ReflectedImmediacy',
      'Tag:NonSelfSubsistence'
    ],
    note: 'Shine read as Sunya; Citi as reflective imaging. To be validated in Second Pass.',
    confidence: 'low'
  },

  // Sat ↔ Cit — Absolute reflection (proposed; medium)
  {
    id: 'lens-ref-positing-sat-cit',
    appliesTo: 'chunk',
    targetId: 'ess-ref-1-positing-reflection-intro',
    tags: [
      'BeingMoment:Sat',
      'ConsciousnessMoment:Cit',
      'Theme:AbsoluteReflection'
    ],
    note: 'Positing reflection as Sat/Cit articulation of equality-with-self.',
    confidence: 'medium'
  },

  // Absolute Knowing — Being:Essence unity (anchor on result of external→immanent)
  {
    id: 'lens-absolute-knowing-unity',
    appliesTo: 'op',
    targetId: 'ess-ref1-op-7-externality-sublated-determining',
    tags: [
      'Theme:AbsoluteKnowing',
      'Unity:Being~Essence',
      'Object:Unity(Self-within-Other)',
      'Result:EssenceInAndForItself'
    ],
    note: 'Absolute Knowing read as unity of Being and Essence (coinciding with immediate).',
    confidence: 'medium'
  }
]

export function getLensTagsFor(id: string): LensTag[] {
  return LENS_TAGS.filter(t => t.targetId === id)
}
export function byTag(tag: string): LensTag[] {
  return LENS_TAGS.filter(t => t.tags.includes(tag))
}
