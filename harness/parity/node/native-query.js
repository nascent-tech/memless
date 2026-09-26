'use strict';

const { native } = require('@nascent-tech/memless/src/library');
const take = require('@nascent-tech/memless/src/message');
const ensureOk = require('@nascent-tech/memless/src/ensure-ok');

// Runs sql through the C ABI directly, keeping the raw result handle instead
// of the bridge's ergonomic {columns, rows} (which would discard cell kind).
function nativeQuery(handle, sql) {
  const outResult = [0n];
  const outMessage = [null];
  const status = native.runQuery(handle, sql, outResult, outMessage);
  ensureOk(status, take(outMessage));
  return outResult[0];
}

module.exports = nativeQuery;
