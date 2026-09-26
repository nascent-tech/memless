'use strict';

const { native } = require('@nascent-tech/memless/src/library');

// Reads a cell's kind tag alongside its four holders, bypassing the bridge's
// cell-value conversion so the kind survives to render (a JS number alone
// cannot tell an integer from a decimal apart, unlike Go's int64/float64).
function readCellRaw(result, row, column) {
  const holders = { integer: [0n], decimal: [0], boolean: [0], text: [null] };
  const kind = native.readCell(
    result, row, column,
    holders.integer, holders.decimal, holders.boolean, holders.text,
  );
  return { kind, holders };
}

module.exports = readCellRaw;
