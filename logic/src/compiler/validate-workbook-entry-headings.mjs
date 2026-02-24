#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';

const rootArg = process.argv[2] ?? '.';
const rootDir = path.resolve(process.cwd(), rootArg);

const FILE_NAME_PATTERN = /PART.*WORKBOOK\.md$/i;
const ENTRY_HEADING = /^###\s+Entry\s+(.+?)\s+—\s+(.+)$/;

const ID_PATTERNS = {
  regular: /^[a-z]{3}-[a-z]{3}-\d{3}$/,
  subgroup: /^[a-z]{3}-[a-z]{3}-[abc]$/,
  subentry: /^[a-z]{3}-[a-z]{3}-[abc]-\d{3}$/,
};

const NOISY_TITLE_PREFIXES = [
  /^Section\s+\d+\s*:/i,
  /^Block\s+\d+\s*:/i,
  /^Paragraph\s+\d+\s*:/i,
  /^Marker\b/i,
  /^[abc]\s*\.\s*\d+\b/i,
  /^[abc]-\d{3}\b/i,
  /^\d+\./,
];

function walk(dir) {
  const out = [];
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  for (const entry of entries) {
    if (
      entry.name === 'node_modules' ||
      entry.name === '.git' ||
      entry.name === 'target'
    ) {
      continue;
    }
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      out.push(...walk(full));
      continue;
    }
    if (FILE_NAME_PATTERN.test(entry.name)) {
      out.push(full);
    }
  }
  return out;
}

function isValidId(id) {
  return (
    ID_PATTERNS.regular.test(id) ||
    ID_PATTERNS.subgroup.test(id) ||
    ID_PATTERNS.subentry.test(id)
  );
}

function wordCount(title) {
  return title.trim().split(/\s+/).filter(Boolean).length;
}

function hasNoisyPrefix(title) {
  return NOISY_TITLE_PREFIXES.some((rx) => rx.test(title));
}

function analyzeFile(filePath) {
  const text = fs.readFileSync(filePath, 'utf8');
  const lines = text.split(/\r?\n/);
  const problems = [];
  const warnings = [];

  lines.forEach((line, index) => {
    if (!line.startsWith('### Entry')) {
      return;
    }

    const lineNo = index + 1;
    const match = line.match(ENTRY_HEADING);

    if (!match) {
      problems.push({
        filePath,
        lineNo,
        type: 'heading-shape',
        message: 'Heading must match: ### Entry <id> — <core title>',
        sample: line.trim(),
      });
      return;
    }

    const id = match[1].trim();
    const title = match[2].trim();

    if (!isValidId(id)) {
      problems.push({
        filePath,
        lineNo,
        type: 'id-format',
        message:
          'ID must be one of: ddd-ppp-001 | ddd-ppp-a | ddd-ppp-a-001 (d/p are 3 lowercase letters)',
        sample: id,
      });
    }

    if (/[()]/.test(id)) {
      problems.push({
        filePath,
        lineNo,
        type: 'id-parentheses',
        message: 'ID cannot contain parentheses',
        sample: id,
      });
    }

    if (hasNoisyPrefix(title)) {
      problems.push({
        filePath,
        lineNo,
        type: 'title-prefix-noise',
        message:
          'Title starts with structural prefix noise (Section/Block/Paragraph/a.1/etc.)',
        sample: title,
      });
    }

    const wc = wordCount(title.replace(/[`“”"']/g, ''));
    if (wc < 2 || wc > 5) {
      warnings.push({
        filePath,
        lineNo,
        type: 'title-word-count',
        message: `Title has ${wc} words (target 2–5, with 3 preferred)`,
        sample: title,
      });
    }
  });

  return { problems, warnings };
}

const files = walk(rootDir);
const allProblems = [];
const allWarnings = [];

for (const file of files) {
  const { problems, warnings } = analyzeFile(file);
  allProblems.push(...problems);
  allWarnings.push(...warnings);
}

function printIssue(issue) {
  const rel = path.relative(process.cwd(), issue.filePath) || issue.filePath;
  console.log(`${rel}:${issue.lineNo} [${issue.type}] ${issue.message}`);
  console.log(`  -> ${issue.sample}`);
}

if (allProblems.length === 0) {
  console.log('No hard violations found.');
} else {
  console.log(`Hard violations: ${allProblems.length}`);
  allProblems.forEach(printIssue);
}

if (allWarnings.length > 0) {
  console.log(`\nSoft warnings: ${allWarnings.length}`);
  allWarnings.forEach(printIssue);
}

if (allProblems.length > 0) {
  process.exitCode = 1;
}
