#!/usr/bin/env bash
set -euo pipefail

if ! command -v cypher-shell >/dev/null 2>&1; then
  echo "Error: cypher-shell is not installed or not in PATH." >&2
  exit 1
fi

NEO4J_URI="${NEO4J_URI:-neo4j://localhost:7687}"
NEO4J_USER="${NEO4J_USER:-neo4j}"

if [[ -z "${NEO4J_PASSWORD:-}" ]]; then
  read -rsp "Neo4j password for ${NEO4J_USER}@${NEO4J_URI}: " NEO4J_PASSWORD
  echo
fi

run_query() {
  local title="$1"
  local query="$2"

  echo
  echo "=== ${title} ==="
  cypher-shell -a "${NEO4J_URI}" -u "${NEO4J_USER}" -p "${NEO4J_PASSWORD}" "${query}"
}

run_query "Node/relationship totals" "MATCH (n) WITH count(n) AS nodes MATCH ()-[r]->() RETURN nodes, count(r) AS rels;"
run_query "Layer cardinalities" "MATCH (layer:ConsciousnessLayer)<-[:IN_LAYER]-(c:IntegratedChunk) RETURN layer.id, layer.kind, count(c) AS chunkCount ORDER BY layer.kind, layer.id;"
run_query "Source chunk counts" "MATCH (s:SourceText)-[:HAS_CHUNK]->(c:IntegratedChunk) RETURN s.id, count(c) AS chunks ORDER BY s.id;"
run_query "Provenance chain counts" "MATCH (s:SourceText)-[:HAS_CHUNK_SEGMENT]->(:ChunkSegment)-[:YIELDS_TOPIC]->(:Topic)-[:LIFTED_TO_IR]->(c:IntegratedChunk) RETURN s.id, count(c) AS chains ORDER BY s.id;"
run_query "Cross-source transitions" "MATCH (a:IntegratedChunk)-[r:NEXT|SPIRALS_TO|SUBLATES|LAYER_NEGATION]->(b:IntegratedChunk) WHERE a.sourceId <> b.sourceId RETURN a.id, type(r), b.id, r.reason ORDER BY a.globalOrder;"

echo
echo "Post-checks complete."
