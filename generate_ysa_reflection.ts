import * as fs from 'fs';
import * as path from 'path';

const baseDir = path.resolve('./reality/src/logos/Reference/yoga-sutras');

// --- REFLECTION / GROUND ---
const groundPath = path.join(baseDir, 'reflection', 'ground');

const groundSanskrit = `
# YOGA-SUTRAS-GROUND-SANSKRIT-WORKBOOK

Status: Translation populated
Doctrine scope: REFLECTION
Subject: SANSKRIT

## Purpose
Establishing YS 4.11 (The foundation of Vasanas) as Reflection/Ground.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK YS_4_11_HETU_PHALA_ASRAYA
\`\`\`sanskrit
हेतुफलाश्रयालम्बनैः सङ्गृहीतत्वादेषामभावे तदभावः ॥ ४.११॥
\`\`\`
**Transliteration:** hetuphalāśrayālambanaiḥ saṅgṛhītatvādeṣāmabhāve tadabhāvaḥ || 4.11 ||

**Translation:** Because these (desires/vasanas) are bound together by cause (hetu), motive/result (phala), substrate (aśraya), and object (ālambana), the disappearance of these four leads to the disappearance of the vasanas.

- **id:** \`ys-4-11-foundation-of-desire\`
- **relations:**
  - \`SUBLATES\` -> \`ys-4-10-beginningless-desire\`
  - \`NEXT\` -> \`ys-4-12-past-future-exist\`
`;

const groundHegel = `
# YOGA-SUTRAS-GROUND-HEGEL-WORKBOOK

Status: Translation populated
Doctrine scope: REFLECTION
Subject: HEGEL

## Purpose
Mapping the causal foundation of Vasanas (YS 4.11) to Hegel's Ground (Grund).

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK HEGEL_GROUND_VASANA
**Alignment:** In Hegel, 'Ground' is the unity of identity and difference; it is that which bears the condition and the conditioned. YS 4.11 explicitly lists the conditions (hetu, phala, asraya, alambana) serving as the 'Ground' for the seemingly eternal Vasanas (subconscious impressions). When the condition (the Ground) is sublated or removed, the conditioned (the vasanas) dissolve. The Vasanas have no independent Absolute Actuality—they are merely Grounded phenomena within Reflection.

- **id:** \`hegel-reflection-ground-vasanas\`
- **relations:**
  - \`REFLECTS\` -> \`ys-4-11-foundation-of-desire\`
`;

const groundFichte = `
# YOGA-SUTRAS-GROUND-FICHTE-WORKBOOK

Status: Translation populated
Doctrine scope: REFLECTION
Subject: FICHTE

## Purpose
Mapping Fichte's ground of the Anstoss to YS 4.11.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK FICHTE_GROUND_VASANA
**Alignment:** Fichte argues that the ego encounters a 'check' (Anstoss) which grounds the production of the objective world. The conditions (hetu/phala) are the structural requirements of the I attempting to posit itself infinitely but being checked. Only through recognizing and dissolving the illusion of the independent 'check' (the Ground of the non-I) does the I return to absolute Freedom. 

- **id:** \`fichte-reflection-ground-vasanas\`
- **relations:**
  - \`REFLECTS\` -> \`ys-4-11-foundation-of-desire\`
`;

fs.writeFileSync(path.join(groundPath, 'SANSKRIT-COMPILER-WORKBOOK.md'), groundSanskrit.trim());
fs.writeFileSync(path.join(groundPath, 'HEGEL-COMPILER-WORKBOOK.md'), groundHegel.trim());
fs.writeFileSync(path.join(groundPath, 'FICHTE-COMPILER-WORKBOOK.md'), groundFichte.trim());

console.log("Written translation and commentary for Reflection (Ground).");
