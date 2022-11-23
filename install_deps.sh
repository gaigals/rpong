#!/bin/bash

cargo install wasm-pack
rustup target add wasm32-unknown-unknown

# pkg which will strip down WASM code.
cargo install wasm-gc

# pkg for creating .WAT files - ASCII represation of the WASM binary.
# Use `wasm2wat` to convert .wasm to .wat
cargo install wasm-bindgen-cli
