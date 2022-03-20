# ramda.guide

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

[https://ramda.guide](https://ramda.guide)

## Status

_In development_

## Dev

### With Docker

With `docker compose` (recommended):

```sh
docker compose build
docker compose up
```

Without `docker compose`:

```sh
docker build . -t ramda-guide
docker volume create ramda-guide-vol
docker run --rm -it -v $PWD:/service ramda-guide:latest
```

### Without Docker

1. [Install nix](https://nixos.org/download.html)
1. [Install nix flakes](https://nixos.wiki/wiki/Flakes)
1. `λ nix build`
1. `λ nix run`
1. `λ nix develop`
    1. `[nix]λ cargo watch -w "./src/" -i "./src/book/book.toml" -x run`

### Start Dev HTTP Server

1. `λ cd web && python -m SimpleHTTPServer 8000`
