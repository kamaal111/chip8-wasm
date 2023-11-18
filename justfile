default:
    just --list

build:
    #!/bin/zsh

    rm -rf www/node_modules www/.next
    yarn
    npx wasm-pack build
    just install-web-dependencies

run:
    just www/run

build-run: build run

bootstrap: install-node-modules

setup-dev-container:
    just .devcontainer/setup-dev-container
    just bootstrap

initialize-dev-container:
    just .devcontainer/initialize-dev-container

[private]
install-node-modules:
    yarn
    just www/bootstrap

[private]
install-web-dependencies:
    just www/install-node-modules
