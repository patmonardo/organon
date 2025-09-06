export interface MaximLaw {
  id: string
  name: string
  statement: string
  resolves: string[]
  notes?: string
}

export const LAW_OF_MAXIMS: MaximLaw = {
  id: 'law-of-maxims',
  name: 'Law of Maxims',
  statement:
    'Resolve standpoint conflicts by specifying when intuition arises only as appearance (not self-grounded), unifying idealism and realism.',
  resolves: ['idealism-vs-realism'],
  notes:
    'Realism preferred over one-sided idealism; synthesis validates appearance-as-appearance and derives error from necessary absence.',
}
