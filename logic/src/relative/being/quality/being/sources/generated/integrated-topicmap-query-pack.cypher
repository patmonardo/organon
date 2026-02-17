// Neo4j Query Pack — Integrated Being IR
// IR: integrated-being-topicmap-ir
// Generated for section: Doctrine of Being / Quality / Being-Nothing-Becoming

// Q1: Graph inventory by labels
MATCH (n)
UNWIND labels(n) AS label
RETURN label, count(*) AS count
ORDER BY count DESC;

// Q2: Source -> Chunk chain
MATCH (s:SourceText)-[:HAS_CHUNK]->(c:IntegratedChunk)
RETURN s.id AS sourceId, c.id AS chunkId, c.globalOrder AS globalOrder, c.lineStart AS lineStart, c.lineEnd AS lineEnd
ORDER BY c.globalOrder;

// Q3: Topic hierarchy in this IR
MATCH (:IntegratedIR {id: 'integrated-being-topicmap-ir'})-[:HAS_TOPIC]->(t:Topic)
OPTIONAL MATCH (p:Topic)-[:PARENT_OF]->(t)
RETURN t.id AS topicId, t.name AS name, t.path AS path, t.level AS level, p.id AS parentId
ORDER BY t.level, t.ordinal;

// Q4: Chunk -> Topic mapping
MATCH (:IntegratedIR {id: 'integrated-being-topicmap-ir'})-[:HAS_SOURCE]->(:SourceText)-[:HAS_CHUNK]->(c:IntegratedChunk)-[r:ABOUT]->(t:Topic)
RETURN c.id AS chunkId, c.globalOrder AS globalOrder, t.id AS topicId, t.name AS topicName, r.reason AS reason
ORDER BY c.globalOrder, t.level;

// Q4b: Chunk -> Becoming concept evidence mapping
MATCH (:IntegratedIR {id: 'integrated-being-topicmap-ir'})-[:HAS_SOURCE]->(:SourceText)-[:HAS_CHUNK]->(c:IntegratedChunk)-[r:ABOUT]->(t:Topic)
WHERE t.id STARTS WITH 'topic-concept-becoming-'
RETURN c.id AS chunkId, c.globalOrder AS globalOrder, t.id AS conceptId, t.name AS conceptName, r.reason AS reason
ORDER BY c.globalOrder, t.name;

// Q5: Trace edges inside this IR
MATCH (:IntegratedIR {id: 'integrated-being-topicmap-ir'})-[:HAS_SOURCE]->(:SourceText)-[:HAS_CHUNK]->(a:IntegratedChunk)-[r:NEXT|NEGATES|SUBLATES|REFLECTS|MEDIATES]->(b:IntegratedChunk)
RETURN a.id AS fromChunk, type(r) AS rel, b.id AS toChunk, r.reason AS reason
ORDER BY a.globalOrder;

// Q6: Key points for one chunk (replace chunk id)
MATCH (c:IntegratedChunk {id: 'being-1'})-[:HAS_KEY_POINT]->(k:KeyPoint)
RETURN c.id AS chunkId, k.ordinal AS ordinal, k.text AS keyPoint
ORDER BY k.ordinal;