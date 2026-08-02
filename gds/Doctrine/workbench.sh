#!/usr/bin/env bash

set -euo pipefail

# Doctrine Workbench runner.
#
# Purpose:
# - make Doctrine study executable for researchers
# - run a canonical mediation path anchored in TaskFrame reasoning
#
# Usage:
#   bash gds/Doctrine/workbench.sh quick
#   bash gds/Doctrine/workbench.sh mediation
#   bash gds/Doctrine/workbench.sh shell

mode="${1:-quick}"

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

run_examples() {
  local title="$1"
  shift

  echo
  echo "== Doctrine Workbench: ${title} =="
  for example_name in "$@"; do
    echo
    echo "-> ${example_name}"
    cargo +stable run -p gds --example "${example_name}"
  done
}

print_artifacts() {
  echo
  echo "== Doctrine Workbench Artifacts =="
  find gds/fixtures/collections -maxdepth 3 -type f \
    \( -path "*shell/shell_compute_protocol*" \
    -o -path "*eval/eval_shell_taskframe_projection*" \
    -o -path "*taskframe/taskframe_shell_model_feature_plan*" \
    -o -path "*procedures/032-pathfinding-procedure-facade*" \) \
    | sort
}

case "$mode" in
  quick)
    run_examples \
      "Quick Orientation (DataFrame -> TaskFrame)" \
      "dataframe_intuition" \
      "dataset_model_moment" \
      "shell_compute_protocol"
    ;;
  mediation)
    run_examples \
      "Mediation Spine (shell_ -> proc_ -> eval_)" \
      "shell_compute_protocol" \
      "proc_pathfinding_procedure" \
      "eval_shell_taskframe_projection"
    ;;
  shell)
    run_examples \
      "Shell Canonical Slice" \
      "shell_model_first" \
      "shell_feature_first" \
      "shell_plan_first" \
      "shell_compute_protocol"
    ;;
  *)
    echo "Unknown mode: ${mode}" >&2
    echo "Expected one of: quick | mediation | shell" >&2
    exit 1
    ;;
esac

print_artifacts
