{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
        rec {
          # `nix build`
          packages.hera = naersk-lib.buildPackage {
            pname = "hera";
            root = ./.;
          };
          defaultPackage = packages.hera;

          # `nix run`
          apps.hera = flake-utils.lib.mkApp {
            drv = packages.hera;
          };
          defaultApp = apps.hera;

          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ 
              rustc 
              cargo
              rustfmt
              clippy
              rls
            ];
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        }
    );
}
