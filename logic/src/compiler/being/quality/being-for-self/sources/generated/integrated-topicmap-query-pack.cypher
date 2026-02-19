// Neo4j Query Pack — Integrated Being-for-self IR
// IR: integrated-being-for-self-topicmap-ir
// Generated for section: Doctrine of Being / Quality / Being-for-itself

// Q1: Graph inventory by labels
MATCH (n)
UNWIND labels(n) AS label
RETURN label, count(*) AS count
ORDER BY count DESC;

// Q2: Source -> Chunk chain
MATCH (s:SourceText)-[:HAS_CHUNK]->(c:IntegratedChunk)
RETURN s.id AS sourceId, c.id AS chunkId, c.globalOrder AS globalOrder, c.lineStart AS lineStart, c.lineEnd AS lineEnd
ORDER BY c.globalOrder;

// Q3: Trace edges inside this IR
MATCH (:IntegratedIR {id: 'integrated-being-for-self-topicmap-ir'})-[:HAS_SOURCE]->(:SourceText)-[:HAS_CHUNK]->(a:IntegratedChunk)-[r:NEXT|NEGATES|SUBLATES|REFLECTS|MEDIATES]->(b:IntegratedChunk)
RETURN a.id AS fromChunk, type(r) AS rel, b.id AS toChunk, r.reason AS reason
ORDER BY a.globalOrder;

// Q4: Key points for one chunk (replace chunk id)
MATCH (c:IntegratedChunk {id: 'being-for-self-a'})-[:HAS_KEY_POINT]->(k:KeyPoint)
RETURN c.id AS chunkId, k.ordinal AS ordinal, k.text AS keyPoint
ORDER BY k.ordinal;