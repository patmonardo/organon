export interface Node {
  id: string
  kind: string
  props?: Record<string, unknown>
}

export interface Edge {
  kind: string
  from: string
  to: string
  props?: Record<string, unknown>
}

export interface GraphDoc {
  nodes: Node[]
  edges: Edge[]
}
