{ lib, rustPlatform, pkg-config, openssl, rust }:

rustPlatform.buildRustPackage rec {
  pname = "goboscript";
  version = "3.0.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [ pkg-config rust ];
  buildInputs = [ openssl ];

  meta = {
    description = "Scratch compiler";
    homepage = "https://github.com/aspizu/goboscript";
    license = lib.licenses.mit;
  };
}
