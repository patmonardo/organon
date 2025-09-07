import fs from 'node:fs/promises'
import path from 'node:path'
import { DatasetPackageManifest, validateDatasetPackage } from '../../package/dataset-manifest'

function toFileUrl(p: string) { return new URL('file://' + path.resolve(p)).href }
async function loadManifest(spec: string): Promise<DatasetPackageManifest> {
  if (spec.endsWith('.json')) return JSON.parse(await fs.readFile(spec, 'utf8'))
  const mod = await import(toFileUrl(spec))
  return (mod as any).default || (mod as any).manifest
}

async function pack(args: string[]) {
  const spec = args[0]; const extra = args.slice(1)
  if (!spec) { console.log('usage: gdsl dataset pack <manifest.(ts|json)>'); process.exit(1) }
  const mod = await import('../../package/dataset-pack')
  const tgz = await (mod as any).packDataset(spec, extra)
  console.log(tgz)
}

async function validate(args: string[]) {
  const spec = args[0]; const json = args.includes('--json')
  if (!spec) { console.log('usage: gdsl dataset validate <manifest.(ts|json)> [--json]'); process.exit(1) }
  const m = await loadManifest(spec)
  validateDatasetPackage(m)
  if (json) console.log(JSON.stringify({ ok: true, id: m.id, version: m.version }, null, 2))
  else console.log(`[ok] ${m.id}@${m.version}`)
}

async function info(args: string[]) {
  const spec = args[0]; const json = args.includes('--json')
  if (!spec) { console.log('usage: gdsl dataset info <manifest.(ts|json)> [--json]'); process.exit(1) }
  const m = await loadManifest(spec)
  const out = { id: m.id, name: m.name, version: m.version, kind: m.kind, datasetId: m.datasetId, units: m.units.length, assets: m.assets?.length || 0 }
  if (json) console.log(JSON.stringify(out, null, 2))
  else console.log(`${out.id} — ${out.name} (${out.version}) | units=${out.units} assets=${out.assets}`)
}

async function list() {
  const dist = path.resolve(process.cwd(), 'gdsl/dist/pkg')
  const installed = path.resolve(process.cwd(), 'gdsl/packages')
  const print = async (dir: string) => {
    let entries: string[] = []
    try { entries = await fs.readdir(dir) } catch {}
    if (!entries.length) return
    console.log(`# ${path.relative(process.cwd(), dir)}`)
    for (const e of entries.filter(x => x.endsWith('.tgz'))) console.log(`- ${e}`)
    for (const e of entries.filter(x => !x.endsWith('.tgz'))) console.log(`- ${e}/`)
  }
  await print(dist)
  await print(installed)
}

const noun = {
  noun: 'dataset',
  aliases: ['ds'],
  help: 'Manage dataset packages (pack, validate, info, list)',
  verbs: { pack, validate, info, list }
}
export default noun
