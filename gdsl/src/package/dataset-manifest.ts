import { NS } from '../registry/canon'

export type PackageKind = 'dataset'

export interface DatasetPackageManifest {
  id: string
  name: string
  version: string
  kind: PackageKind
  datasetId: string
  units: string[]
  assets?: string[]
  meta?: Record<string, any>
  // Extensions keyed by namespace (e.g., "@organon/reality")
  ext?: Record<string, any>
}

export const PKG_NS = `${NS}:pkg`
export function makeDatasetPackageId(slug: string) {
  return `${PKG_NS}:dataset:${slug}`
}

const SEMVER_RX = /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/

export function validateDatasetPackage(m: DatasetPackageManifest) {
  if (!m.id?.startsWith(`${PKG_NS}:dataset:`)) throw new Error('pkg: bad dataset pkg id namespace')
  if (!m.name) throw new Error('pkg: name required')
  if (!SEMVER_RX.test(m.version || '')) throw new Error(`pkg: bad semver ${m.version}`)
  if (m.kind !== 'dataset') throw new Error('pkg: kind must be "dataset"')
  if (!m.datasetId?.startsWith(`${NS}:datasets:`)) throw new Error('pkg: datasetId must be reality:logos:datasets:{name}')
  if (!Array.isArray(m.units) || m.units.length === 0) throw new Error('pkg: units[] required')
  for (const u of m.units) if (!u.startsWith(NS + ':')) throw new Error(`pkg: unit id not in NS: ${u}`)
  return true
}
