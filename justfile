default:
    just --list

build:
    npx wasm-pack build
    just install-web-dependencies
    just www/build

run:
    just www/run

build-run: build run

bootstrap: install-node-modules

[private]
install-node-modules:
    yarn
    just www/bootstrap

[private]
install-web-dependencies:
    just www/install-node-modules
