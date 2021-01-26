{ pkgs }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    mdbook
  ];

  shellHook = ''
  '';
}
