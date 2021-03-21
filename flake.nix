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
      flake = false;
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
        rust-overlay' = import rust-overlay;
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay' ];
        };
        rust = (pkgs.rustChannelOf {
          date = "2021-03-04";
          channel = "nightly";
        }).rust;
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
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
          buildInputs = [ pkgs.cargo-watch ];
          nativeBuildInputs = [ rust ];
        };
      }
    );
}
