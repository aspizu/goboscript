# Install

=== "unix"
    ```bash
    curl -fsSL https://aspiz.uk/goboscript.sh | sh
    ```

=== "Windows"
    ```bash
    iwr https://aspiz.uk/goboscript.ps1 | iex
    ```

## Install from source

This installs the latest bleeding-edge version from the git repository. You will need
`git`, and the [rust toolchain](https://rustup.rs/) installed.

```bash
git clone https://github.com/aspizu/goboscript
cd goboscript
cargo install --path .
```

To update the installation:

```bash
cd goboscript
git pull
cargo install --path .
```

## Install from source (using cargo)

This installs the latest bleeding-edge version from the git repository with a single
command.

```bash
cargo install --git https://github.com/aspizu/goboscript
```

To update the installation:

```bash
cargo install --git https://github.com/aspizu/goboscript --force
```
