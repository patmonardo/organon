/* eslint-disable no-console */
import * as fs from 'node:fs'
import * as path from 'node:path'
import type { LogicalOperation } from '../src/logic/index'
import { analyzePredicates } from '../src/logic/analysis/predicate-report'
import { parseClause } from '../src/logic/analysis/clause-parser'

// TODO: centralize these imports into ALL_CHUNKS/ALL_HLOS
import { CANONICAL_CHUNKS as CH_RA2, LOGICAL_OPERATIONS as HLO_RA2 } from '../logic/being-for-self/attraction2'
// ...import other registries...
const CHUNKS = [...CH_RA2] // extend with others
const HLOS: LogicalOperation[] = [...HLO_RA2] // extend with others

function esc(x: string) { return x.replace(/\\/g,'\\\\').replace(/"/g,'\\"') }

function buildCypher() {
  const lines: string[] = []
  // Chunks
  lines.push('UNWIND $chunks AS c')
  lines.push('MERGE (ch:Chunk {id: c.id})')
  lines.push('SET ch.title = c.title, ch.concise = c.concise')
  lines.push('WITH 1 as _;')
  // HLOs
  lines.push('UNWIND $hlos AS h')
  lines.push('MERGE (o:HLO {id: h.id})')
  lines.push('SET o.label = h.label')
  lines.push('WITH h, o')
  lines.push('MATCH (ch:Chunk {id: h.chunkId})')
  lines.push('MERGE (ch)-[:HAS_HLO]->(o)')
  lines.push('WITH h, o')
  lines.push('UNWIND h.clauses AS clause')
  lines.push('WITH o, clause WHERE clause STARTS WITH "assert(" OR clause STARTS WITH "tag("')
  lines.push('MERGE (cl:Clause {raw: clause})')
  lines.push('MERGE (o)-[:HAS_CLAUSE]->(cl)')
  lines.push('RETURN count(*) AS linked;')
  return lines.join('\n')
}

async function main() {
  const cypher = buildCypher()
  const payload = {
    chunks: CHUNKS.map(c => ({ id: c.id, title: c.title, summary: (c as any).concise ?? '' })),
    hlos: HLOS.map(h => ({ id: h.id, label: h.label, chunkId: h.chunkId, clauses: h.clauses ?? [] }))
  }
  const outDir = path.resolve(__dirname, '../../dist')
  fs.mkdirSync(outDir, { recursive: true })
  fs.writeFileSync(path.join(outDir, 'logic.cypher'), cypher, 'utf8')
  fs.writeFileSync(path.join(outDir, 'logic.params.json'), JSON.stringify(payload, null, 2), 'utf8')
  console.log('[export] wrote dist/logic.cypher and dist/logic.params.json')
  // Optional: quick stats
  const stats = analyzePredicates(HLOS)
  fs.writeFileSync(path.join(outDir, 'predicate-usage.json'), JSON.stringify(stats, null, 2), 'utf8')
}
main().catch(e => { console.error(e); process.exit(1) })
