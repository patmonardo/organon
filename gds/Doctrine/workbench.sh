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
#   bash gds/Doctrine/workbench.sh full
#   bash gds/Doctrine/workbench.sh map
#   bash gds/Doctrine/workbench.sh list
#   bash gds/Doctrine/workbench.sh example shell_compute_protocol
#   bash gds/Doctrine/workbench.sh cheap
#   bash gds/Doctrine/workbench.sh cheap-mediation
#   bash gds/Doctrine/workbench.sh cheap-shell
#   bash gds/Doctrine/workbench.sh cheap-example shell_compute_protocol
#
# Budget controls:
#   DOCTRINE_WORKBENCH_BUILD=0   # skip prebuild and run existing binaries only
#   DOCTRINE_WORKBENCH_REFRESH=0 # skip artifact listing for faster terminal output

mode="${1:-quick}"
target_example="${2:-}"
build_flag="${DOCTRINE_WORKBENCH_BUILD:-1}"
refresh_flag="${DOCTRINE_WORKBENCH_REFRESH:-1}"

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

apply_cheap_mode() {
  build_flag="0"
  refresh_flag="0"
}

prepare_examples() {
  if [[ "$build_flag" == "1" ]]; then
    echo
    echo "== Doctrine Workbench Build =="
    echo "prebuilding gds examples once for this session"
    cargo +stable build -p gds --examples
  else
    echo
    echo "== Doctrine Workbench Build =="
    echo "skipped (DOCTRINE_WORKBENCH_BUILD=0)"
  fi
}

run_one_example() {
  local example_name="$1"
  local example_bin="target/debug/examples/${example_name}"

  echo
  echo "-> ${example_name}"

  if [[ -x "$example_bin" ]]; then
    "$example_bin"
  elif [[ "$build_flag" == "0" ]]; then
    echo "missing binary: ${example_bin}" >&2
    echo "run once with DOCTRINE_WORKBENCH_BUILD=1 to build examples" >&2
    exit 1
  else
    # Fallback if a binary was not emitted for any reason.
    cargo +stable run -p gds --example "${example_name}"
  fi
}

run_examples() {
  local title="$1"
  shift

  echo
  echo "== Doctrine Workbench: ${title} =="
  for example_name in "$@"; do
    run_one_example "$example_name"
  done
}

write_session_map() {
  local session_root="gds/fixtures/collections/doctrine/doctrine_workbench_session"
  local session_file="${session_root}/00-session-map.txt"
  mkdir -p "$session_root"

  {
    echo "Doctrine Workbench Session Map"
    echo
    echo "mode: ${mode}"
    echo "timestamp: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
    echo
    echo "hierarchy: Doctrine -> References -> Exemplars -> Examples -> Fixtures"
    echo "mediation: DataFrame -> TaskFrame through shell_ -> proc_ -> eval_"
    echo
    echo "artifact roots:"
    echo "- gds/fixtures/collections/shell/shell_compute_protocol"
    echo "- gds/fixtures/collections/eval/eval_shell_taskframe_projection"
    echo "- gds/fixtures/task/workbench/taskframe_shell_model_feature_plan"
    echo "- gds/fixtures/procedures/032-pathfinding-procedure-facade"
  } > "$session_file"

  echo
  echo "== Doctrine Workbench Session Map =="
  echo "persisted: ${session_file}"
}

print_artifacts() {
  echo
  echo "== Doctrine Workbench Artifacts =="
  find gds/fixtures -maxdepth 5 -type f \
    \( -path "*collections/shell/shell_compute_protocol*" \
    -o -path "*collections/eval/eval_shell_taskframe_projection*" \
    -o -path "*task/workbench/taskframe_shell_model_feature_plan*" \
    -o -path "*procedures/032-pathfinding-procedure-facade*" \) \
    | sort
}

