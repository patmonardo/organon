MATCH (a:ConsciousnessLayer {id: 'principle-citta'})
MATCH (b:ConsciousnessLayer {id: 'law-consciousness'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.operator = 'SUBLATION'
SET r.description = 'Principle sublates into particular lawful operation.';
MATCH (a:ConsciousnessLayer {id: 'law-consciousness'})
MATCH (b:ConsciousnessLayer {id: 'science-reflection'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.operator = 'SUBLATION'
SET r.description = 'Lawful contradiction sequence sublates into Reflection.';
MATCH (a:ConsciousnessLayer {id: 'principle-citta'})
MATCH (b:ConsciousnessLayer {id: 'science-reflection'})
MERGE (a)-[r:NEGATES]->(b)
SET r.operator = 'NEGATION'
SET r.description = 'Layered negation links the principle directly to reflection-science.';
