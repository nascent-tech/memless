'use strict';

function sameColumns(publicColumns, rawColumns) {
  const sameLength = publicColumns.length === rawColumns.length;
  return sameLength && publicColumns.every((name, index) => name === rawColumns[index]);
}

module.exports = sameColumns;
