default:
    just --list

build:
    npx wasm-pack build
    just install-web-dependencies
    just copy-build-to-web

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

[private]
copy-build-to-web:
    mkdir -p www/node_modules
    rm -rf www/node_modules/chip8
    mkdir -p www/node_modules/chip8
    cp -r pkg/. www/node_modules/chip8
