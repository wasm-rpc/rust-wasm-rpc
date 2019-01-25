const WasmRPC = require('/Users/masonf/src/js-wasm-rpc-client').default;

async function run() {
  const complexArgs = new WasmRPC();
  await complexArgs.loadFile('dist/wasm_snappy.wasm');
  let bytes = new Buffer(10000);
  let result = await complexArgs.call("run", bytes);
  let compressionPercentage = (1 - (Buffer.byteLength(result) / Buffer.byteLength(bytes))) * 100;
  console.log(`Compressed the buffer by: ${compressionPercentage}%`)

}
run();
