import { makeDatasetId, validateDataset, Dataset } from '../registry/canon'
import { YS_I_42_UNIT, YS_I_43_UNIT } from '../ys'

export const DATASET_ID = makeDatasetId('vitarka')

export const VITARKA_DATASET: Dataset = {
  id: DATASET_ID,
  title: 'Vitarka (I.42–I.43) — Light as First Principle',
  scope: 'being-only',
  logosMode: 'prajna',
  synthesis: 'pre-factum',
  faculty: 'buddhi',
  lens: 'yoga',
  units: [YS_I_42_UNIT, YS_I_43_UNIT],
}

validateDataset(VITARKA_DATASET)
