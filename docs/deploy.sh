#!/usr/bin/bash
set -e
cd "$(realpath -m "$0/../..")"
[[ -z "$MKDOCS_GIT_COMMITTERS_APIKEY" ]] && export MKDOCS_GIT_COMMITTERS_APIKEY="$(gh auth token)"
export NO_MKDOCS_2_WARNING=1
uv sync
uv run mkdocs build --strict -d "$(pwd)/docs/site/"
