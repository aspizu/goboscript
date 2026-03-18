```markdown
# Install

!!! tip
    goboscript requires the **nightly** Rust toolchain. Install it once with:
    ```bash
    rustup toolchain install nightly
    ```

## Install from source

Clones and installs the latest version from the git repository. Requires `git` and the
[Rust toolchain](https://rustup.rs/).

```bash
git clone https://github.com/aspizu/goboscript
cd goboscript
cargo +nightly install --path .
```

To update:

```bash
cd goboscript
git pull
cargo +nightly install --path .
```

## Install from source (using cargo)

Installs the latest version from the git repository in a single command.

```bash
cargo +nightly install --git https://github.com/aspizu/goboscript
```

To update:

```bash
cargo +nightly install --git https://github.com/aspizu/goboscript --force
```
```
