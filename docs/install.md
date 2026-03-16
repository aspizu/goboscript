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

## Troubleshooting

### Rust Toolchain: Nightly Required

goboscript uses unstable Rust features (specifically `normalize_lexically`) that are only available on the **nightly** release channel. Attempting to install with the default stable toolchain will fail with:

```
error[E0554]: `#![feature]` may not be used on the stable release channel
```

Before installing, make sure you have the nightly toolchain available:

```sh
rustup toolchain install nightly
```

Then install goboscript by passing the `+nightly` flag to Cargo:

```sh
cargo +nightly install --git https://github.com/aspizu/goboscript
```

!!! note
    You do not need to switch your default toolchain to nightly globally.
    The `+nightly` flag tells Cargo to use the nightly toolchain for this command only,
    leaving your system default unchanged.
