// Neo4j Query Pack — Integrated Reflection/Ground IR
// IR: integrated-reflection-ground-topicmap-ir
// Generated for section: Doctrine of Essence / Reflection / Ground

// Q1: Graph inventory by labels
MATCH (n)
UNWIND labels(n) AS label
RETURN label, count(*) AS count
ORDER BY count DESC;

// Q2: Source -> Chunk chain
MATCH (s:SourceText)-[:HAS_CHUNK]->(c:IntegratedChunk)
RETURN s.id AS sourceId, c.id AS chunkId, c.globalOrder AS globalOrder, c.lineStart AS lineStart, c.lineEnd AS lineEnd
ORDER BY c.globalOrder;

// Q3: Cross-source trace edges
MATCH (a:IntegratedChunk)-[r:NEXT|NEGATES|SUBLATES|REFLECTS|MEDIATES]->(b:IntegratedChunk)
WHERE a.sourceId <> b.sourceId
RETURN a.id AS fromChunk, a.sourceId AS fromSource, type(r) AS rel, b.id AS toChunk, b.sourceId AS toSource, r.reason AS reason
ORDER BY a.globalOrder;

// Q4: Key points for one chunk (replace chunk id)
MATCH (c:IntegratedChunk {id: 'con-16'})-[:HAS_KEY_POINT]->(k:KeyPoint)
RETURN c.id AS chunkId, k.ordinal AS ordinal, k.text AS keyPoint
ORDER BY k.ordinal;