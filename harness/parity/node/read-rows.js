'use strict';

const { native } = require('@nascent-tech/memless/src/library');
const columns = require('@nascent-tech/memless/src/columns');
const readRow = require('./read-row');

// Reads a raw result handle into plain data (columns, rows of {kind, value}),
// releasing it even if a read raises. The only place that owns this handle's
// lifecycle: render and the public-query cross-check both consume its output.
function readRows(result) {
  try {
    const header = columns(result);
    const count = Number(native.rowCount(result));
    const rows = Array.from({ length: count }, (_, row) => readRow(result, row, header.length));
    return { columns: header, rows };
  } finally {
    native.releaseResult(result);
  }
}

module.exports = readRows;
