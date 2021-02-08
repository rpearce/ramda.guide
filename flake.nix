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
          date = "2021-01-28";
          channel = "nightly";
        }).rust;
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };
      in rec {
        packages = utils.lib.flattenTree {
          mdbook = pkgs.mdbook;
          news = naersk-lib.buildPackage {
            pname = "news";
            root = ./src/news;
          };
        };

        defaultPackage = pkgs.mdbook;

        apps.book = utils.lib.mkApp {
          drv = pkgs.mdbook;
        };

        apps.news = utils.lib.mkApp {
          drv = packages.news;
        };

        # @TODO: combine builds into defaultApp
        defaultApp = apps.book;

        devShell = pkgs.mkShell {
          buildInputs = [ pkgs.mdbook ];
          nativeBuildInputs = [ rust ];
        };
      }
    );
}
