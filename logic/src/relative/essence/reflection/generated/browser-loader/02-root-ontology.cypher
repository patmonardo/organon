MERGE (protocol:CITProtocol {id: 'cit-citi-citta-reflection-debug-ir'})
SET protocol.mode = 'debug'
SET protocol.title = 'Integrated Reflection Chunking/TopicMap IR — CIT-CITI-CITTA'
SET protocol.section = 'Doctrine of Essence / Reflection / Foundation + Ground'
SET protocol.generatedAt = '2026-02-17T13:17:59.335Z'
SET protocol.totalSources = 9
SET protocol.totalChunks = 118;

MERGE (cit:CITCategory {id: 'CIT'})
SET cit.title = 'Intuition';
MERGE (citi:CITCategory {id: 'CITI'})
SET citi.title = 'Concept';
MERGE (citta:CITCategory {id: 'CITTA'})
SET citta.title = 'Science of Consciousness';
MERGE (cit)-[:SUBLATED_MEMBER_OF]->(citta);
MERGE (citi)-[:SUBLATED_MEMBER_OF]->(citta);
MERGE (protocol)-[:ROOT_CATEGORY]->(citta);
MERGE (protocol)-[:MEMBER_CATEGORY]->(cit);
MERGE (protocol)-[:MEMBER_CATEGORY]->(citi);
