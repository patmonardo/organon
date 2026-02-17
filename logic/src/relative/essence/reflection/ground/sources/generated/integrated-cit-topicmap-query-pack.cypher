// Neo4j Query Pack — Integrated Reflection IR
// IR: cit-citi-citta-reflection-debug-ir-ground
// Generated for section: Doctrine of Essence / Reflection / Foundation + Ground / GROUND

// Q1: Graph inventory by labels
MATCH (n)
UNWIND labels(n) AS label
RETURN label, count(*) AS count
ORDER BY count DESC;

// Q2: Graph inventory by relationship types
MATCH ()-[r]->()
RETURN type(r) AS relationship, count(*) AS count
ORDER BY count DESC;

// Q3: Provenance path TXT -> CHUNK -> TOPIC -> IR for one chunk
// Replace 'det-1' as needed.
MATCH (s:SourceText)-[:HAS_CHUNK_SEGMENT]->(seg:ChunkSegment {id: 'chunk:det-1'})-[:YIELDS_TOPIC]->(t:Topic)-[:LIFTED_TO_IR]->(c:IntegratedChunk)
RETURN s.id AS sourceId, s.sourceFile AS txtFile, seg.id AS chunkSegmentId, t.id AS topicId, c.id AS irChunkId, c.lineStart AS lineStart, c.lineEnd AS lineEnd;

// Q4: Full provenance chain list for a source
// Replace source id (e.g. 'source-condition').
MATCH (s:SourceText {id: 'source-condition'})-[:HAS_CHUNK_SEGMENT]->(seg:ChunkSegment)-[:YIELDS_TOPIC]->(t:Topic)-[:LIFTED_TO_IR]->(c:IntegratedChunk)
RETURN s.id AS sourceId, seg.id AS chunkSegmentId, t.id AS topicId, c.id AS irChunkId, c.lineStart AS startLine, c.lineEnd AS endLine
ORDER BY c.globalOrder;

// Q5: Doctrine layers with member counts
MATCH (layer:ConsciousnessLayer)<-[:IN_LAYER]-(c:IntegratedChunk)
RETURN layer.id AS layerId, layer.kind AS kind, count(c) AS chunkCount
ORDER BY layer.kind, layer.id;

// Q6: Principle -> Law -> Science traversal
MATCH p=(a:ConsciousnessLayer {id: 'principle-citta'})-[:SUBLATES|NEGATES*1..3]->(b:ConsciousnessLayer {id: 'science-reflection'})
RETURN p;

// Q7: Cross-section transition edges
MATCH (a:IntegratedChunk)-[r:NEXT|SPIRALS_TO|SUBLATES|LAYER_NEGATION]->(b:IntegratedChunk)
WHERE a.sourceId <> b.sourceId
RETURN a.id AS fromChunk, a.sourceId AS fromSource, type(r) AS rel, b.id AS toChunk, b.sourceId AS toSource, r.reason AS reason
ORDER BY a.globalOrder;

// Q8: CIT/CITI chunks sublated into CITTA
MATCH (c:IntegratedChunk)-[:CLASSIFIED_AS]->(cat:CITCategory)
WHERE cat.id IN ['CIT', 'CITI']
OPTIONAL MATCH (c)-[:SUBLATED_MEMBER_OF]->(root:CITCategory {id: "CITTA"})
RETURN c.id AS chunkId, cat.id AS role, root.id AS sublatedInto
ORDER BY c.globalOrder;

// Q9: Layered chunk sequence with traces
MATCH (c:IntegratedChunk)-[:IN_LAYER]->(layer:ConsciousnessLayer)
OPTIONAL MATCH (c)-[r:NEXT|NEGATES|SUBLATES|REFLECTS|MEDIATES|SPIRALS_TO|LAYER_NEGATION]->(n:IntegratedChunk)
RETURN layer.id AS layerId, c.id AS chunkId, c.globalOrder AS ord, type(r) AS traceType, n.id AS nextChunk
ORDER BY c.globalOrder;

// Q10: Quick sanity check counts
MATCH (s:SourceText)
WITH count(s) AS sources
MATCH (c:IntegratedChunk)
WITH sources, count(c) AS chunks
MATCH (seg:ChunkSegment)
WITH sources, chunks, count(seg) AS segments
MATCH (t:Topic)
RETURN sources, chunks, segments, count(t) AS topics;
