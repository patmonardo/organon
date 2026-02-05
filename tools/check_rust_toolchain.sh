#!/usr/bin/env bash
set -euo pipefail

# Fail if the active rust toolchain is 'nightly' or if a rust-toolchain* file pins to nightly.

if command -v rustc >/dev/null 2>&1; then
  v=$(rustc --version)
  if echo "$v" | grep -qi night; then
    echo "ERROR: Active rustc toolchain looks like nightly: $v" >&2
    exit 1
  fi
  echo "Active rustc: $v"
else
  echo "rustc not found in PATH; skipping active-toolchain check" >&2
fi

# Check for repo-checked toolchain files that pin nightly
files=$(git ls-files 'rust-toolchain' 'rust-toolchain.toml' || true)
if [ -n "$files" ]; then
  bad=0
  while IFS= read -r f; do
    if grep -qi 'nightly' "$f"; then
      echo "ERROR: $f pins to nightly" >&2
      grep -n 'nightly' "$f" || true
      bad=1
    fi
  done <<< "$files"
  if [ "$bad" -eq 1 ]; then
    exit 1
  fi
fi

echo "Rust toolchain check passed (no nightly detected)."
