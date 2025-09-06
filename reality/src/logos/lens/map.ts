export const DATASET = {
  id: 'dataset:fichte-logologia-lens',
  title: 'Fichte — Logologia Lens (Samāpatti pipeline)',
  provenance: { lens: 'fichte', source: 'reality/src/logologia/logos.ts' },
  chunks: [
    { id: 'fichte-1804-cycle-a', source: '../../logologia/logos.ts' }
  ],
  terms: [
    // method umbrella + modes + species
    { id: 'samapatti', label: 'Samāpatti', aliases: ['sattva','समापत्ति'] },
    { id: 'vitarka', label: 'Vitarka', aliases: ['वितर्क'] },
    { id: 'vicara',  label: 'Vicāra',  aliases: ['विचार'] },
    { id: 'savitarka', label: 'Savitarka', aliases: ['सवितर्क'] },
    { id: 'nirvitarka', label: 'Nirvitarka', aliases: ['निर्वितर्क'] },
    { id: 'savicara', label: 'Savicāra', aliases: ['सविचार'] },
    { id: 'nirvicara', label: 'Nirvicāra', aliases: ['निर्विचार'] },

    // faculties
    { id: 'manas', label: 'Manas (Understanding)', aliases: ['मनस्'] },
    { id: 'buddhi', label: 'Buddhi (Reason)', aliases: ['बुद्धि'] },

    // principle–model–resolution
    { id: 'prajna', label: 'Prajñā (Principle)', aliases: ['प्रज्ञा'] },
    { id: 'samskara', label: 'Saṃskāra (Model)', aliases: ['संस्कार'] },
    { id: 'nirodha', label: 'Nirodha (Resolution)', aliases: ['निरोध'] }
  ],
  hlos: []
}
