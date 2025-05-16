{ pkgs ? import <nixpkgs> {} }:
with pkgs;
mkShell {
  nativeBuildInputs = [
    openssl
    gcc
    pkg-config
    alsa-lib
    xorg.libX11.dev
    rust-analyzer
  ];
}
