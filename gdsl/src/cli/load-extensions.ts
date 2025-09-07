export type GdslApi = {
  registerMode: typeof import('../registry/modes').registerMode
  registerNoun: (def: { noun: string; verbs: Record<string, (args: string[]) => any> }) => void
}

export async function loadExtensions(api: GdslApi, specs: string[] = []) {
  for (const spec of specs) {
    const mod = await import(spec)
    const activate = (mod.default?.activate ?? mod.activate) as ((api: GdslApi) => any) | undefined
    if (activate) await activate(api)
  }
}
