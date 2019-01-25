const {
  default: WasmRPC,
  WasmRPCError,
}= require('/Users/masonf/src/js-wasm-rpc-client');

async function run() {
  const fileName = '../../target/wasm32-unknown-unknown/debug/error_handler.wasm';
  let complexArgs = new WasmRPC();
  await complexArgs.loadFile(fileName);
  let result = complexArgs.call("success");

  // With catch
  complexArgs.call("error")
    .catch((error) => {
      if (error instanceof WasmRPCError) {
        console.log(`User error code: ${error.code}`)
        console.log(`User error message: ${error.message}`)
      }
  });

  // With aync/await
  try {
    await complexArgs.call("error")
  } catch (error) {
    if (error instanceof WasmRPCError) {
      console.log(`User error code: ${error.code}`)
      console.log(`User error message: ${error.message}`)
    } else {
      throw e;
    }
  }

  // Panic (don't print the ugly backtrace)
  complexArgs.call("panic").catch(() => null);

  // Custom panic message
  complexArgs = new WasmRPC();
  await complexArgs.loadFile(fileName);

  complexArgs.call("custom_panic").catch(() => null);

  // Argument parsing error
  complexArgs = new WasmRPC();
  await complexArgs.loadFile(fileName);
  complexArgs.call("bad_arg", "string").catch(() => null);

}
run();
