'use strict';

// Whether an instruction in a ';;'-separated transaction suite is a SELECT.
function isSelect(sql) {
  return sql.trim().toUpperCase().startsWith('SELECT');
}

module.exports = isSelect;
