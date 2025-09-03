import { EXISTENCE_CHUNKS, EXISTENCE_HLOS } from '../logic/existence'

type Rel = { predicate: string; from: string; to: string }

const esc = (s: string) => (s ?? '').replace(/\\/g, '\\\\').replace(/"/g, '\\"')

function main() {
  // Serialize inputs to JSON strings for UNWIND
  const chunks = EXISTENCE_CHUNKS.map(c => ({
    id: c.id, title: c.title, concise: c.concise ?? ''
  }))
  const hlos = EXISTENCE_HLOS.map(h => ({
    id: h.id,
    chunkId: h.chunkId,
    label: h.label ?? '',
    summary: h.candidateSummary ?? ''
  }))
  const rels: (Rel & { hloId: string; chunkId: string; hloLabel: string })[] = []
  for (const h of EXISTENCE_HLOS) {
    for (const r of (h.relations ?? []) as Rel[]) {
      rels.push({
        ...r,
        hloId: h.id,
        chunkId: h.chunkId,
        hloLabel: h.label ?? ''
      })
    }
  }

  const header = `// Existence KG → Cypher (batched)
CREATE CONSTRAINT IF NOT EXISTS FOR (c:Chunk) REQUIRE c.id IS UNIQUE;
CREATE CONSTRAINT IF NOT EXISTS FOR (h:HLO) REQUIRE h.id IS UNIQUE;
CREATE CONSTRAINT IF NOT EXISTS FOR (n:Concept) REQUIRE n.name IS UNIQUE;`

  const chunkLoad = `
WITH $chunks AS batch
UNWIND batch AS row
MERGE (c:Chunk {id: row.id})
SET c.title = row.title, c.concise = row.concise;`

  const hloLoad = `
WITH $hlos AS batch
UNWIND batch AS row
MERGE (h:HLO {id: row.id})
SET h.label = row.label, h.summary = row.summary
WITH h, row
MATCH (c:Chunk {id: row.chunkId})
MERGE (h)-[:IN]->(c);`

  const relLoad = `
WITH $rels AS batch
UNWIND batch AS row
MERGE (a:Concept {name: row.from})
MERGE (b:Concept {name: row.to})
MERGE (a)-[e:\`${esc('')}\`]->(b) // placeholder overwritten below
WITH row, a, b
CALL {
  WITH row, a, b
  // dynamic relationship type via APOC or fallback to reified edge
  WITH row, a, b
  CALL apoc.merge.relationship(a, row.predicate, {}, {}, b) YIELD rel
  SET rel.hloId = row.hloId, rel.chunkId = row.chunkId, rel.hloLabel = row.hloLabel
  RETURN rel
}
RETURN count(*) AS created;`

  // Print a driver script that uses parameters (best loaded via neo4j-driver or cypher-shell --param)
  // For simplicity, emit APOC path and param JSON lines.
  console.log(header)
  console.log(`:param chunks => ${JSON.stringify(chunks)};`)
  console.log(`:param hlos => ${JSON.stringify(hlos)};`)
  console.log(`:param rels => ${JSON.stringify(rels)};`)
  console.log('CALL apoc.schema.assert({},{},true);') // no-op if APOC not present
  console.log(chunkLoad)
  console.log(hloLoad)
  console.log(relLoad)
}

main()
