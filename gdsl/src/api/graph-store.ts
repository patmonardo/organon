export type NodeKey = { label: string; key: string; value: string | number }

export interface GraphStore {
  getNode?(key: NodeKey): Promise<{ id: string; labels: string[]; props: Record<string, unknown> } | null>
  findNodes?(label: string, where?: unknown, page?: unknown): Promise<any[]>
  upsertNode(node: { labels: string[]; key?: NodeKey; props?: Record<string, unknown> }): Promise<string>
  upsertEdge(edge: { type: string; from: NodeKey; to: NodeKey; props?: Record<string, unknown> }): Promise<void>
  beginTx?(): Promise<any>
  close?(): Promise<void>
}

export class MemoryGraphStore implements GraphStore {
  private nodes = new Map<string, { id: string; labels: string[]; props?: Record<string, unknown> }>()
  private edges: { type: string; from: string; to: string; props?: Record<string, unknown> }[] = []
  private seq = 0

  async upsertNode(node: { labels: string[]; key?: NodeKey; props?: Record<string, unknown> }) {
    const id = node.key ? `${node.key.label}|${node.key.key}|${String(node.key.value)}` : `__n:${++this.seq}`
    this.nodes.set(id, { id, labels: node.labels, props: node.props })
    return id
  }

  async upsertEdge(edge: { type: string; from: NodeKey; to: NodeKey; props?: Record<string, unknown> }) {
    const from = `${edge.from.label}|${edge.from.key}|${String(edge.from.value)}`
    const to = `${edge.to.label}|${edge.to.key}|${String(edge.to.value)}`
    this.edges.push({ type: edge.type, from, to, props: edge.props })
  }

  dump() {
    return { nodes: Array.from(this.nodes.values()), edges: this.edges.slice() }
  }
}