list_examples() {
  cat <<'EOF'
Doctrine Workbench Modes
- quick
- mediation
- shell
- full
- map
- list
- example <example_name>
- cheap
- cheap-mediation
- cheap-shell
- cheap-example <example_name>

Recommended low-cost example targets
- dataframe_intuition
- dataset_model_moment
- shell_compute_protocol
- proc_pathfinding_procedure
- eval_shell_taskframe_projection
- taskframe_shell_model_feature_plan
EOF
}

case "$mode" in
  quick)
    prepare_examples
    run_examples \
      "Quick Orientation (DataFrame -> TaskFrame)" \
      "dataframe_intuition" \
      "dataset_model_moment" \
      "shell_compute_protocol"
    ;;
  cheap)
    apply_cheap_mode
    prepare_examples
    run_examples \
      "Cheap Orientation (DataFrame -> TaskFrame)" \
      "dataframe_intuition" \
      "dataset_model_moment" \
      "shell_compute_protocol"
    ;;
  mediation)
    prepare_examples
    run_examples \
      "Mediation Spine (shell_ -> proc_ -> eval_)" \
      "shell_compute_protocol" \
      "proc_pathfinding_procedure" \
      "eval_shell_taskframe_projection"
    ;;
  cheap-mediation)
    apply_cheap_mode
    prepare_examples
    run_examples \
      "Cheap Mediation Spine (shell_ -> proc_ -> eval_)" \
      "shell_compute_protocol" \
      "proc_pathfinding_procedure" \
      "eval_shell_taskframe_projection"
    ;;
  shell)
    prepare_examples
    run_examples \
      "Shell Canonical Slice" \
      "shell_model_first" \
      "shell_feature_first" \
      "shell_plan_first" \
      "shell_compute_protocol"
    ;;
  cheap-shell)
    apply_cheap_mode
    prepare_examples
    run_examples \
      "Cheap Shell Canonical Slice" \
      "shell_model_first" \
      "shell_feature_first" \
      "shell_plan_first" \
      "shell_compute_protocol"
    ;;
  full)
    prepare_examples
    run_examples \
      "Systematic Spine (Doctrine -> Runnable Evidence)" \
      "dataset_frame_dsl" \
      "dataset_model_moment" \
      "dataset_plan_moment" \
      "dataframe_intuition" \
      "shell_model_first" \
      "shell_compute_protocol" \
      "proc_pathfinding_procedure" \
      "eval_shell_taskframe_projection" \
      "taskframe_shell_model_feature_plan"
    ;;
  map)
    echo
    echo "== Doctrine Workbench: Map Only =="
    echo "no example execution requested"
    ;;
  list)
    echo
    echo "== Doctrine Workbench: Available Modes =="
    list_examples
    ;;
  example)
    if [[ -z "$target_example" ]]; then
      echo "missing example name for 'example' mode" >&2
      echo "usage: bash gds/Doctrine/workbench.sh example shell_compute_protocol" >&2
      exit 1
    fi

    prepare_examples
    run_examples \
      "Single Example (${target_example})" \
      "$target_example"
    ;;
  cheap-example)
    if [[ -z "$target_example" ]]; then
      echo "missing example name for 'cheap-example' mode" >&2
      echo "usage: bash gds/Doctrine/workbench.sh cheap-example shell_compute_protocol" >&2
      exit 1
    fi

    apply_cheap_mode
    prepare_examples
    run_examples \
      "Cheap Single Example (${target_example})" \
      "$target_example"
    ;;
  *)
    echo "Unknown mode: ${mode}" >&2
    echo "Expected one of: quick | mediation | shell | full | map | list | example <name> | cheap | cheap-mediation | cheap-shell | cheap-example <name>" >&2
    exit 1
    ;;
esac

if [[ "$refresh_flag" == "1" ]]; then
  print_artifacts
else
  echo
  echo "== Doctrine Workbench Artifacts =="
  echo "skipped listing (DOCTRINE_WORKBENCH_REFRESH=0)"
fi

write_session_map
