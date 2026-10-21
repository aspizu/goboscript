# Install

## Install from source

Clones and installs the latest version from the git repository. Requires `git` and the
[Rust toolchain](https://rustup.rs/).

```bash
git clone https://github.com/aspizu/goboscript
cd goboscript
cargo install --path .
```

To update:

```bash
cd goboscript
git pull
cargo install --path .
```

## Install from source (using cargo)

Installs the latest version from the git repository in a single command.

```bash
cargo install --git https://github.com/aspizu/goboscript --locked
```

To update:

```bash
cargo install --git https://github.com/aspizu/goboscript --locked --force
```

## Install with nix

!!! note

    The nix flake installs goboscript from source, like the other methods, so you will need to be patient.

    You can also [install the goboscript VSCode extension with nix](./editor-integration/vscode.md).

### devShell

The demo devshell can be accessed with `nix develop github:aspizu/goboscript#demo`

### Nixos standalone installation (flake)

To install the goboscript *compiler*, you can add `goboscript.packages.${pkgs.stdenv.hostPlatform.system}.goboscript` to your system packages list.
For nix flakes, add the input `goboscript` and add it to `environment.systemPackages` in your flake like so:

[This repository](https://codeberg.org/rtk/dotfiles) is an example usage of the goboscript flake.

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=26.05";
    goboscript.url = "github:aspizu/goboscript";
  };
  outputs = { self, nixpkgs, goboscript, ... }: {
    nixosConfigurations.yourHostname = nixpkgs.lib.nixosSystem {
      modules = [
        ({ pkgs, ... }: {
          environment.systemPackages = [
            goboscript.packages.${pkgs.stdenv.hostPlatform.system}.goboscript # this is the important line
          ];
        })
        # The rest of your nixos configuration...
      ];
    };
  };
}
```
