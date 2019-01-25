const { edit } = require("@webassemblyjs/wasm-edit");
const fs = require('fs');
const buf = fs.readFileSync('../target/wasm32-unknown-unknown/release/base_token.wasm');

const visitors = {
  Func(path) {
    if(path.node.name.value.match("fmt..")) {
      path.remove();
    }
  }
};

const newBinary = edit(buf, visitors);
fs.writeFileSync("base_token.wasm", newBinary);
