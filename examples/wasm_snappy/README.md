# Example: WASM Snappy

Sometimes it's helpful to compile a Rust crate to WebAssembly so it can be used
in a WebAssembly environment. This project wraps the [snap](https://crates.io/crates/snap) crate.

## Build

    $ ./scripts/build.sh
    ...
    $ node run.js
    Compressed the buffer by: 95.25%


