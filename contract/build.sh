#!/bin/sh

echo ">> Building contract"

rustup target add wasm32-unknown-unknown
RUSTFLAGS="-C link-args=-s" cargo build --target wasm32-unknown-unknown --release