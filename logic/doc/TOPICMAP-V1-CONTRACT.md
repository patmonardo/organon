# TopicMap v1 Contract (Working Draft)

Status: draft for decision
Scope: `logic/` generators, starting with Being → Becoming

## Why this exists

Current output proves generation works, but semantics are mixed:

- `Topic` is used both as ontology and as section/category bucket.
- `IntegratedChunk -[:ABOUT]-> Topic` alone is not enough to represent concepts.

This contract separates **concept ontology** from **text evidence**.

## Topic Maps baseline (standards-oriented)

The classical Topic Maps model (ISO/IEC 13250 family; TMDM/XTM lineage) is built from these core parts:

1. **Topics**

- Subjects/concepts in the map.
- A topic is about a subject, not merely a document span.

2. **Topic names**

- Human-readable names for topics.
- Can be scoped by context (language, perspective, discipline).

3. **Occurrences**

- Information resources about a topic (documents, passages, URLs, notes).
- In our graph, chunk evidence behaves like occurrences.

4. **Associations**

- Typed relations between topics (e.g., hierarchy, opposition, transformation).
- In this project: `PARENT_OF`, `RELATES_TO`, `NEGATES`, `SUBLATES`.

5. **Roles in associations**

- Each participating topic plays a role in an association.
- In property-graph form, role semantics are usually captured by relationship type + direction + properties.

6. **Subject identity**

- Stable identity via subject identifiers/locators to merge same subject across sources.
- In this project: durable topic IDs and deterministic generation strategy.

7. **Scope / context**

- Contextual validity (e.g., translation, editorial school, periodization).
- Optional in v1, but should be representable later.

This is why v1 separates concept topics from chunk passages: chunks are evidence/occurrence-like, not the topics themselves.

## Node labels (and why)

- `IntegratedIR`
  - Generation context/container for one integrated artifact.
  - Keeps provenance and allows multiple generated IRs to coexist.

- `SourceText`
  - Source document metadata.

- `IntegratedChunk`
  - Passage-level text unit with line range and extracted key points.

- `ConceptTopic`
  - Conceptual node in the ontology (e.g., Being, Nothing, Becoming, Coming-to-be).
  - This is the primary “topic map” node.

- `SectionTopic` (optional but recommended)
  - Editorial/table-of-contents node (e.g., `C.2. The moments of becoming`).
  - Distinct from concept ontology to avoid semantic conflation.

## Relationship types (and why)

- `IntegratedIR -[:HAS_SOURCE]-> SourceText`
- `SourceText -[:HAS_CHUNK]-> IntegratedChunk`

Ontology layer:

- `IntegratedIR -[:HAS_CONCEPT]-> ConceptTopic`
- `ConceptTopic -[:PARENT_OF]-> ConceptTopic` (taxonomy/hierarchy)
- `ConceptTopic -[:RELATES_TO]-> ConceptTopic` (non-taxonomic conceptual relation)
- `ConceptTopic -[:NEGATES]-> ConceptTopic` (dialectical opposition)
- `ConceptTopic -[:SUBLATES]-> ConceptTopic` (dialectical transformation)

Evidence layer:

- `IntegratedChunk -[:MENTIONS]-> ConceptTopic`
  - Preferred over `ABOUT` for passage evidence semantics.
- `IntegratedChunk -[:HAS_KEY_POINT]-> KeyPoint` (existing)

Optional editorial layer:

- `IntegratedIR -[:HAS_SECTION]-> SectionTopic`
- `SectionTopic -[:PARENT_OF]-> SectionTopic`
- `IntegratedChunk -[:IN_SECTION]-> SectionTopic`

## Minimal invariants (must pass)

For each generated IR:

1. At least one `ConceptTopic` exists.
2. Every `ConceptTopic` is reachable from IR via `[:HAS_CONCEPT]`.
3. At least one ontology edge among `PARENT_OF | RELATES_TO | NEGATES | SUBLATES` exists.
4. At least one chunk-to-concept evidence edge exists (`MENTIONS`).
5. No chunk is attached only to sections without at least one concept mention.

## Becoming seed concept set (minimum 8)

For `C. BECOMING`, start with at least these conceptual topics:

1. Being
2. Nothing
3. Becoming
4. Coming-to-be
5. Ceasing-to-be
6. Unity (of being and nothing)
7. Vanishing
8. Existence

These are **ConceptTopic** nodes, distinct from passage IDs like `becoming-1..7`.

## Canonical validation queries

```cypher
// 1) IR concept count
MATCH (:IntegratedIR {id:$irId})-[:HAS_CONCEPT]->(c:ConceptTopic)
RETURN count(c) AS conceptCount;

// 2) Ontology connectivity
MATCH (:IntegratedIR {id:$irId})-[:HAS_CONCEPT]->(c:ConceptTopic)
OPTIONAL MATCH (c)-[r:PARENT_OF|RELATES_TO|NEGATES|SUBLATES]-(:ConceptTopic)
RETURN count(DISTINCT c) AS concepts, count(r) AS ontologyEdges;

// 3) Evidence coverage
MATCH (:IntegratedIR {id:$irId})-[:HAS_SOURCE]->(:SourceText)-[:HAS_CHUNK]->(ch:IntegratedChunk)
OPTIONAL MATCH (ch)-[m:MENTIONS]->(:ConceptTopic)
RETURN count(DISTINCT ch) AS chunks, count(m) AS mentionEdges;

// 4) Chunks lacking concept mentions (should be 0 for strict mode)
MATCH (:IntegratedIR {id:$irId})-[:HAS_SOURCE]->(:SourceText)-[:HAS_CHUNK]->(ch:IntegratedChunk)
WHERE NOT (ch)-[:MENTIONS]->(:ConceptTopic)
RETURN ch.id LIMIT 25;
```

## Migration notes from current model

Current model has:

- `Topic`
- `IntegratedChunk -[:ABOUT]-> Topic`

Migration strategy:

1. Rename/replace `Topic` with `ConceptTopic` for conceptual nodes.
2. Replace `ABOUT` with `MENTIONS` for passage evidence edges.
3. Introduce `SectionTopic` only for editorial headings if needed.
4. Keep `IntegratedChunk` and `KeyPoint` as evidence layer unchanged.

## Decision checkpoint for project continuation

Continue project only if team agrees to:

- Concept/evidence separation (`ConceptTopic` vs chunk/passages), and
- A strict generator contract with invariant checks.

If agreed, next implementation step is a single generator pilot (`generate-being-ir.ts`) that emits both concept ontology and evidence links under this contract.
