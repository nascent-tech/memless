'use strict';

const renderCell = require('./render-cell');

// Renders one row of {kind, value} cells: "row" then one tab-prefixed cell each.
function renderRow(cells) {
  return 'row\t' + cells.map((cell) => renderCell(cell.kind, cell.value)).join('\t') + '\n';
}

module.exports = renderRow;
