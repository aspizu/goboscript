gdsl:
    python gdsl.py
    cargo +nightly fmt
    cargo fix --allow-dirty
