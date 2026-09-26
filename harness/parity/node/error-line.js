'use strict';

const { MemlessRefusal } = require('@nascent-tech/memless');

// The load/query/SELECT error tag: "refused: <msg>" or "fault: <msg>" (with
// a space), matching the PHP and Go drivers.
function errorLine(error) {
  if (error instanceof MemlessRefusal) return 'refused: ' + error.message;
  return 'fault: ' + error.message;
}

module.exports = errorLine;
