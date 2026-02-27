import * as fs from 'fs';
import * as path from 'path';

const baseDir = path.resolve('./reality/src/logos/Reference/yoga-sutras');

// --- APPEARANCE / RELATION ---
const relatPath = path.join(baseDir, 'appearance', 'relation');

const relatSanskrit = `
# YOGA-SUTRAS-RELATION-SANSKRIT-WORKBOOK

Status: Translation populated
Doctrine scope: APPEARANCE
Subject: SANSKRIT

## Purpose
Establishing YS 4.23 (The mind colored by both seer and seen) as Appearance/Relation.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK YS_4_23_CITTAM_SARVARTHAM
\`\`\`sanskrit
द्रष्टृदृश्योपरक्तं चित्तं सर्वार्थम्॥ ४.२३॥
\`\`\`
**Transliteration:** draṣṭṛdṛśyoparaktaṃ cittaṃ sarvārtham || 4.23 ||

**Translation:** The mind, colored by the Seer and the Seen, understands everything.

- **id:** \`ys-4-23-seer-seen-relation\`
- **relations:**
  - \`SUBLATES\` -> \`ys-4-19-mind-not-self-illuminating\`
  - \`NEXT\` -> \`ys-4-24-citta-for-another\`
`;

const relatHegel = `
# YOGA-SUTRAS-RELATION-HEGEL-WORKBOOK

Status: Translation populated
Doctrine scope: APPEARANCE
Subject: HEGEL

## Purpose
Mapping the Seer-Seen relation (YS 4.23) to Hegel's Appearance/Relation.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK HEGEL_RELATION_SEER_SEEN
**Alignment:** In Hegel's Essential Relation, the two sides (Inner and Outer, Whole and Parts) exist only in their relation to each other. The Citta (Mind) is the medium, the relation itself, between the absolute Purusha (Seer) and Prakriti (Seen). The mind encompasses 'everything' ('sarvartham') because it is the comprehensive relation holding the objective universe and the subjective awareness together in the sphere of Appearance.

- **id:** \`hegel-appearance-relation-citta\`
- **relations:**
  - \`REFLECTS\` -> \`ys-4-23-seer-seen-relation\`
`;

const relatFichte = `
# YOGA-SUTRAS-RELATION-FICHTE-WORKBOOK

Status: Translation populated
Doctrine scope: APPEARANCE
Subject: FICHTE

## Purpose
Mapping Fichte's knowing to YS 4.23.

## Core edges
- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\`

## Outline

### CHUNK FICHTE_RELATION_SEER_SEEN
**Alignment:** The positing of the Not-I by the I (and vice versa) creates the sphere of experience. The 'citta' here is the mediating faculty of imagination (Einbildungskraft) that is colored simultaneously by the pure subject and the intended object, forming the total relation of Knowing.

- **id:** \`fichte-appearance-relation-citta\`
- **relations:**
  - \`REFLECTS\` -> \`ys-4-23-seer-seen-relation\`
`;

fs.writeFileSync(path.join(relatPath, 'SANSKRIT-COMPILER-WORKBOOK.md'), relatSanskrit.trim());
fs.writeFileSync(path.join(relatPath, 'HEGEL-COMPILER-WORKBOOK.md'), relatHegel.trim());
fs.writeFileSync(path.join(relatPath, 'FICHTE-COMPILER-WORKBOOK.md'), relatFichte.trim());

console.log("Written translation and commentary for Appearance (Relation).");
