import type { z } from 'zod'

export type DatasetMode<T = unknown> = {
  domain: string                        // e.g., 'logic' | 'model' | 'task'
  datasetSchema: z.ZodType<T>           // replaces/extends payload in DatasetCore
  // Optional extras a mode can provide:
  projections?: Record<string, (artifact: unknown) => unknown>
  verbs?: Record<string, (args: string[]) => Promise<void> | void>
  activateOrder?: number                // load order hint
}

const modes = new Map<string, DatasetMode<any>>()

export function registerMode<T>(mode: DatasetMode<T>) {
  modes.set(mode.domain, mode as DatasetMode<any>)
}
export function getMode(domain: string) {
  return modes.get(domain)
}
export function listModes() {
  return Array.from(modes.keys())
}
