import { pathToFileURL } from 'node:url'
import fg from 'fast-glob'
import pc from 'picocolors'

type Hlo = { chunkId: string; clauses: string[]; id: string }
type Chunk = { id: string; title: string }
type Unit = { id: string; chunks: Chunk[]; hlos: Hlo[] }
type ModuleLike = Record<string, any>

const allowedOps = ['define', 'assert', 'conclude', 'note', 'warn', 'todo', 'link']

function extractSymbols(clause: string): string[] {
  const m = clause.match(/^[a-z]+?\((.+)\)$/i)
  if (!m) return []
  const inner = m[1]
  if (clause.startsWith('link(')) {
    return inner.split(/[→,\-–>]/).map(s => s.trim()).filter(Boolean)
  }
  return [inner.trim()]
}

async function main() {
  // allow override via CLI or env
  const glob = process.argv[2] || process.env.KG_GLOB || 'reality/src/logos/**/*.ts'
  const files = await fg([glob])
  let errors = 0
  let warnings = 0

  for (const f of files) {
    const mod = (await import(pathToFileURL(f).toString())) as ModuleLike
    const unitExports = Object.values(mod).filter(
      (v: any) => v && typeof v === 'object' && 'id' in v && 'chunks' in v && 'hlos' in v
    )
    const ontologies = Object.entries(mod)
      .filter(([k, v]) => k.endsWith('_ONTOLOGY') && v && typeof v === 'object')
      .map(([, v]) => v as Record<string, unknown>)
    const ontologyKeys = new Set<string>(ontologies.flatMap(o => Object.keys(o)))

    for (const unit of unitExports as Unit[]) {
      const chunkIds = new Set(unit.chunks.map(c => c.id))
      for (const hlo of unit.hlos) {
        if (!chunkIds.has(hlo.chunkId)) {
          console.error(pc.red(`x ${unit.id}: HLO ${hlo.id} missing chunkId: ${hlo.chunkId}`))
          errors++
        }
        for (const clause of hlo.clauses) {
          const op = clause.slice(0, clause.indexOf('('))
          if (!allowedOps.includes(op)) {
            console.error(pc.red(`x ${unit.id}: ${hlo.id} unknown op: ${op} in "${clause}"`))
            errors++
          }
          for (const sym of extractSymbols(clause)) {
            if (!ontologyKeys.has(sym) && op !== 'link') {
              console.warn(pc.yellow(`! ${unit.id}: ${hlo.id} nonlocal symbol "${sym}"`))
              warnings++
            }
          }
        }
      }
      const seen = new Set<string>()
      for (const h of unit.hlos) {
        if (seen.has(h.id)) {
          console.error(pc.red(`x ${unit.id}: duplicate HLO id: ${h.id}`))
          errors++
        }
        seen.add(h.id)
      }
    }
  }

  const hdr = errors ? pc.red : pc.green
  console.log(hdr(`Done. ${errors} error(s), ${warnings} warning(s).`))
  process.exit(errors ? 1 : 0)
}

main().catch(e => {
  console.error(pc.red(e.stack || e))
  process.exit(1)
})
