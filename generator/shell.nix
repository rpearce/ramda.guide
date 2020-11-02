let
  cfg = import ../nix/default.nix { };
  hp = cfg.haskellPackages;
in
{}:

hp.shellFor {
  packages = p: [
    p.ramda-guide
  ];

  buildInputs = with hp; [
    cabal-install
    ghcid
    hlint
    hp.ramda-guide
    ormolu
  ];

  withHoogle = true;
}
