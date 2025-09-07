import { View, validateView, makeViewId } from '@organon/gdsl/registry/canon'

export const FICHTE_LOGOLOGIA_LENS_VIEW: View = {
  id: makeViewId('lenses:fichte-logologia'),
  title: 'Fichte — Logologia Lens',
  terms: [
    { id: 'samapatti', label: 'Samāpatti', aliases: ['sattva','समापत्ति'] },
    { id: 'vitarka', label: 'Vitarka', aliases: ['वितर्क'] },
    { id: 'vicara',  label: 'Vicāra',  aliases: ['विचार'] },
    { id: 'savitarka', label: 'Savitarka', aliases: ['सवितर्क'] },
    { id: 'nirvitarka', label: 'Nirvitarka', aliases: ['निर्वितर्क'] },
    { id: 'savicara', label: 'Savicāra', aliases: ['सविचार'] },
    { id: 'nirvicara', label: 'Nirvicāra', aliases: ['निर्विचार'] },
    { id: 'manas', label: 'Manas (Understanding)', aliases: ['मनस्'] },
    { id: 'buddhi', label: 'Buddhi (Reason)', aliases: ['बुद्धि'] },
    { id: 'prajna', label: 'Prajñā (Principle)', aliases: ['प्रज्ञा'] },
    { id: 'samskara', label: 'Saṃskāra (Model)', aliases: ['संस्कार'] },
    { id: 'nirodha', label: 'Nirodha (Resolution)', aliases: ['निरोध'] }
  ],
  edges: [],
  // refs: ['reality:logos:...'] // add unit/view ids if you want cross-links
}

validateView(FICHTE_LOGOLOGIA_LENS_VIEW)
