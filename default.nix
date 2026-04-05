{ lib, rustPlatform, pkg-config, openssl, rust }:

rustPlatform.buildRustPackage {
  pname = "goboscript";
  version = "3.3.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [ pkg-config rust ];
  buildInputs = [ openssl ];

  meta = {
    description = "goboscript is the Scratch compiler";
    homepage = "https://github.com/aspizu/goboscript";
    license = lib.licenses.mit;
  };
}
