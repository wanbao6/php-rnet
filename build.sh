#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="$SCRIPT_DIR/.env"

if [ ! -f "$ENV_FILE" ]; then
    echo "Error: .env file not found."
    echo "Copy .env.example to .env and adjust the paths for your system:"
    echo ""
    echo "  cp .env.example .env"
    echo "  \$EDITOR .env"
    exit 1
fi

# Load .env
set -a
# shellcheck disable=SC1090
source "$ENV_FILE"
set +a

# Validate required variables
missing=()
[ -z "$CMAKE_BIN" ]      && missing+=("CMAKE_BIN")
[ -z "$LIBCLANG_PATH" ]  && missing+=("LIBCLANG_PATH")
[ -z "$PHP_CONFIG" ]     && missing+=("PHP_CONFIG")
[ -z "$PHP" ]            && missing+=("PHP")

if [ ${#missing[@]} -gt 0 ]; then
    echo "Error: the following variables are not set in .env:"
    for v in "${missing[@]}"; do
        echo "  - $v"
    done
    exit 1
fi

export PATH="$HOME/.cargo/bin:$CMAKE_BIN:/usr/bin:$PATH"
export LIBCLANG_PATH
export PHP_CONFIG
export PHP
export LD_LIBRARY_PATH="${LD_LIBRARY_PATH_EXTRA:+$LD_LIBRARY_PATH_EXTRA:}$LD_LIBRARY_PATH"
[ -n "$BINDGEN_EXTRA_CLANG_ARGS" ] && export BINDGEN_EXTRA_CLANG_ARGS

cargo build --release "$@"
