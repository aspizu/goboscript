gdsl:
    python gdsl.py
    cargo +nightly fmt
    cargo fix --allow-dirty

test:
    uv run tools/run.py tests/*
