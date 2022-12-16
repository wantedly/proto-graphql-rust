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
    if type -P shfmt &>/dev/null; then
        shfmt -l -w $(git ls-files '*.sh')
    fi
    if type -P npm &>/dev/null; then
        npx prettier -l -w $(git ls-files '*.yml') $(git ls-files '*.js')
    fi
    if type -P clang-format &>/dev/null; then
        clang-format -i $(git ls-files '*.proto')
    fi
else
    cargo fmt --all --check
    shfmt -d $(git ls-files '*.sh')
    npx prettier -c $(git ls-files '*.yml') $(git ls-files '*.js')
    clang-format -i $(git ls-files '*.proto')
    git diff --exit-code
fi
