# Install

=== "unix"
    ```shell
    curl -fsSL https://aspiz.uk/goboscript.sh | sh
    ```

=== "Windows"
    ```shell
    iwr https://aspiz.uk/goboscript.ps1 | iex
    ```

## Install from source

This installs the latest bleeding-edge version from the git repository. You will need
`git`, and the [rust toolchain](https://rustup.rs/) installed.

```shell
git clone https://github.com/aspizu/goboscript
cd goboscript
cargo install --path .
```

To update the installation:

```shell
cd goboscript
git pull
cargo install --path .
```

## Install from source (using cargo)

This installs the latest bleeding-edge version from the git repository with a single
command.

```shell
cargo install --git https://github.com/aspizu/goboscript
```

To update the installation:

```shell
cargo install --git https://github.com/aspizu/goboscript --force
```
