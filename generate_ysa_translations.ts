import * as fs from 'fs';
import * as path from 'path';

// This script will inject some sample content into the workbooks to start the translation process
// specifically targeting the culmination in Dharma Megha Samadhi (Actuality/Substance in this schema).
const baseDir = path.resolve('./reality/src/logos/Reference/yoga-sutras');

// Example: Map Kaivalya Pada Sutra 4.29 (Dharma Megha) to "actuality/substance"
const dharmaMeghaPath = path.join(baseDir, 'actuality', 'substance');

const sanskritContent = `
# YOGA-SUTRAS-SUBSTANCE-SANSKRIT-WORKBOOK

Status: Translation populated
Doctrine scope: ACTUALITY
Subject: SANSKRIT

## Purpose
Establishing the Sanskrit and philosophical translation of YS 4.29 (Dharma Megha Samadhi) as Substance/Reciprocity in the Logic cycle.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`
- Validity edges: \`VALID_IN\`, \`SUPPORTED_BY\`, \`CONDITIONED_BY\`, \`REJECTED_BY\`

## Outline

### CHUNK YS_4_29_DHARMA_MEGHA
\`\`\`sanskrit
प्रसङ्ख्यानेऽप्यकुसीदस्य सर्वथा विवेकख्यातेर्धर्ममेघः समाधिः ॥ ४.२९॥
\`\`\`
**Transliteration:** prasaṅkhyāne'pyakusīdasya sarvathā vivekakhyāterdharmameghaḥ samādhiḥ || 4.29 ||

**Translation:** For one who is free from all desire, even the highest states of insight (prasaṅkhyāna), there arises—through continuous, undistracted discernment (vivekakhyāti)—the samadhi known as a 'cloud of dharma' (Dharma Megha).

- **id:** \`ys-4-29-dharma-megha\`
- **relations:**
  - \`SUBLATES\` -> \`ys-4-25-special-vision\`
  - \`MEDIATES\` -> \`ys-4-30-klesha-karma-nivritti\`
`;

fs.writeFileSync(path.join(dharmaMeghaPath, 'SANSKRIT-COMPILER-WORKBOOK.md'), sanskritContent.trim());

const hegelContent = `
# YOGA-SUTRAS-SUBSTANCE-HEGEL-WORKBOOK

Status: Translation populated
Doctrine scope: ACTUALITY
Subject: HEGEL

## Purpose
Demonstrating that Hegel's Logic never separates itself from the Path to Truth by aligning the peak of his Objective Logic (Actuality/Substance) with Dharma Megha Samadhi. Hegel raises Dharma qua Sara to Dharma Megha.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK HEGEL_SUBSTANCE_DHARMA_MEGHA
**Alignment:** Dharma Megha Samadhi corresponds to the consummation of Substance into the Concept (the Absolute).
Hegel's Logic treats the absolute relation (Substance, Causality, Reciprocity) as the threshold to the Concept. 
Dharma Megha ('Cloud of Virtue' or 'Raincloud of Knowable Truth') is precisely the "Idea" in Hegel, where the logical form is no longer abstracted from content ('jnana'). Here, Logic *is* Science *is* Life. The separation of Logic from material, which Kant upholds, is sublated here. 

- **id:** \`hegel-actuality-substance-dharmamegha\`
- **relations:**
  - \`REFLECTS\` -> \`ys-4-29-dharma-megha\`
  - \`SUBLATES\` -> \`kantian-abstract-reason\`
`;

fs.writeFileSync(path.join(dharmaMeghaPath, 'HEGEL-COMPILER-WORKBOOK.md'), hegelContent.trim());

const fichteContent = `
# YOGA-SUTRAS-SUBSTANCE-FICHTE-WORKBOOK

Status: Translation populated
Doctrine scope: ACTUALITY
Subject: FICHTE

## Purpose
Mapping Fichte's 1804 Science of Knowing (Wissenschaftslehre) to Dharma Megha Samadhi.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK FICHTE_KNOWING_DHARMA_MEGHA
**Alignment:** Fichte's system culminates in the insight that true Knowing is the immediate life of the Absolute itself. Fichte's 'Wissenschaftslehre' bridges the gap between pure logic and lived realization.
In Dharma Megha Samadhi, Fichte's "Seeing" (Sicht) recognizes that it is nothing but the manifestation of the Absolute. This continuous, undistracted discernment (Viveka-Khyati) perfectly maps to Fichte's constant genetic construction of truth.

- **id:** \`fichte-science-of-knowing-dharmamegha\`
- **relations:**
  - \`REFLECTS\` -> \`ys-4-29-dharma-megha\`
  - \`REFLECTS\` -> \`hegel-actuality-substance-dharmamegha\`
`;

fs.writeFileSync(path.join(dharmaMeghaPath, 'FICHTE-COMPILER-WORKBOOK.md'), fichteContent.trim());

console.log("Written translation and commentary for Dharma Megha (Substance).");
