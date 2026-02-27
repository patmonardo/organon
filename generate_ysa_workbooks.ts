import * as fs from 'fs';
import * as path from 'path';

const baseDir = path.resolve('./reality/src/logos/Reference/yoga-sutras');

const categories = {
  reflection: ['essence', 'foundation', 'ground'],
  appearance: ['thing', 'world', 'relation'],
  actuality: ['absolute', 'actuality', 'substance']
};

const workbooks = ['SANSKRIT-COMPILER-WORKBOOK.md', 'HEGEL-COMPILER-WORKBOOK.md', 'FICHTE-COMPILER-WORKBOOK.md'];

function template(folder: string, type: string): string {
  const content = [
    `# YOGA-SUTRAS-${folder.toUpperCase()}-${type}-WORKBOOK\n`,
    `Status: Scaffold workbook (translation in progress)`,
    `Doctrine scope: ${folder.toUpperCase()}`,
    `Subject: ${type.toUpperCase()}\n`,
    `## Purpose`,
    `Establish the generator-first contract mapping Yoga Sutras Kaivalya/Vibhuti padas to ${type} Logic within the scope of ${folder}.\n`,
    `## Core edges`,
    `- \`NEGATES\`, \`SUBLATES\`, \`MEDIATES\`, \`REFLECTS\`, \`NEXT\``,
    `- Validity edges: \`VALID_IN\`, \`SUPPORTED_BY\`, \`CONDITIONED_BY\`, \`REJECTED_BY\`\n`,
    `## Outline\n`,
    `<!-- TODO: Expand chunk logic here -->`,
  ];
  return content.join('\n');
}

Object.entries(categories).forEach(([major, minors]) => {
  minors.forEach((minor) => {
    const dirPath = path.join(baseDir, major, minor);
    workbooks.forEach((wb) => {
      const type = wb.split('-')[0];
      const filePath = path.join(dirPath, wb);
      fs.writeFileSync(filePath, template(minor, type));
      console.log(`Created ${filePath}`);
    });
  });
});

