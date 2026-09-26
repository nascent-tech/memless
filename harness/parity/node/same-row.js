'use strict';

const sameCell = require('./same-cell');

// A public-query row (plain values) against a raw row (an array of {kind, value}).
function sameRow(publicRow, rawRow) {
  const sameLength = publicRow.length === rawRow.length;
  return sameLength && publicRow.every((value, index) => sameCell(rawRow[index].value, value));
}

module.exports = sameRow;
