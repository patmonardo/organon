import type { World, WorldEdge } from '../../schema/world';

export type WorldIndexes = {
  byNode: Map<string, { out: WorldEdge[]; in: WorldEdge[]; undirected: WorldEdge[] }>;
  byKind: Map<string, WorldEdge[]>;
  degree: Map<string, number>;
};

export function buildWorldIndexes(world: World): WorldIndexes {
  const byNode: WorldIndexes['byNode'] = new Map();
  const byKind: WorldIndexes['byKind'] = new Map();
  const degree: WorldIndexes['degree'] = new Map();

  const edges: WorldEdge[] = world.shape.relations ?? [];
  const nodes = world.shape.things ?? [];

  for (const n of nodes) {
    byNode.set(n.id, { out: [], in: [], undirected: [] });
    degree.set(n.id, 0);
  }

  for (const e of edges) {
    const bucket = byKind.get(e.kind) ?? [];
    bucket.push(e);
    byKind.set(e.kind, bucket);

    const s = byNode.get(e.source.id) ?? { out: [], in: [], undirected: [] };
    const t = byNode.get(e.target.id) ?? { out: [], in: [], undirected: [] };

    if (e.direction === 'bidirectional') {
      s.undirected.push(e);
      t.undirected.push(e);
      degree.set(e.source.id, (degree.get(e.source.id) ?? 0) + 1);
      degree.set(e.target.id, (degree.get(e.target.id) ?? 0) + 1);
    } else {
      s.out.push(e);
      t.in.push(e);
      degree.set(e.source.id, (degree.get(e.source.id) ?? 0) + 1);
      degree.set(e.target.id, (degree.get(e.target.id) ?? 0) + 1);
    }

    byNode.set(e.source.id, s);
    byNode.set(e.target.id, t);
  }

  return { byNode, byKind, degree };
}

export function neighbors(indexes: WorldIndexes, id: string): string[] {
  const b = indexes.byNode.get(id);
  if (!b) return [];
  const out = b.out.map((e) => e.target.id);
  const und = b.undirected.map((e) => (e.source.id === id ? e.target.id : e.source.id));
  const incoming = b.in.map((e) => e.source.id);
  return Array.from(new Set([...out, ...und, ...incoming])).sort();
}
