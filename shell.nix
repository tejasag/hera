{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
  buildInputs = [ 
    pkgs.cargo 
    pkgs.rustc
    pkgs.rustfmt
    pkgs.clippy
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
