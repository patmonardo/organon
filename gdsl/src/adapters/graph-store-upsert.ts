import type { GraphArtifact } from '../schema/projection'
import type { GraphStore, NodeKey } from './graph-store'
export async function upsertArtifactToStore(store: GraphStore, artifact: GraphArtifact) {
  for (const n of artifact.nodes) {
    const key: NodeKey = { label: n.labels?.[0] ?? 'Node', key: 'id', value: (n as any).id }
    await store.upsertNode({ labels: n.labels ?? ['Node'], key, props: (n as any).props })
  }
  for (const e of artifact.edges) {
    const fromKey: NodeKey = { label: 'Node', key: 'id', value: e.from }
    const toKey: NodeKey = { label: 'Node', key: 'id', value: e.to }
    await store.upsertEdge({ type: e.type, from: fromKey, to: toKey, props: e.props })
  }
}
