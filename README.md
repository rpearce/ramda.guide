# ramda.guide

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

[https://ramda.guide](https://ramda.guide)

## Status

_In development_

## Dev

### Terminal 1

1. [Install nix](https://nixos.org/download.html)
1. [Install nix flakes](https://nixos.wiki/wiki/Flakes)
1. `λ nix build --extra-experimental-features flakes`
1. `λ nix run .#hull --extra-experimental-features flakes`
1. `λ nix develop --extra-experimental-features flakes`
    1. `[nix]λ cargo watch -w src/ -i src/book/book.toml -x run`

### Terminal 2

1. `λ cd web && python -m SimpleHTTPServer 8000`
