let
  # Rolling updates, not deterministic.
  pkgs = import (fetchTarball ("channel:nixpkgs-unstable")) { config.allowUnfree = true; };
in
pkgs.mkShell {
  buildInputs = with pkgs; [ pkgconfig openssl gcc cargo rustc rust-analyzer vscode vscode-extensions.rust-lang.rust-analyzer ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
