#!/usr/bin/env bash
set -euo pipefail

mechanic_pattern='board|card|deck|grid|suit|resource|capture|hand|pile|trick|pot|auction|betting|drafting|role|scenario|faction|territory|initiative|eligibility'

if grep -RniE "${mechanic_pattern}" crates/engine-core/src; then
  echo "engine-core contains mechanic vocabulary" >&2
  exit 1
fi

tree_output="$(cargo tree -p engine-core)"
printf '%s\n' "${tree_output}"

if printf '%s\n' "${tree_output}" | grep -E 'game-stdlib|ai-core|wasm-api|games/|apps/web'; then
  echo "engine-core has a forbidden Rulepath dependency" >&2
  exit 1
fi

support_reverse_tree="$(cargo tree --workspace -e normal,build --invert game-test-support)"
printf '%s\n' "${support_reverse_tree}"

support_reverse_edges="$(printf '%s\n' "${support_reverse_tree}" | sed '1d')"
if [[ -n "${support_reverse_edges}" ]]; then
  echo "game-test-support has a forbidden normal/build reverse dependency" >&2
  exit 1
fi

echo "engine-core boundary check passed"
echo "game-test-support dev-only boundary check passed"
