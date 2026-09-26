'use strict';

const { MemlessRefusal } = require('@nascent-tech/memless');

// The write/transaction-verb error tag: "refused:<msg>" or "fault:<msg>" (no
// space), matching the PHP and Go drivers.
function writeError(error) {
  if (error instanceof MemlessRefusal) return 'refused:' + error.message;
  return 'fault:' + error.message;
}

module.exports = writeError;
