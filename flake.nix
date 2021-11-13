{
  description = "ramda.guide";

  nixConfig.bash-prompt = "[nix]λ ";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
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
        # `nix build`
        packages.hull = naersk-lib.buildPackage {
          pname = "hull";
          root = ./.;
        };
        defaultPackage = packages.hull;

        # `nix run`
        apps.hull = flake-utils.lib.mkApp {
          drv = packages.hull;
        };
        defaultApp = apps.hull;

        # `nix develop`
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.cargo-edit
            pkgs.cargo-watch
            pkgs.rust-analyzer
          ];
          nativeBuildInputs = [ rust-dev ];
        };
      }
    );
}
