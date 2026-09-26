'use strict';

const renderColumnsLine = require('./render-columns-line');
const renderRow = require('./render-row');

// Renders already-read plain data (columns, rows of {kind, value}) as
// "ok\nempty\n" or "ok\ncols...\nrow...\n...". Touches no native handle: the
// caller (read-rows) owns that lifecycle.
function render(columns, rows) {
  if (rows.length === 0) return 'ok\nempty\n';
  return 'ok\n' + renderColumnsLine(columns) + rows.map((row) => renderRow(row)).join('');
}

module.exports = render;
