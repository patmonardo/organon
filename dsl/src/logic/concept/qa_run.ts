import { integrityReport } from './qa'

const report = integrityReport()
console.log(JSON.stringify(report, null, 2))
