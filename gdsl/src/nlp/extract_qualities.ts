import { readFileSync } from 'node:fs';
import { atom, fact, rule, v, str, type Program } from '../core';
import {
  quality,
  opposes,
  dominates,
} from '../domains/quality';

// Heuristics: find "A vs B", "A versus B", "either A or B", "A not B"
function norm(w: string) {
  return w
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-');
}

function findOppositions(text: string): Array<[string, string]> {
  const pairs: Array<[string, string]> = [];
  const vs = text.matchAll(
    /\b([A-Za-z][\w-]{1,30})\s*(?:vs\.?|versus)\s*([A-Za-z][\w-]{1,30})/gi,
  );
  for (const m of vs) pairs.push([norm(m[1]), norm(m[2])]);

  const either = text.matchAll(
    /\beither\s+([A-Za-z][\w-]{1,30})\s+or\s+([A-Za-z][\w-]{1,30})/gi,
  );
  for (const m of either) pairs.push([norm(m[1]), norm(m[2])]);

  const nots = text.matchAll(
    /\b([A-Za-z][\w-]{1,30})\s+not\s+([A-Za-z][\w-]{1,30})/gi,
  );
  for (const m of nots) pairs.push([norm(m[1]), norm(m[2])]);

  return Array.from(new Set(pairs.map((p) => JSON.stringify(p)))).map((s) =>
    JSON.parse(s),
  );
}

function findQualities(text: string, entity: string): string[] {
  // naive: adjectives near "is"/"as"
  const qs = new Set<string>();
  const cop = text.matchAll(
    /\b(?:is|as|are|appears|shows)\s+([A-Za-z][\w-]{2,30})\b/gi,
  );
  for (const m of cop) qs.add(norm(m[1]));
  return Array.from(qs);
}

const file = process.argv[2];
const entity = norm(process.argv[3] ?? 'subject'); // optional entity id
if (!file) {
  console.error(
    'usage: tsx src/nlp/extract_qualities.ts path/to/text.txt [entityId]',
  );
  process.exit(1);
}

const txt = readFileSync(file, 'utf8');
const facts: Program['facts'] = [];
const S = v(entity);

for (const q of findQualities(txt, entity)) {
  facts.push(fact(quality(S, q)));
}
// seed dominance if a single quality stands out (heuristic: first)
const firstQ = findQualities(txt, entity)[0];
if (firstQ) facts.push(fact(dominates(S, firstQ)));

for (const [a, b] of findOppositions(txt)) {
  facts.push(fact(opposes(a, b)));
}

const prog = {
  facts,
  rules: [...require('../domains/quality').qualityRules()],
  meta: { tags: ['nlp:qualities'] },
};
console.log(JSON.stringify(prog, null, 2));
