#!/bin/sh

cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/canvas_wasm_webdom.wasm .