import type { Chunk, LogicalOperation } from './index'

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'bfs-om-b-one-and-void',
    title: 'b) The one and the void',
    text: `The one is the void as the abstract self-reference of negation.
But the void, as nothing, is absolutely diverse
from the simple immediacy of the one,
from the being of the latter which is also affirmative,
and because the two stand in one single reference,
namely to the one, their diversity is posited;
however, as distinct from the affirmative being,
the nothing stands as void outside the one as existent.
Being-for-itself, determined in this way
as the one and the void,
has again acquired an existence.
The one and the void have their negative self-reference
as their common and simple terrain.
The moments of being-for-itself
come out of this unity,
become external to themselves;
for through the simple unity of the moments
the determination of being comes into play,
and the unity itself thus withdraws to one side,
is therefore lowered to existence,
and there it is confronted by its other determination
standing over against it, negation as such
and likewise as the existence of the nothing,
as the void.`,
    summary:
      'The one is void as abstract self-reference of negation. Void (nothing) is absolutely diverse from the one’s affirmative immediacy and stands outside it as existent. Determined as one-and-void, being-for-itself regains existence; both share negative self-reference as common ground. The unity withdraws, moments externalize, and being is confronted by negation as such and as the existent void.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'bfs-op-om-b1-one-is-void-as-abstract-self-reference',
    chunkId: 'bfs-om-b-one-and-void',
    label: 'The one is the void as abstract self-reference of negation',
    clauses: [
      'assert(is(TheOne,"void-as-abstract-self-reference-of-negation"))'
    ],
    predicates: [{ name: 'OneVoid_AbstractSelfRef', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-b2-void-diverse-and-outside',
    chunkId: 'bfs-om-b-one-and-void',
    label: 'Void (nothing) absolutely diverse from the one’s affirmative immediacy; stands outside the one as existent',
    clauses: [
      'assert(absolutelyDiverse("void","affirmative-immediacy-of-one"))',
      'assert(singleReferenceTo(TheOne,true))',
      'assert(standsOutsideAsExistent("void",TheOne,true))'
    ],
    predicates: [{ name: 'OneVoid_Diversity_Exteriority', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-b3-one-and-void-reacquire-existence',
    chunkId: 'bfs-om-b-one-and-void',
    label: 'Being-for-itself as one-and-void has again acquired an existence',
    clauses: [
      'assert(configuredAs(BeingForItself,["one","void"]))',
      'assert(hasExistenceAgain(BeingForItself,true))'
    ],
    predicates: [{ name: 'OneVoid_ExistenceAgain', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-b4-common-terrain-negative-self-reference',
    chunkId: 'bfs-om-b-one-and-void',
    label: 'One and void share negative self-reference as common simple terrain',
    clauses: [
      'assert(commonTerrain(["one","void"],"negative-self-reference"))',
      'assert(isSimple(CommonTerrain,true))'
    ],
    predicates: [{ name: 'OneVoid_CommonTerrain', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-b5-unity-withdraws-moments-externalize',
    chunkId: 'bfs-om-b-one-and-void',
    label: 'Unity withdraws; moments externalize; being’s determination enters; negation confronts as such and as existent void',
    clauses: [
      'assert(unityWithdrawsToOneSide(true))',
      'assert(momentsBecomeExternal(true))',
      'assert(determinationOfBeingEnters(true))',
      'assert(confrontedBy(["negation-as-such","existent-void"],true))'
    ],
    predicates: [{ name: 'OneVoid_Withdrawal_Confrontation', args: [] }],
    relations: []
  }
]
