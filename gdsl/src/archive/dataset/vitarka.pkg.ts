import { makeDatasetPackageId, DatasetPackageManifest } from '../packaging/dataset-manifest'

const manifest: DatasetPackageManifest = {
  id: makeDatasetPackageId('vitarka'),
  name: 'Vitarka (I.42–I.43)',
  version: '0.1.0',
  kind: 'dataset',
  datasetId: 'reality:logos:datasets:vitarka',
  units: [
    'reality:logos:ys:i.42',
    'reality:logos:ys:i.43'
  ],
  assets: [
    'gdsl/src/datasets/vitarka.ts'
  ],
  meta: { policy: 'curative', lens: 'yoga' }
}

export default manifest
