import BaseLens from './base.lens';
import { makeLensId, type Lens } from '../schema/lens';
import type { GraphArtifact } from '../schema/projection';
import type { NodeRow, EdgeRow } from '../schema/projection';

/** DatasetLens: expose dataset structure (chunks / HLOs / nodes) as a Lens */
export class DatasetLens extends BaseLens {
  private constructor(data: Lens) {
    super(data);
  }

  static create(id = makeLensId('dataset')) {
    return new DatasetLens({
      id,
      title: 'Dataset Lens (chunks & HLOs)',
      terms: [],
      edges: [],
    });
  }

  /** Build a lens from a GraphArtifact by including nodes that look like dataset entities */
  static fromArtifact(id: string, artifact: GraphArtifact) {
    const terms = (artifact.nodes ?? []).map((n: NodeRow) => ({
      id: String(n.id),
      label: String((n.props as any)?.label ?? n.id),
      aliases: (n.props as any)?.aliases,
      desc: (n.props as any)?.desc,
      tags: (n.props as any)?.tags,
    }));

    const termIds = new Set(terms.map((t) => t.id));
    const edges = (artifact.edges ?? [])
      .filter(
        (e: EdgeRow) =>
          termIds.has(String(e.from)) && termIds.has(String(e.to)),
      )
      .map((e: EdgeRow) => ({
        type: e.type,
        from: String(e.from),
        to: String(e.to),
        props: e.props,
      }));

    return new DatasetLens({
      id,
      title: `Dataset Lens — ${artifact.dataset}`,
      terms,
      edges,
    });
  }

  // --- new: pipeline helper (pure, no IO) ---
  static computePipelineRows(artifact: GraphArtifact) {
    type TagMap = Map<string, Record<string, string>>;
    const collectTags = (art: GraphArtifact): TagMap => {
      const tags: TagMap = new Map();
      for (const e of art.edges ?? []) {
        if (e.type === 'HLO_TAGS' || e.type === 'HLO_TAGS_TERM') {
          const key = String(e.props?.key ?? '');
          if (!key) continue;
          const from = String(e.from);
          const value =
            e.type === 'HLO_TAGS_TERM'
              ? String(e.to).startsWith('term:')
                ? String(e.to).slice(5)
                : String(e.to)
              : String(e.props?.value ?? '');
          const entry = tags.get(from) ?? {};
          entry[key] = value;
          tags.set(from, entry);
        }
      }
      return tags;
    };

    const groupBy = <T extends string>(
      ids: string[],
      tags: TagMap,
      key: string,
      fallback: T,
    ) => {
      const out = new Map<T, string[]>();
      for (const id of ids) {
        const v = (tags.get(id)?.[key] ?? fallback) as T;
        const arr = out.get(v) ?? [];
        arr.push(id);
        out.set(v, arr);
      }
      return out;
    };

    const toNumber = (x?: string) => {
      const n = Number(x);
      return Number.isFinite(n) ? n : NaN;
    };

    const tags = collectTags(artifact);
    const hloIds = Array.from(tags.keys());
    const byCycle = new Map<string, string[]>();
    for (const hloId of hloIds) {
      const cyc = tags.get(hloId)?.cycle ?? 'UNSPECIFIED';
      let arr = byCycle.get(cyc);
      if (!arr) {
        arr = [];
        byCycle.set(cyc, arr);
      }
      arr.push(hloId);
    }

    const result: Array<{
      cycle: string;
      lens: string;
      steps: Array<Record<string, unknown>>;
    }> = [];
    for (const [cycle, ids] of byCycle) {
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
              order: toNumber(t.order),
              faculty: t.faculty ?? '',
            };
          })
          .sort((a, b) =>
            Number.isFinite(a.order) && Number.isFinite(b.order)
              ? a.order - b.order
              : 0,
          );
        result.push({ cycle, lens, steps: rows });
      }
    }
    return result;
  }
}
