{ pkgs ? (import <nixpkgs> {}).pkgs }:
with pkgs;
mkShell {
  buildInputs = [
      rustc
      rustfmt
      cargo
      cargo-info
      clippy
      rust-analyzer-unwrapped
      openssl
      pkg-config
  ];
    PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig";  
}
