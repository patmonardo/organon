// Neo4j Query Pack — Actuality/Actuality IR
// IR: actuality-actuality-topicmap-ir
// Generated for section: Doctrine of Essence / Actuality / Actuality

// Q1: Graph inventory by labels
MATCH (n)
UNWIND labels(n) AS label
RETURN label, count(*) AS count
ORDER BY count DESC;

// Q2: Source -> Chunk chain
MATCH (s:SourceText)-[:HAS_CHUNK]->(c:TopicMapChunk)
RETURN s.id AS sourceId, c.id AS chunkId, c.globalOrder AS globalOrder, c.lineStart AS lineStart, c.lineEnd AS lineEnd
ORDER BY c.globalOrder;

// Q3: Cross-source trace edges
MATCH (a:TopicMapChunk)-[r:NEXT|NEGATES|SUBLATES|REFLECTS|MEDIATES]->(b:TopicMapChunk)
WHERE a.sourceId <> b.sourceId
RETURN a.id AS fromChunk, a.sourceId AS fromSource, type(r) AS rel, b.id AS toChunk, b.sourceId AS toSource, r.reason AS reason
ORDER BY a.globalOrder;

// Q4: Key points for one chunk (replace chunk id)
MATCH (c:TopicMapChunk {id: 'act-c-11'})-[:HAS_KEY_POINT]->(k:KeyPoint)
RETURN c.id AS chunkId, k.ordinal AS ordinal, k.text AS keyPoint
ORDER BY k.ordinal;