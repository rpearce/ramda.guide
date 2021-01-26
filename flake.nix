{
  description = "test-mdbook";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/master";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, flake-utils, nixpkgs }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "x86_64-darwin" ]
      (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in
        {
          checks = {};

          devShell = import ./shell.nix { inherit pkgs; };

          defaultPackage = pkgs.mdbook;

          packages = flake-utils.lib.flattenTree {
            mdbook = pkgs.mdbook;
          };
        }
      );
}
