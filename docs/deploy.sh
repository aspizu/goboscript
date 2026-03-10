#!/usr/bin/bash
set -e
export NO_MKDOCS_2_WARNING=1
uv sync
uv run mkdocs build --strict
