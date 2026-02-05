#!/usr/bin/env bash
set -euo pipefail

# Simple repository check to prohibit forbidden spellings in source files
# Allowed: "Dataset" (internal type), "DATASET" in docs/headers, and the special-case "GDSDataFrame"
# Forbidden: "DataSet", "dataSet", "GdsDataFrame", "gdsDataFrame" in source code files

patterns=("DataSet" "dataSet" "GdsDataFrame" "gdsDataFrame")

# File extensions to check (use git to list tracked files)
files=$(git ls-files '*.rs' '*.ts' '*.tsx' '*.js' '*.jsx' '*.py' '*.java' '*.go' '*.cpp' '*.c' '*.h' '*.json' '*.toml' '*.yaml' '*.yml' || true)

# Exclude doc directories and large build/output dirs
exclude_regex='(^gds/doc/|^logic/doc/|^model/doc/|^doc/|^reality/doc/|^target/|^node_modules/|^\.git/)'

bad=0
while IFS= read -r f; do
  # skip files in excluded directories
  if echo "$f" | grep -qE "$exclude_regex"; then
    continue
  fi

  for p in "${patterns[@]}"; do
    if grep -nH -e "$p" -- "$f" >/dev/null 2>&1; then
      echo "Forbidden pattern '$p' found in $f:" >&2
      grep -n -e "$p" -- "$f" >&2
      bad=1
    fi
  done

done <<< "$files"

if [ "$bad" -eq 1 ]; then
  echo >&2
  echo "Naming policy violation: use 'Dataset' (PascalCase) for internal types and avoid 'DataSet' / 'dataSet' in source files." >&2
  echo "See gds/doc/DATASET_NAMING.md for the policy." >&2
  exit 1
fi

echo "No forbidden dataset spellings found."
