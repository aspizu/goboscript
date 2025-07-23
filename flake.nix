{
  description = "Scratch compiler";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, ... }:
  flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-darwin" ] (system: let
    pkgs = nixpkgs.legacyPackages.${system};
  in rec {
    packages.goboscript = pkgs.callPackage ./default.nix { };

    legacyPackages = packages;

    defaultPackage = packages.goboscript;

    devShell = pkgs.mkShell {
      buildInputs = with pkgs; [ cargo rustc git ];
    };
  });
}

