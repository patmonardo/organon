#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
POLARS_REF="$ROOT/ref/polars"
DATAFRAME="$ROOT/gds/src/collections/dataframe"

if [[ ! -d "$POLARS_REF" ]]; then
  echo "Missing ref/polars at: $POLARS_REF" >&2
  exit 1
fi

if [[ ! -d "$DATAFRAME" ]]; then
  echo "Missing dataframe module at: $DATAFRAME" >&2
  exit 1
fi

section() {
  echo
  echo "## $1"
}

compare_dirs() {
  local left="$1"
  local right="$2"
  local label_left="$3"
  local label_right="$4"

  echo "### $label_left vs $label_right"
  comm -3 \
    <(find "$left" -maxdepth 1 -mindepth 1 -type d -printf '%f\n' | sort) \
    <(find "$right" -maxdepth 1 -mindepth 1 -type d -printf '%f\n' | sort)
}

compare_files_without_ext() {
  local left="$1"
  local right="$2"
  local left_ext="$3"
  local right_ext="$4"
  local label_left="$5"
  local label_right="$6"

  echo "### $label_left vs $label_right"
  comm -3 \
    <(find "$left" -maxdepth 1 -mindepth 1 -type f -name "*.$left_ext" -printf '%f\n' | sed "s/\.$left_ext$//" | sort) \
    <(find "$right" -maxdepth 1 -mindepth 1 -type f -name "*.$right_ext" -printf '%f\n' | sed "s/\.$right_ext$//" | sort)
}

stamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
echo "# Polars Surface Diff"
echo
echo "Generated: $stamp"
echo "Reference: ref/polars"
echo "Target: gds/src/collections/dataframe"

section "Top-Level Directories"
compare_dirs "$POLARS_REF" "$DATAFRAME" "polars/*" "dataframe/*"

section "Expr Modules"
compare_files_without_ext "$POLARS_REF/expr" "$DATAFRAME/expressions" py rs "polars/expr/*.py" "dataframe/expressions/*.rs"

section "Functions Modules"
compare_files_without_ext "$POLARS_REF/functions" "$DATAFRAME/functions" py rs "polars/functions/*.py" "dataframe/functions/*.rs"

section "Series Modules"
compare_files_without_ext "$POLARS_REF/series" "$DATAFRAME/namespaces" py rs "polars/series/*.py" "dataframe/namespaces/*.rs"
