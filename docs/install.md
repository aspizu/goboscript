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

## Nixos standalone installation (flake)

!!! Note

For nix flakes, add the input `goboscript` and add it to `environment.systemPackages` in your flake, roughly like so:

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=25.11";
    goboscript.url = "github:aspizu/goboscript";
  };
  outputs = { self, nixpkgs, goboscript, ... }: {
    nixosConfigurations.yourHostname = nixpkgs.lib.nixosSystem {
      modules = [
        ({ pkgs, ... }: {
          environment.systemPackages = [
            goboscript.packages.${pkgs.stdenv.hostPlatform.system}.goboscript
          ];
        })
      ];
    };
  };
}
```

## Nix devShell

You can test goboscript without installing it to your system with the nix devshell.
This will create a subshell where `goboscript` is installed.

Simply run the command `nix develop github:aspizu/goboscript`
