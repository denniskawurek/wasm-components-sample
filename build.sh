#!/usr/bin/env sh

(cd string-length && cargo component build --release)
(cd app && cargo component build --release)

#mv string_length/target/wasm32-wasi/release/string_length.wasm string_length/target/wasm32-wasi/release/string-length.wasm

(wasm-tools compose app/target/wasm32-wasi/release/app.wasm -d string-length/target/wasm32-wasi/release/string-length.wasm -o out.wasm)