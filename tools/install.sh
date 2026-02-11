#!/bin/sh

if ! command -v cargo >/dev/null 2>&1; then
    if ! command -v rustup >/dev/null 2>&1; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    fi
    . $HOME/.cargo/env
fi

cargo install --git https://github.com/aspizu/goboscript
