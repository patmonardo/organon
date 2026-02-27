import * as fs from 'fs';
import * as path from 'path';

const baseDir = path.resolve('./reality/src/logos/Reference/yoga-sutras');

// --- ACTUALITY / ABSOLUTE ---
const absolutePath = path.join(baseDir, 'actuality', 'absolute');

const absoluteSanskrit = `
# YOGA-SUTRAS-ABSOLUTE-SANSKRIT-WORKBOOK

Status: Translation populated
Doctrine scope: ACTUALITY
Subject: SANSKRIT

## Purpose
Establishing the Sanskrit and philosophical translation of YS 4.25 (Cessation of self-reflection) as Actuality/Absolute.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK YS_4_25_VISESHA_DARSHINAH
\`\`\`sanskrit
विशेषदर्शिन आत्मभावभावनाविनिवृत्तिः ॥ ४.२५॥
\`\`\`
**Transliteration:** viśeṣadarśina ātmabhāvabhāvanāvinivṛttiḥ || 4.25 ||

**Translation:** For one who sees the distinction (viśeṣadarśina), the contemplation on the nature of the self (ātmabhāva) completely ceases.

- **id:** \`ys-4-25-special-vision\`
- **relations:**
  - \`SUBLATES\` -> \`ys-4-21-infinite-regress-citta\`
  - \`NEXT\` -> \`ys-4-29-dharma-megha\`
`;

const absoluteHegel = `
# YOGA-SUTRAS-ABSOLUTE-HEGEL-WORKBOOK

Status: Translation populated
Doctrine scope: ACTUALITY
Subject: HEGEL

## Purpose
Mapping the cessation of self-reflection (YS 4.25) to Hegel's Absolute Actuality.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK HEGEL_ABSOLUTE_CESSATION
**Alignment:** In Hegel's movement to the Absolute, reflection returns entirely into itself. The separation between the observing consciousness and the observed self ceases. Actuality as Absolute is completely self-evident and requires no external "contemplation" to sustain its self-certainty. 
This is the philosophical rigorousness behind YS 4.25: The infinite regress of mind reflecting on mind (YS 4.21) is sublated when the distinction of the absolute Purusha is seen. 

- **id:** \`hegel-actuality-absolute-cessation\`
- **relations:**
  - \`REFLECTS\` -> \`ys-4-25-special-vision\`
`;

const absoluteFichte = `
# YOGA-SUTRAS-ABSOLUTE-FICHTE-WORKBOOK

Status: Translation populated
Doctrine scope: ACTUALITY
Subject: FICHTE

## Purpose
Mapping Fichte's 1804 absolute reflection to YS 4.25.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK FICHTE_ABSOLUTE_CESSATION
**Alignment:** For Fichte, the ego (I) must eventually recognize that all self-positing is anchored in an un-posited Absolute. The anxious striving to 'know oneself' objectively ceases when the 'I' realizes it is the immediate expression of the Light. The *Atmabhava-bhavana-vinivritti* (cessation of dwelling on the self) matches Fichte's dissolution of the individual ego's reflective obsession into pure Knowing.

- **id:** \`fichte-actuality-absolute-cessation\`
- **relations:**
  - \`REFLECTS\` -> \`ys-4-25-special-vision\`
`;

fs.writeFileSync(path.join(absolutePath, 'SANSKRIT-COMPILER-WORKBOOK.md'), absoluteSanskrit.trim());
fs.writeFileSync(path.join(absolutePath, 'HEGEL-COMPILER-WORKBOOK.md'), absoluteHegel.trim());
fs.writeFileSync(path.join(absolutePath, 'FICHTE-COMPILER-WORKBOOK.md'), absoluteFichte.trim());


// --- ACTUALITY / ACTUALITY ---
const actPath = path.join(baseDir, 'actuality', 'actuality');

const actSanskrit = `
# YOGA-SUTRAS-ACTUALITY-SANSKRIT-WORKBOOK

Status: Translation populated
Doctrine scope: ACTUALITY
Subject: SANSKRIT

## Purpose
Establishing YS 4.30-31 (Cessation of Kleshas and Karma / The infinity of Knowledge) as Actuality/Actuality.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK YS_4_30_KLESHA_NIVRITTI
\`\`\`sanskrit
ततः क्लेशकर्मनिवृत्तिः ॥ ४.३०॥
\`\`\`
**Transliteration:** tataḥ kleśakarmanivṛttiḥ || 4.30 ||

**Translation:** From that (Dharma Megha Samadhi), there is the cessation of afflictions (klesha) and actions (karma).

### CHUNK YS_4_31_JNANA_ANANTYA
\`\`\`sanskrit
तदा सर्वावरणमलापेतस्य ज्ञानस्यानन्त्याज्ज्ञेयमल्पम्॥ ४.३१॥
\`\`\`
**Transliteration:** tadā sarvāvaraṇamalāpetasya jñānasyānantyājjñeyamalpam || 4.31 ||

**Translation:** Then, because of the infinity of knowledge free from all obscuring impurities, that which is left to be known becomes insignificant.

- **id:** \`ys-4-30-klesha-karma-nivritti\`
- **relations:**
  - \`SUBLATES\` -> \`ys-4-7-karma-types\`
  - \`NEXT\` -> \`ys-4-32-gunas-end\`
`;

const actHegel = `
# YOGA-SUTRAS-ACTUALITY-HEGEL-WORKBOOK

Status: Translation populated
Doctrine scope: ACTUALITY
Subject: HEGEL

## Purpose
Mapping the cessation of Kleshas (YS 4.30) to Hegel's Actuality.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK HEGEL_ACTUALITY_INFINITY
**Alignment:** Hegel defines Actuality as the unity of Essence and Existence where all inner potential has become outward realization. 
The cessation of Kleshas (afflictions) and Karma represents the exhaustion of blind, mechanical 'causes' driving action. All action is now entirely free, self-determined Actuality. The 'infinity of knowledge' (YS 4.31) mirrors Hegel's 'True Infinity', where nothing remains outside the Concept 'to be known' externally—the knower and known are fully synthesized.

- **id:** \`hegel-actuality-actuality-infinity\`
- **relations:**
  - \`REFLECTS\` -> \`ys-4-30-klesha-karma-nivritti\`
`;

const actFichte = `
# YOGA-SUTRAS-ACTUALITY-FICHTE-WORKBOOK

Status: Translation populated
Doctrine scope: ACTUALITY
Subject: FICHTE

## Purpose
Mapping Fichte's 1804 absolute knowing to YS 4.30.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK FICHTE_ACTUALITY_INFINITY
**Alignment:** Fichte's true Knowing leaves no residue. The 'veil' of phenomena (avaraṇa) is fully pierced. The "infinitude of knowledge" perfectly expresses Fichte's realization that the Knowing is itself the infinite life of the Absolute, unconstrained by the finite objects (which become 'insignificant' or 'alpam').

- **id:** \`fichte-actuality-actuality-infinity\`
- **relations:**
  - \`REFLECTS\` -> \`ys-4-30-klesha-karma-nivritti\`
`;

fs.writeFileSync(path.join(actPath, 'SANSKRIT-COMPILER-WORKBOOK.md'), actSanskrit.trim());
fs.writeFileSync(path.join(actPath, 'HEGEL-COMPILER-WORKBOOK.md'), actHegel.trim());
fs.writeFileSync(path.join(actPath, 'FICHTE-COMPILER-WORKBOOK.md'), actFichte.trim());

console.log("Written translation and commentary for Absolute and Actuality.");
