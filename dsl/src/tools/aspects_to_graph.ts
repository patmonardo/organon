import fs from 'node:fs'
import type { FormsDoc } from '../schema/forms'
import type { GraphDoc, Node, Edge } from '../schema/graph'

function N(id: string, kind: string, props?: Record<string, unknown>): Node {
  return { id, kind, props }
}
function E(kind: string, from: string, to: string, props?: Record<string, unknown>): Edge {
  return { kind, from, to, props }
}

export function toGraph(doc: FormsDoc): GraphDoc {
  const nodes: Node[] = []
  const edges: Edge[] = []

  const have = new Set<string>()
  const pushN = (n: Node) => { if (!have.has(n.id)) { nodes.push(n); have.add(n.id) } }

  for (const s of doc.shapes) pushN(N(s.id, 'Shape', { name: s.name, ...s.meta }))
  for (const c of doc.contexts) pushN(N(c.id, 'Context', { name: c.name, ...c.meta }))
  for (const m of doc.morphs) {
    pushN(N(m.id, 'Morph', { name: m.name, ...m.meta }))
    edges.push(E('USES_SHAPE', m.id, m.shapeId))
    edges.push(E('USES_CONTEXT', m.id, m.contextId))
  }
  for (const e of doc.entities) {
    pushN(N(e.id, 'Entity', { shapeId: e.shapeId, clauses: e.clauses, ...e.meta }))
    edges.push(E('AXIOMATIZED_BY', e.id, e.shapeId))
  }
  for (const p of doc.properties) {
    pushN(N(p.id, 'Property', { contextId: p.contextId, law: p.law, value: p.value, ...p.meta }))
    edges.push(E('EVALUATED_IN', p.id, p.contextId))
  }
  for (const a of doc.aspects) {
    pushN(N(a.id, 'Aspect', { stage: a.stage, ...a.meta }))
    edges.push(E('DRIVEN_BY', a.id, a.morphId))
    edges.push(E('OF_ENTITY', a.id, a.entityId))
    edges.push(E('REALIZES_PROPERTY', a.id, a.propertyId))
    for (const j of a.judgments ?? []) {
      const jid = `${a.id}#${(j.source ?? j.type)}#${j.subject}->${(j.relation ?? j.predicate)}->${j.object}`
      pushN(N(jid, 'Judgment', j as any))
      pushN(N(j.subject, 'Concept', { name: j.subject }))
      pushN(N(j.object, 'Concept', { name: j.object }))
      edges.push(E('JUDGMENT_OF', jid, a.id))
      edges.push(E('SUBJECT', jid, j.subject))
      edges.push(E('OBJECT', jid, j.object))
      if (j.relation) edges.push(E(j.relation, j.subject, j.object))
      if (j.predicate && j.type === 'Affirmative') edges.push(E('IS', j.subject, j.object))
      if (j.predicate && j.type === 'Negative') edges.push(E('NOT_IS', j.subject, j.object))
    }
  }

  return { nodes, edges }
}

function main() {
  const path = process.argv[2]
  if (!path) {
    console.error('Usage: tsx src/tools/aspects_to_graph.ts <forms.json>')
    process.exit(1)
  }
  const forms: FormsDoc = JSON.parse(fs.readFileSync(path, 'utf8'))
  const graph = toGraph(forms)
  console.log(JSON.stringify(graph))
}

// Run only when invoked directly (best-effort ESM guard)
try {
  const isMain = process.argv[1] && new URL(import.meta.url).pathname === process.argv[1]
  if (isMain) main()
} catch {
  // ignore if import.meta.url is not available
}
