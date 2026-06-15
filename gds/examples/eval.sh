#!/usr/bin/env bash

set -euo pipefail

examples=(
  "eval_workbench_catalog"
  "eval_map_surfaces"
  "eval_train_template_law"
  "eval_predict_template_law"
  "eval_facade_boundary_discipline"
  "eval_catalog_symbolics"
  "eval_shell_taskframe_projection"
)

for example_name in "${examples[@]}"; do
  echo "${example_name}:"
  cargo run -p gds --example "${example_name}"
done

echo
echo "Eval fixtures:"
find gds/fixtures/collections/eval -maxdepth 2 -type f | sort
