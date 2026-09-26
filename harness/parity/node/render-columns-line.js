'use strict';

const escape = require('./escape');

// Renders the header line: "cols" then one escaped, tab-prefixed name each.
function renderColumnsLine(columns) {
  return 'cols' + columns.map((name) => '\t' + escape(name)).join('') + '\n';
}

module.exports = renderColumnsLine;
