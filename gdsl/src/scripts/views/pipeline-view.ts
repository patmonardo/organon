import fs from 'fs';
import path from 'path';

type Edge = { type: string; from: string; to: string; props?: any };
type Node = { id: string; labels?: string[]; props?: any };
type Artifact = { nodes: Node[]; edges: Edge[] };

function load(file: string): Artifact {
  return JSON.parse(fs.readFileSync(path.resolve(file), 'utf8'));
}

// Collect tags per HLO: key->value from HLO_TAGS, and key->termId for HLO_TAGS_TERM
function collectTags(art: Artifact) {
  const tags = new Map<string, Record<string, string>>();
  for (const e of art.edges) {
    if (e.type === 'HLO_TAGS') {
      const key = e.props?.key;
      const value = e.props?.value;
      if (!key) continue;
      const t = tags.get(e.from) ?? {};
      t[String(key)] = String(value);
      tags.set(e.from, t);
    }
    if (e.type === 'HLO_TAGS_TERM') {
      const key = e.props?.key;
      const termId = String(e.to).startsWith('term:')
        ? String(e.to).slice(5)
        : String(e.to);
      if (!key) continue;
      const t = tags.get(e.from) ?? {};
      t[String(key)] = termId;
      tags.set(e.from, t);
    }
  }
  return tags;
}

function groupBy<T extends string>(
  ids: string[],
  tags: Map<string, Record<string, string>>,
  key: string,
  fallback: T,
) {
  const out = new Map<T, string[]>();
  for (const id of ids) {
    const v = (tags.get(id)?.[key] ?? fallback) as T;
    const arr = out.get(v) ?? [];
    arr.push(id);
    out.set(v, arr);
  }
  return out;
}

function asNumber(x?: string) {
  const n = Number(x);
  return Number.isFinite(n) ? n : NaN;
}

function main() {
  const file = process.argv[2];
  if (!file) {
    console.error(
      'usage: tsx src/scripts/views/pipeline-view.ts dist/datasets/dataset_science-of-logic-being.json',
    );
    process.exit(2);
  }
  const art = load(file);
  const tags = collectTags(art);

  // all HLO ids tagged with a cycle
  const byCycle: Record<string, string[]> = {};
  for (const [hloId, t] of tags) {
    const cyc = t.cycle ?? 'UNSPECIFIED';
    (byCycle[cyc] ||= []).push(hloId);
  }

  const result: any[] = [];
  for (const [cycle, ids] of Object.entries(byCycle)) {
    // split per lens (default hegel if none)
    const byLens = groupBy(ids, tags, 'lens', 'hegel');
    for (const [lens, hlos] of byLens) {
      const rows = hlos
        .map((id) => {
          const t = tags.get(id) ?? {};
          return {
            hloId: id,
            lens,
            cycle,
            plane: t.plane ?? 'dyadic',
            phase: t.phase ?? 'unknown',
            role: t.role ?? 'unknown',
            order: asNumber(t.order),
            faculty: t.faculty ?? '',
          };
        })
        .sort((a, b) => a.order - b.order);
      result.push({ cycle, lens, steps: rows });
    }
  }

  console.log(JSON.stringify(result, null, 2));
}

main();
