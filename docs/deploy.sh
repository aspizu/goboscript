#!/usr/bin/bash
set -e
cd "$(dirname "$(dirname "$(realpath "$0")")")"
export NO_MKDOCS_2_WARNING=1
uv sync
uv run mkdocs build --strict
