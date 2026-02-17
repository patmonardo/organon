MERGE (layer:ConsciousnessLayer {id: 'principle-citta'})
SET layer.title = 'Principle of Consciousness (CITTA)'
SET layer.kind = 'PRINCIPLE'
SET layer.description = 'Kantian-style membership protocol where CIT and CITI are sublated members of CITTA.'
SET layer.chunkIds = ['idn-3', 'idn-4', 'idn-6'];
MATCH (protocol:CITProtocol {id: 'cit-citi-citta-reflection-debug-ir'})
MATCH (layer:ConsciousnessLayer {id: 'principle-citta'})
MERGE (protocol)-[:HAS_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'idn-3'})
MATCH (layer:ConsciousnessLayer {id: 'principle-citta'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'idn-4'})
MATCH (layer:ConsciousnessLayer {id: 'principle-citta'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'idn-6'})
MATCH (layer:ConsciousnessLayer {id: 'principle-citta'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MERGE (layer:ConsciousnessLayer {id: 'law-consciousness'})
SET layer.title = 'Law of Consciousness (Particular Operation)'
SET layer.kind = 'LAW'
SET layer.description = 'Operational sublation where identity and difference are driven into contradiction.'
SET layer.chunkIds = ['diff-13', 'diff-14', 'diff-15', 'ctr-1', 'ctr-4', 'ctr-5', 'ctr-10'];
MATCH (protocol:CITProtocol {id: 'cit-citi-citta-reflection-debug-ir'})
MATCH (layer:ConsciousnessLayer {id: 'law-consciousness'})
MERGE (protocol)-[:HAS_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'diff-13'})
MATCH (layer:ConsciousnessLayer {id: 'law-consciousness'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'diff-14'})
MATCH (layer:ConsciousnessLayer {id: 'law-consciousness'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'diff-15'})
MATCH (layer:ConsciousnessLayer {id: 'law-consciousness'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ctr-1'})
MATCH (layer:ConsciousnessLayer {id: 'law-consciousness'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ctr-4'})
MATCH (layer:ConsciousnessLayer {id: 'law-consciousness'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ctr-5'})
MATCH (layer:ConsciousnessLayer {id: 'law-consciousness'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ctr-10'})
MATCH (layer:ConsciousnessLayer {id: 'law-consciousness'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MERGE (layer:ConsciousnessLayer {id: 'science-reflection'})
SET layer.title = 'Science of Consciousness (Reflection + Ground)'
SET layer.kind = 'SCIENCE'
SET layer.description = 'Reflection completed through Ground: layered negation operators, grounding, and conditioned procession to concrete existence.'
SET layer.chunkIds = ['ref-1', 'ref-2', 'ref-3', 'ref-4', 'ref-5', 'ref-6', 'ref-7', 'ref-8', 'ref-9', 'ref-10', 'ref-11', 'ref-12', 'ref-13', 'ref-14', 'ref-15', 'ref-16', 'ref-17', 'ref-18', 'ref-19', 'det-1', 'det-2', 'det-3', 'det-4', 'det-5', 'det-6', 'det-7', 'det-8', 'det-9', 'det-10', 'det-11', 'det-12', 'det-13', 'det-14', 'abs-1', 'abs-2', 'abs-3', 'abs-4', 'abs-5', 'abs-6', 'abs-7', 'abs-8', 'abs-9', 'abs-10', 'abs-11', 'abs-12', 'abs-13', 'abs-14', 'abs-15', 'abs-16', 'con-1', 'con-2', 'con-3', 'con-4', 'con-5', 'con-6', 'con-7', 'con-8', 'con-9', 'con-10', 'con-11', 'con-12', 'con-13', 'con-14', 'con-15', 'con-16'];
MATCH (protocol:CITProtocol {id: 'cit-citi-citta-reflection-debug-ir'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (protocol)-[:HAS_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-1'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-2'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-3'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-4'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-5'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-6'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-7'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-8'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-9'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-10'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-11'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-12'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-13'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-14'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-15'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-16'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-17'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-18'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'ref-19'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-1'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-2'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-3'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-4'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-5'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-6'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-7'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-8'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-9'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-10'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-11'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-12'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-13'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'det-14'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-1'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-2'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-3'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-4'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-5'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-6'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-7'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-8'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-9'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-10'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-11'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-12'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-13'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-14'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-15'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'abs-16'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-1'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-2'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-3'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-4'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-5'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-6'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-7'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-8'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-9'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-10'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-11'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-12'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-13'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-14'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-15'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
MATCH (chunk:IntegratedChunk {id: 'con-16'})
MATCH (layer:ConsciousnessLayer {id: 'science-reflection'})
MERGE (chunk)-[:IN_LAYER]->(layer);
