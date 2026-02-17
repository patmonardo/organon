#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

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

FILES=(
  "01-schema-constraints.cypher"
  "02-root-ontology.cypher"
  "03-source-01-source-essence.cypher"
  "03-source-02-source-reflection.cypher"
  "03-source-03-source-shine.cypher"
  "03-source-04-source-identity.cypher"
  "03-source-05-source-difference.cypher"
  "03-source-06-source-contradiction.cypher"
  "03-source-07-source-determinate-ground.cypher"
  "03-source-08-source-absolute-ground.cypher"
  "03-source-09-source-condition.cypher"
  "04-cross-chunk-traces.cypher"
  "05-layer-membership.cypher"
  "06-layer-relations.cypher"
)

for file in "${FILES[@]}"; do
  path="${SCRIPT_DIR}/${file}"
  if [[ ! -f "${path}" ]]; then
    echo "Error: missing file ${path}" >&2
    exit 1
  fi

  echo "==> Running ${file}"
  cypher-shell -a "${NEO4J_URI}" -u "${NEO4J_USER}" -p "${NEO4J_PASSWORD}" -f "${path}"
done

echo
echo "Load complete. Run post-checks with:"
echo "  bash ${SCRIPT_DIR}/run-post-checks.sh"
