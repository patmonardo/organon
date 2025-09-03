/* eslint-disable no-console */
import * as fs from 'node:fs'
import * as path from 'node:path'
import type { LogicalOperation } from '../src/logic/index'
import { analyzePredicates } from '../src/logic/analysis/predicate-report' // was ../src/analysis/...

// Import registries (extend this list as needed)
import { LOGICAL_OPERATIONS as HLO_BFS } from '../logic/being-for-self/being-for-self'
import { LOGICAL_OPERATIONS as HLO_OM } from '../logic/being-for-self/one-many'
import { LOGICAL_OPERATIONS as HLO_OM1 } from '../logic/being-for-self/one-many1'
import { LOGICAL_OPERATIONS as HLO_OM2 } from '../logic/being-for-self/one-many2'
import { LOGICAL_OPERATIONS as HLO_RA } from '../logic/being-for-self/attraction'
import { LOGICAL_OPERATIONS as HLO_RA1 } from '../logic/being-for-self/attraction1'
import { LOGICAL_OPERATIONS as HLO_RA2 } from '../logic/being-for-self/attraction2'

function collectHLOs(): LogicalOperation[] {
  return [
    ...HLO_BFS,
    ...HLO_OM,
    ...HLO_OM1,
    ...HLO_OM2,
    ...HLO_RA,
    ...HLO_RA1,
    ...HLO_RA2
  ]
}

async function main() {
  const hlos = collectHLOs()
  const predReport = analyzePredicates(hlos)

  console.log('[analyze] HLOs:', predReport.totals.hlos, 'clauses:', predReport.totals.clauses)
  console.log('[analyze] asserts:', predReport.totals.asserts, 'tags:', predReport.totals.tags, 'unknown:', predReport.totals.unknown)

  const top = Object.entries(predReport.predicates)
    .sort((a, b) => b[1].count - a[1].count)
    .slice(0, 20)

  console.log('[analyze] top predicates:')
  for (const [name, s] of top) console.log(`  - ${name}: ${s.count}`)

  const outDir = path.resolve(__dirname, '../../reports')
  fs.mkdirSync(outDir, { recursive: true })
  fs.writeFileSync(path.join(outDir, 'predicate-usage.json'), JSON.stringify(predReport, null, 2), 'utf8')
  console.log('[analyze] wrote reports/predicate-usage.json')
}

main().catch(err => {
  console.error(err)
  process.exit(1)
})
