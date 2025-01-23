set shell := ["powershell.exe", "-c"]


gdsl:
    python gdsl.py
    cargo +nightly fmt
    cargo fix --allow-dirty
