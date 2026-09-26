'use strict';

const fs = require('node:fs');
const path = require('node:path');

// The workspace root, three levels above bindings/node/src.
const root = path.resolve(__dirname, '..', '..', '..');

const names = [
  'target/release/libmemless_capi.dylib',
  'target/debug/libmemless_capi.dylib',
  'target/release/libmemless_capi.so',
  'target/debug/libmemless_capi.so',
];

// libraryPath resolves the cdylib from MEMLESS_LIB, a trusted (ideally
// absolute) path, or, inside a checked-out workspace, under target/.
// MEMLESS_LIB loads arbitrary native code, like any FFI library path.
function libraryPath() {
  const env = process.env.MEMLESS_LIB;
  if (env && !fs.existsSync(env)) {
    throw new Error(`memless cdylib not found at ${env}; set MEMLESS_LIB`);
  }
  if (env) {
    return env;
  }
  const found = names.map((name) => path.join(root, name)).find((file) => fs.existsSync(file));
  if (!found) {
    throw new Error('memless cdylib not found; set MEMLESS_LIB');
  }
  return found;
}

module.exports = { libraryPath };
