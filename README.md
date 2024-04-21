# Wasm components sample

This is an example for the usage of Wasm components.

[`string_length`](string_length/wit/world.wit) exports a function which is used by [`app`](app/wit/world.wit).

## Setup

Run either:

```
chmod +x build.sh
./build.sh
```

or

```
(cd string_length && cargo component build --release) \\
(cd app && cargo component build)
```

### The error

This leads to this error:

```
error: failed to create a target world for package `app` (/home/dennis/code/rust/wasm-components-sample/app/Cargo.toml)

Caused by:
    0: failed to merge local target `/home/dennis/code/rust/wasm-components-sample/app/wit`
    1: package not found
            --> /home/dennis/code/rust/wasm-components-sample/app/wit/world.wit:4:12
             |
           4 |     import dkwr:stringlength/len@0.1.0;
             |            ^----------------
```