const WasmRPC = require('/Users/masonf/src/js-wasm-rpc-client').default;

async function run() {
    const complexArgs = new WasmRPC();
    await complexArgs.loadFile('../../target/wasm32-unknown-unknown/debug/complex_arguments.wasm');
    // await complexArgs.loadFile('dist/complex_arguments.wasm');
    let result = await complexArgs.call('run', 42, "hello", {key: "value"}, new Buffer([1,2,3]));

    console.log(result);
}
run();
