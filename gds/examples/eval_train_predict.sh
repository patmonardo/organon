#!/usr/bin/env bash

set -euo pipefail

examples=(
  "eval_train_template_law"
  "eval_predict_template_law"
  "eval_catalog_symbolics"
)

for example_name in "${examples[@]}"; do
  echo "${example_name}:"
  cargo run -p gds --example "${example_name}"
done

echo
echo "Rapid loop fixtures:"
find gds/fixtures/collections/eval -maxdepth 2 -type f \
  \( -path "*/eval_train_template_law/*" \
     -o -path "*/eval_predict_template_law/*" \
     -o -path "*/eval_catalog_symbolics/*" \) | sort
