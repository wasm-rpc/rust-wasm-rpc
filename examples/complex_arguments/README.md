# Example: Complex Arguments
A simple example of a WASM RPC executable.

The `run` function takes an integer, a string and an object. All the values are
modified and returned to the caller. All encoding and decoding is handled by the
`wasm_rpc` macros.

## Build

    $ ./scripts/build.sh
    ...
    $ node run.js
    [ 43, 'hello world', { key: 'value', key2: 'value2' } ]


