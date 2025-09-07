import fs from 'node:fs/promises';
import path from 'node:path';
import {
  DatasetPackageManifest,
  validateDatasetPackage,
} from '../../package/dataset-manifest';

// lazy import to keep single source of truth for packing
async function packDataset(manifestSpec: string, extPaths: string[]) {
  const mod = await import('../../package/pack-dataset');
  return (mod as any).packDataset
    ? (mod as any).packDataset(manifestSpec, extPaths)
    : (mod as any).default?.(manifestSpec, extPaths);
}

function toFileUrl(p: string) {
  return new URL('file://' + path.resolve(p)).href;
}

async function loadManifest(spec: string): Promise<DatasetPackageManifest> {
  if (spec.endsWith('.json'))
    return JSON.parse(await fs.readFile(spec, 'utf8'));
  const mod = await import(toFileUrl(spec));
  return (mod as any).default || (mod as any).manifest;
}

async function cmdPack(args: string[]) {
  const spec = args[0];
  const ext = args.filter((a) => a.startsWith('--ext=')).map((a) => a.slice(6));
  if (!spec) {
    console.log(
      'usage: gdsl dataset pack <manifest.(ts|json)> [--ext=validator.ts]...',
    );
    process.exit(1);
  }
  const tgzPath = await packDataset(spec, ext);
  console.log(tgzPath);
}

async function cmdValidate(args: string[]) {
  const spec = args[0];
  const json = args.includes('--json');
  if (!spec) {
    console.log('usage: gdsl dataset validate <manifest.(ts|json)> [--json]');
    process.exit(1);
  }
  const m = await loadManifest(spec);
  validateDatasetPackage(m);
  if (json)
    console.log(
      JSON.stringify({ ok: true, id: m.id, version: m.version }, null, 2),
    );
  else console.log(`[ok] ${m.id}@${m.version}`);
}

async function cmdInfo(args: string[]) {
  const spec = args[0];
  const json = args.includes('--json');
  if (!spec) {
    console.log('usage: gdsl dataset info <manifest.(ts|json)> [--json]');
    process.exit(1);
  }
  const m = await loadManifest(spec);
  const out = {
    id: m.id,
    name: m.name,
    version: m.version,
    kind: m.kind,
    datasetId: m.datasetId,
    units: m.units.length,
    assets: m.assets?.length || 0,
  };
  if (json) console.log(JSON.stringify(out, null, 2));
  else
    console.log(
      `${out.id} — ${out.name} (${out.version}) | units=${out.units} assets=${out.assets}`,
    );
}

async function cmdList() {
  const dist = path.resolve(process.cwd(), 'gdsl/dist/pkg');
  const installed = path.resolve(process.cwd(), 'gdsl/packages');
  for (const dir of [dist, installed]) {
    let entries: string[] = [];
    try {
      entries = await fs.readdir(dir);
    } catch {}
    if (!entries.length) continue;
    console.log(`# ${path.relative(process.cwd(), dir)}`);
    for (const e of entries.filter((x) => x.endsWith('.tgz')))
      console.log(`- ${e}`);
    for (const e of entries.filter((x) => !x.endsWith('.tgz')))
      console.log(`- ${e}/`);
  }
}

export async function run(verb: string, args: string[]) {
  switch (verb) {
    case 'pack':
      return cmdPack(args);
    case 'validate':
      return cmdValidate(args);
    case 'info':
      return cmdInfo(args);
    case 'list':
      return cmdList();
    default:
      console.error(`unknown dataset verb: ${verb}`);
      process.exit(1);
  }
}
