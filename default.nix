{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
  vscode-utils,
}: rec {
  default = goboscript;
  goboscript = rustPlatform.buildRustPackage {
    pname = "goboscript";
    version = "3.3.0";

    src = ./.;

    cargoLock.lockFile = ./Cargo.lock;

    nativeBuildInputs = [pkg-config];
    buildInputs = [openssl];

    meta = {
      description = "goboscript is the Scratch compiler";
      homepage = "https://github.com/aspizu/goboscript";
      license = lib.licenses.mit;
    };
  };
  # Just fetch the existing builds which are sent to the VS Marketplace
  goboscript-vscode = vscode-utils.buildVscodeMarketplaceExtension {
    mktplcRef = {
      name = "goboscript";
      publisher = "aspizu";
      version = "1.0.3";
      hash = "sha256-myVGhoighWMktD78qHTTs/t0dU0S68xX0aWbWiFEtqs=";
    };
  };
}
