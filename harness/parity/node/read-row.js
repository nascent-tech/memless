'use strict';

const readCellRaw = require('./read-cell-raw');
const cellValue = require('@nascent-tech/memless/src/cell-value');

// One row as an array of {kind, value}: value is the same conversion the
// bridge's public surface uses, kept alongside its kind tag since a decimal
// and a safe-range integer are both a plain JS number once converted.
function readRow(result, row, width) {
  const cells = Array.from({ length: width }, (_, column) => {
    const { kind, holders } = readCellRaw(result, row, column);
    return { kind, value: cellValue(kind, holders) };
  });
  return cells;
}

module.exports = readRow;
