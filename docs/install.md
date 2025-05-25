# Install

## Install from binaries

Pre-built binaries are available for Windows, macOS, and Linux.

Download the latest release from <https://github.com/aspizu/goboscript/releases>, and
extract the archive to a folder in your `PATH`.

On Windows, you can copy the `goboscript.exe` file to `C:\Windows\System32` to make it
available from the command line.

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

## Install from crates.io

This installs the latest stable version from crates.io with a single command.

```shell
cargo install goboscript
```

To update the installation:

```shell
cargo install goboscript --force
```
