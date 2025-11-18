import { pathToFileURL } from 'node:url'
import fg from 'fast-glob'
import { writeFile, mkdir } from 'node:fs/promises'
import { resolve, dirname } from 'node:path'

type Unit = { id: string; title?: string; chunks: any[]; hlos: any[] }
type ModuleLike = Record<string, any>

async function main() {
  const files = await fg(['reality/src/logos/**/*.ts'])
  const units: Unit[] = []
  const symbols: string[] = []

  for (const f of files) {
    const mod = (await import(pathToFileURL(f).toString())) as ModuleLike
    for (const [k, v] of Object.entries(mod)) {
      if (k.endsWith('_ONTOLOGY') && v && typeof v === 'object') {
        symbols.push(...Object.keys(v as Record<string, unknown>))
      }
      if (k.endsWith('_UNIT') && v && typeof v === 'object' && 'chunks' in v && 'hlos' in v) {
        units.push(v as Unit)
      }
    }
  }

  const outPath = resolve('gdsl/src/kg/kg.json')
  await mkdir(dirname(outPath), { recursive: true })
  await writeFile(outPath, JSON.stringify({ units, symbols }, null, 2), 'utf8')
  console.log(`KG JSON written: ${outPath} (units: ${units.length}, symbols: ${symbols.length})`)
}

main().catch(err => {
  console.error(err)
  process.exit(1)
})
