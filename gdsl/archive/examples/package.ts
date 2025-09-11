import { View, validateView } from '../registry/canon'

export const REALITY_VIEW: View = {
  id: 'view:@reality',
  title: '@reality — Transcendental Idealism (Vitarka + SATTVA + Hegel bridge)',
  terms: [
    { id: 'AbsoluteDialectic', label: 'Absolute Dialectic', aliases: [] },
    { id: 'AbsoluteReason', label: 'Absolute Reason', aliases: [] },
    { id: 'Relative', label: 'Relative', aliases: [] },
    { id: 'HegelSystemOfSystems', label: 'Hegel — System of Systems', aliases: ['Hegel'] },
    { id: 'HegelDialectic', label: 'Hegel — Dialectic', aliases: [] },
    { id: 'SATTVA', label: 'Sattva', aliases: ['sattva'] },
    { id: 'PrincipleOfLight', label: 'Principle of Light (Vitarka)', aliases: [] },
  ],
  edges: [
    { type: 'EXPLAINS', from: 'SATTVA', to: 'AbsoluteDialectic' },
    { type: 'PROVIDES_PRINCIPLES_TO', from: 'AbsoluteReason', to: 'HegelSystemOfSystems' },
    { type: 'DESCENDS_TO', from: 'HegelDialectic', to: 'Relative' },
    { type: 'ALIGNS_WITH', from: 'PrincipleOfLight', to: 'AbsoluteDialectic' },
  ],
  refs: [
    'reality:logos:ys:i.42',
    'reality:logos:ys:i.43',
  ],
}

validateView(REALITY_VIEW)
