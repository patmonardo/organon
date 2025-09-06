import type { Chunk, LogicalOperation } from '../essence'

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'being-1-pure-indeterminate-immediate',
    title: 'Pure being: indeterminate immediacy, empty; equal only to itself; in fact nothing',
    text: `A. BEING

Being, pure being, without further determination.
In its indeterminate immediacy it is equal only to itself
and also not unequal with respect to another;
it has no difference within it, nor any outwardly.
If any determination or content were posited in it as distinct,
or if it were posited by this determination or content
as distinct from an other,
it would thereby fail to hold fast to its purity.
It is pure indeterminateness and emptiness.
There is nothing to be intuited in it,
if one can speak here of intuiting;
or, it is only this pure empty intuiting itself.
Just as little is anything to be thought in it,
or, it is equally only this empty thinking.
Being, the indeterminate immediate is in fact nothing,
and neither more nor less than nothing.`,
    summary:
      'Pure being = indeterminate immediacy, without further determination; equal only to itself, no inner or outer difference; any determination destroys purity. It is pure indeterminateness and emptiness; only empty intuiting/empty thinking. The indeterminate immediate is in fact nothing.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'being-op-1-pure-without-determination',
    chunkId: 'being-1-pure-indeterminate-immediate',
    label: 'Pure being: without further determination; indeterminate immediacy',
    digest:
      'Marks Being as pure, without further determination; indeterminate immediate.',
    clauses: [
      'tag(Being,"pure")',
      'tag(Being,"without-further-determination")',
      'tag(Being,"indeterminate-immediacy")'
    ],
    predicates: [{ name: 'DefinesPureBeing', args: [] }],
    relations: [
      { predicate: 'hasMode', from: 'Being', to: 'IndeterminateImmediate' }
    ]
  },
  {
    id: 'being-op-2-self-equality-no-unequal',
    chunkId: 'being-1-pure-indeterminate-immediate',
    label: 'Equal only to itself; not unequal to another',
    digest:
      'Being equals only itself; not unequal with respect to another.',
    clauses: [
      'assert(equalOnlyToItself(Being))',
      'assert(not(unequalToAnother(Being)))'
    ],
    predicates: [{ name: 'SelfEquality', args: [] }],
    relations: []
  },
  {
    id: 'being-op-3-no-difference-in-or-out',
    chunkId: 'being-1-pure-indeterminate-immediate',
    label: 'No inner or outer difference',
    digest:
      'Being has no internal difference and none outwardly.',
    clauses: [
      'annotate(Being,{noDifference:"internal|external"})',
      'tag(Being,"no-difference")'
    ],
    predicates: [{ name: 'NoDifference', args: [] }],
    relations: [
      { predicate: 'lacks', from: 'Being', to: 'Difference' }
    ]
  },
  {
    id: 'being-op-4-determination-destroys-purity',
    chunkId: 'being-1-pure-indeterminate-immediate',
    label: 'Any determination destroys purity',
    digest:
      'If any determination/content is posited as distinct, Being loses purity.',
    clauses: [
      'tag(Determination,"excludes-pure-being")',
      'annotate(Being,{constraint:"no-determination"})'
    ],
    predicates: [{ name: 'PurityConstraint', args: [] }],
    relations: [
      { predicate: 'excludes', from: 'PureBeing', to: 'Determination' }
    ]
  },
  {
    id: 'being-op-5-empty-intuiting-thinking',
    chunkId: 'being-1-pure-indeterminate-immediate',
    label: 'Empty intuiting and empty thinking',
    digest:
      'Nothing to intuit or think in pure being; only empty intuiting/empty thinking.',
    clauses: [
      'tag(Intuiting,"empty")',
      'tag(Thinking,"empty")'
    ],
    predicates: [{ name: 'EmptyIntuitionThinking', args: [] }],
    relations: []
  },
  {
    id: 'being-op-6-being-equals-nothing',
    chunkId: 'being-1-pure-indeterminate-immediate',
    label: 'Indeterminate immediate is in fact nothing',
    digest:
      'Pure being (indeterminate immediate) is in fact nothing; neither more nor less than nothing.',
    clauses: [
      'assert(equalsTruth(Being, Nothing))',
      'tag(Identity,"being≡nothing")'
    ],
    predicates: [{ name: 'BeingEqualsNothing', args: [] }],
    relations: [
      { predicate: 'equates', from: 'Being', to: 'Nothing' }
    ]
  }
]
