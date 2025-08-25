#!/usr/bin/env sh
set -euo

export PATH="$PATH:~/.local/bin"
export PATH="$PATH:~/.cargo/bin"

if ! command -v cargo >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

cargo install --git https://github.com/aspizu/goboscript

if ! command -v uv >/dev/null 2>&1; then
    curl -LsSf https://astral.sh/uv/install.sh | sh
fi

uv tool install git+https://github.com/aspizu/sb2gs
uv tool install git+https://github.com/aspizu/backpack
