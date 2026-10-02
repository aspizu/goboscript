{
  description = "goboscript is the Scratch compiler";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];
    forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system (pkgsFor system));
    pkgsFor = system: nixpkgs.legacyPackages.${system};
  in {
    packages = forAllSystems (_: pkgs: pkgs.callPackages ./default.nix {});
    devShells = forAllSystems (system: pkgs: {
      default = pkgs.mkShell {
        buildInputs = with pkgs; [git cargo rustc];
        packages = [self.packages.${system}.goboscript];
      };
    });
  };
}
