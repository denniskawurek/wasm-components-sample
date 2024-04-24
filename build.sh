#!/usr/bin/env sh

(cd string-operations && cargo component build --release)
(cd app && cargo component build --release)

(cd string-operations/target/wasm32-wasi/release/; mv string_operations.wasm string-operations.wasm)
(wasm-tools compose app/target/wasm32-wasi/release/app.wasm -d string-operations/target/wasm32-wasi/release/string-operations.wasm -o out.wasm)