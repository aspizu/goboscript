{
  description = "goboscript is the Scratch compiler";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachSystem ["x86_64-linux" "aarch64-darwin"] (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (import rust-overlay)
        ];
      };
      rust = pkgs.rust-bin.stable.latest.default;
    in rec {
      packages.goboscript = pkgs.callPackage ./default.nix {
        inherit rust;
      };

      legacyPackages = packages;

      defaultPackage = packages.goboscript;

      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [rust git];
        packages = [packages.goboscript];
      };
    });
}
