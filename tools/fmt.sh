#!/bin/bash

# Format all code.
#
# Usage:
#    ./tools/fmt.sh

set -euo pipefail
IFS=$'\n\t'

cd "$(cd "$(dirname "$0")" && pwd)"/..

# shellcheck disable=SC2046
if [[ -z "${CI:-}" ]]; then
    cargo fmt --all
    shfmt -l -w $(git ls-files '*.sh')
    prettier -l -w $(git ls-files '*.yml')
    prettier -l -w $(git ls-files '*.js')
    clang-format -i $(git ls-files '*.proto')
else
    cargo fmt --all -- --check
    shfmt -d $(git ls-files '*.sh')
    prettier -c $(git ls-files '*.yml')
    prettier -c $(git ls-files '*.js')
    clang-format -i $(git ls-files '*.proto')
    git diff --exit-code
fi
