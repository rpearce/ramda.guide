{
  description = "ramda.guide";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { naersk, nixpkgs, rust-overlay, self, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import rust-overlay)
          naersk.overlay
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable.latest.default;
        naersk-lib = pkgs.naersk.override {
          cargo = pkgs.rust-bin.nightly.latest.cargo;
          rustc = rust;
        };
        rust-dev = rust.override {
          extensions = [
            "clippy-preview"
            "rust-src"
            "rustc-dev"
            "rustfmt-preview"
          ];
        };
      in rec {
        packages = utils.lib.flattenTree {
          hull = naersk-lib.buildPackage {
            pname = "hull";
            root = ./.;
          };
        };

        defaultPackage = packages.hull;

        apps.hull = utils.lib.mkApp {
          drv = packages.hull;
        };

        defaultApp = apps.hull;

        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.rust-analyzer
            #pkgs.cargo-watch
          ];
          nativeBuildInputs = [ rust-dev ];
        };
      }
    );
}
