'use strict';

// Escapes backslash, tab and newline the same way the PHP and Go drivers do,
// so a column name or text cell renders identically on all three bridges.
function escape(text) {
  return text.replace(/\\/g, '\\\\').replace(/\t/g, '\\t').replace(/\n/g, '\\n');
}

module.exports = escape;
