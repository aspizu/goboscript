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

## Install with nix

!!! note

    The nix flake installs goboscript from source, like the other methods, so you will need to be patient.

### devShell

You can test goboscript without installing it to your system with the nix devshell, a bit like `nix-shell -p {some package}`.
This will create a subshell where `goboscript` is installed.
Once you exit this subshell, you will no longer be able to use the `goboscript` command
(until you open a new devShell or install it system-wide).
Once you run `nix-collect-garbage`, the `goboscript` installation files will actually be removed from your system.

Simply run the command `nix develop github:aspizu/goboscript`

### Nixos standalone installation (flake)

This is for if you want to have `goboscript` available system-wide.
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

