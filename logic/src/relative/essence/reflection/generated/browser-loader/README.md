# Browser Loader Chunks

Use this order in Neo4j Browser (one file at a time):
1. Paste and run: `01-schema-constraints.cypher`
2. Paste and run: `02-root-ontology.cypher`
3. Paste and run: `03-source-01-source-essence.cypher`
4. Paste and run: `03-source-02-source-reflection.cypher`
5. Paste and run: `03-source-03-source-shine.cypher`
6. Paste and run: `03-source-04-source-identity.cypher`
7. Paste and run: `03-source-05-source-difference.cypher`
8. Paste and run: `03-source-06-source-contradiction.cypher`
9. Paste and run: `03-source-07-source-determinate-ground.cypher`
10. Paste and run: `03-source-08-source-absolute-ground.cypher`
11. Paste and run: `03-source-09-source-condition.cypher`
12. Paste and run: `04-cross-chunk-traces.cypher`
13. Paste and run: `05-layer-membership.cypher`
14. Paste and run: `06-layer-relations.cypher`

Quick check after each step:
- `MATCH (n) RETURN count(n) AS nodes;`
- `MATCH ()-[r]->() RETURN count(r) AS rels;`

Notes:
- Property keys may persist in Neo4j Browser metadata even after deletes.
- If a step partially fails, re-run the same chunk; `MERGE` keeps it idempotent.
