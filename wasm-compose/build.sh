#!/bin/sh

set -x

(cd service && cargo component build --release)
(cd middleware && cargo component build --release)
(cd server && \
    wasm-tools compose -c config.yml \
        -o service.wasm \
        ../middleware/target/wasm32-wasi/release/middleware.wasm
)
