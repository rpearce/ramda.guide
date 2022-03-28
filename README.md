# ramda.guide

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

[https://ramda.guide](https://ramda.guide)

## Status

_In development_

## Setup

### With Docker

#### With `docker compose` (recommended)

```sh
docker compose build
docker compose up
```

#### Without `docker compose`

```sh
docker build . -t ramda-guide
docker volume create ramda-guide-vol
docker run --rm -it -v $PWD:/service ramda-guide:latest
```

### Without Docker

1. [Install nix](https://nixos.org/download.html)
1. [Install nix flakes](https://nixos.wiki/wiki/Flakes)

## Commands

If you are using Docker, prefix the following with `docker compose run --rm
app`; for example:

```sh
docker compose run --rm app nix run .#watch
```

* `nix build`: build the package
* `nix run` or `nix run .#app`: run the package
* `nix run .#watch`: watch the package for changes and rerun
* `nix develop`: enter a reproducible rust shell environment
  * How to watch for changes and rerun:
    ```sh
    cargo watch -w "./src/" -i "./src/book/book.toml" -x "run"
    ```

### Dev HTTP Server

To view the output, run the following in a separate window/pane:

```sh
cd web && python -m SimpleHTTPServer 8000
```

### Updating the cachix cache

https://docs.cachix.org/pushing
