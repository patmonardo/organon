import fs from 'node:fs/promises'
import path from 'node:path'
import { spawn } from 'node:child_process'
import { DatasetPackageManifest, validateDatasetPackage } from './dataset-manifest'

function toFileUrl(p: string) { return new URL('file://' + path.resolve(p)).href }

async function loadManifest(spec: string): Promise<DatasetPackageManifest> {
  if (spec.endsWith('.json')) return JSON.parse(await fs.readFile(spec, 'utf8'))
  const mod = await import(toFileUrl(spec))
  return (mod as any).default || (mod as any).manifest
}

async function cp(src: string, dst: string) {
  await fs.mkdir(path.dirname(dst), { recursive: true })
  await fs.copyFile(src, dst)
}

async function tarDir(outTgz: string, cwd: string) {
  return new Promise<void>((resolve, reject) => {
    const p = spawn('tar', ['-czf', outTgz, '.'], { cwd, stdio: 'inherit' })
    p.on('exit', code => code === 0 ? resolve() : reject(new Error(`tar exit ${code}`)))
  })
}

export async function packDataset(spec: string, _extPaths: string[] = []) {
  const m = await loadManifest(spec)
  validateDatasetPackage(m)
  const outRoot = path.resolve(process.cwd(), 'gdsl/dist/pkg')
  const dirName = `${m.id.replace(/[:]/g, '.')}-${m.version}`
  const stageDir = path.join(outRoot, dirName)

  await fs.rm(stageDir, { recursive: true, force: true })
  await fs.mkdir(stageDir, { recursive: true })
  await fs.writeFile(path.join(stageDir, 'manifest.json'), JSON.stringify(m, null, 2), 'utf8')
  for (const a of m.assets || []) {
    const abs = path.resolve(process.cwd(), a)
    const rel = path.relative(process.cwd(), abs)
    await cp(abs, path.join(stageDir, rel))
  }
  const tgz = path.join(outRoot, `${dirName}.tgz`)
  await tarDir(tgz, stageDir)
  console.log(`[pack:dataset] ${m.id}@${m.version} -> ${path.relative(process.cwd(), tgz)}`)
  return tgz
}

// CLI entry (optional direct invocation)
async function main() {
  const spec = process.argv[2]
  if (!spec) { console.log('usage: gdsl-pack-dataset <manifest.(ts|json)>'); process.exit(1) }
  await packDataset(spec)
}
if (import.meta.url === toFileUrl(path.resolve(__filename))) {
  main().catch(e => { console.error(e); process.exit(1) })
}
