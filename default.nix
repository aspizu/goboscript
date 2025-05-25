{ lib, rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "goboscript";
  version = "3.0.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = {
    description = "Scratch compiler";
    homepage = "https://github.com/aspizu/goboscript";
    license = lib.licenses.mit;
  };
}