import type { Chunk, LogicalOperation } from '../essence'

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'being-2-nothing-pure-emptiness',
    title: 'Nothing: pure nothingness; complete emptiness; absence of determination; same as pure being',
    text: `B. NOTHING

Nothing, pure nothingness;
it is simple equality with itself,
complete emptiness,
complete absence of determination and content;
lack of all distinction within.
In so far as mention can be made here of
intuiting and thinking,
it makes a difference whether something or nothing is
being intuited or thought.
To intuit or to think nothing has therefore a meaning;
the two are distinguished and so nothing is (concretely exists)
in our intuiting or thinking;
or rather it is the empty intuiting and thinking itself,
like pure being.
Nothing is therefore the same determination
or rather absence of determination,
and thus altogether the same as what pure being is.`,
    summary:
      'Pure nothingness = complete emptiness, absence of determination/content, no internal distinction; thinking/intuition of nothing is empty thinking/empty intuiting; nothing is altogether the same as pure being.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'nothing-op-1-pure-emptiness',
    chunkId: 'being-2-nothing-pure-emptiness',
    label: 'Pure nothingness: complete emptiness; absence of determination/content',
    digest:
      'Marks Nothing as pure nothingness: complete emptiness, absence of determination and content.',
    clauses: [
      'tag(Nothing,"pure")',
      'tag(Nothing,"nothingness")',
      'tag(Nothing,"complete-emptiness")',
      'tag(Nothing,"absence-of-determination")',
      'tag(Nothing,"absence-of-content")'
    ],
    predicates: [{ name: 'DefinesPureNothing', args: [] }],
    relations: []
  },
  {
    id: 'nothing-op-2-self-equality',
    chunkId: 'being-2-nothing-pure-emptiness',
    label: 'Simple equality with itself',
    digest:
      'Nothing has simple equality with itself; not unequal to another.',
    clauses: [
      'assert(equalOnlyToItself(Nothing))',
      'assert(not(unequalToAnother(Nothing)))'
    ],
    predicates: [{ name: 'SelfEquality', args: [] }],
    relations: []
  },
  {
    id: 'nothing-op-3-no-internal-distinction',
    chunkId: 'being-2-nothing-pure-emptiness',
    label: 'Lack of all distinction within',
    digest:
      'No internal distinction is found in Nothing.',
    clauses: [
      'annotate(Nothing,{noDifference:"internal"})',
      'tag(Nothing,"no-internal-distinction")'
    ],
    predicates: [{ name: 'NoInternalDifference', args: [] }],
    relations: [
      { predicate: 'lacks', from: 'Nothing', to: 'Difference' }
    ]
  },
  {
    id: 'nothing-op-4-empty-intuiting-thinking',
    chunkId: 'being-2-nothing-pure-emptiness',
    label: 'Empty intuiting and empty thinking (of nothing)',
    digest:
      'To intuit/think nothing is meaningful as empty intuiting/empty thinking (distinguished from something).',
    clauses: [
      'assert(distinguished(Intuiting,Something,Nothing))',
      'assert(distinguished(Thinking,Something,Nothing))',
      'tag(Intuiting,"empty")',
      'tag(Thinking,"empty")',
      'annotate(Nothing,{positedIn:["intuiting","thinking"]})'
    ],
    predicates: [{ name: 'EmptyIntuitionThinking', args: [] }],
    relations: [
      { predicate: 'positedIn', from: 'Nothing', to: 'Intuiting' },
      { predicate: 'positedIn', from: 'Nothing', to: 'Thinking' }
    ]
  },
  {
    id: 'nothing-op-5-same-as-pure-being',
    chunkId: 'being-2-nothing-pure-emptiness',
    label: 'Nothing is altogether the same as pure being',
    digest:
      'Nothing is the same (absence of determination) as pure being; identity of nothing and pure being.',
    clauses: [
      'assert(equalsTruth(Nothing, Being))',
      'tag(Identity,"nothing≡being")',
      'tag(Nothing,"same-absence-of-determination-as-being")'
    ],
    predicates: [{ name: 'NothingEqualsPureBeing', args: [] }],
    relations: [
      { predicate: 'equates', from: 'Nothing', to: 'Being' }
    ]
  }
]
