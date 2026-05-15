#!/usr/bin/env bash
set -e

# Ensure Cargo is in PATH
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env"
export PATH="$HOME/.cargo/bin:$PATH"

cd "$(dirname "$0")"

exec node_modules/.bin/tauri dev "$@"
