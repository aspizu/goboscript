{
  description = "Scratch compiler";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, rust-overlay, ... }:
  flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-darwin" ] (system: let
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
    rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
  in rec {
    packages.goboscript = pkgs.callPackage ./default.nix {
      inherit (pkgs) pkg-config openssl;
      inherit rust;
    };

    legacyPackages = packages;

    defaultPackage = packages.goboscript;

    devShell = pkgs.mkShell {
      buildInputs = with pkgs; [ git openssl pkg-config ];
      nativeBuildInputs = [ rust ];
    };
  });
}
