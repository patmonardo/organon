export type NodeKey = { label: string; key: string; value: string }
export interface GraphStore {
  upsertNode(node: { labels?: string[]; key: NodeKey; props?: Record<string, unknown> }): Promise<void>
  upsertEdge(edge: { type: string; from: NodeKey; to: NodeKey; props?: Record<string, unknown> }): Promise<void>
  dump?(): unknown
}
export class MemoryGraphStore implements GraphStore {
  private nodes = new Map<string, any>()
  private edges: any[] = []
  async upsertNode(n: { labels?: string[]; key: NodeKey; props?: Record<string, unknown> }) {
    const k = `${n.key.label}:${n.key.value}`; this.nodes.set(k, { labels: n.labels ?? ['Node'], props: n.props })
  }
  async upsertEdge(e: { type: string; from: NodeKey; to: NodeKey; props?: Record<string, unknown> }) {
    this.edges.push(e)
  }
  dump() { return { nodes: Array.from(this.nodes.entries()), edges: this.edges } }
}
