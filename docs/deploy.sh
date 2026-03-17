#!/usr/bin/bash
set -e
cd "$(realpath -m "$0/../..")"
[[ -z "$MKDOCS_GIT_COMMITTERS_APIKEY" ]] && export MKDOCS_GIT_COMMITTERS_APIKEY="$(gh auth token)"
export NO_MKDOCS_2_WARNING=1
uv sync
cd docs/javascripts
bunx --bun js-yaml ../../editors/code/syntaxes/goboscript.tmGrammar.yml > goboscript.tmGrammar.json
bun i
bun build --target=browser --format=iife --minify main.ts --outfile=main.js
[[ $1 == "--build-only" ]] && exit
rm -rf node_modules
cd ../..
uv run mkdocs build --strict
