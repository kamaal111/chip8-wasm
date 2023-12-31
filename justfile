default:
    just --list

build-release:
    #!/bin/zsh

    npx wasm-pack build --release
    just www/build-release

build-dev:
    #!/bin/zsh

    rm -rf www/node_modules
    yarn
    npx wasm-pack build
    just www/build-dev

run:
    just www/run

build-run-dev: build-dev run

setup-dev-container: 
    just .devcontainer/setup-dev-container
    just build-dev
    just install-node-modules
    just www/install-node-modules

initialize-dev-container:
    just .devcontainer/initialize-dev-container

[private]
install-node-modules:
    yarn
