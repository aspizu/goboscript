# Install

## Auto Install

Recommended method to install. Also installs sb2gs and backpack.

### Unix

```shell
curl -fsSL https://raw.githubusercontent.com/aspizu/goboscript/refs/heads/main/install.sh | sh
```

### Windows

```shell
iwr https://raw.githubusercontent.com/aspizu/goboscript/refs/heads/main/install.ps1 | iex
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
