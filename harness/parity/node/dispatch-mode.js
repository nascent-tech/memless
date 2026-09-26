'use strict';

const writeOutcome = require('./write-outcome');
const writeDiskOutcome = require('./write-disk-outcome');
const transactionOutcome = require('./transaction-outcome');
const transactionDiskOutcome = require('./transaction-disk-outcome');

// Routes a 3-argument invocation (path, sql, mode) to its outcome function.
function dispatchMode(path, sql, mode) {
  if (mode === 'write') return writeOutcome(path, sql);
  if (mode === 'write-disk') return writeDiskOutcome(path, sql);
  if (mode === 'transaction') return transactionOutcome(path, sql);
  if (mode === 'transaction-disk') return transactionDiskOutcome(path, sql);
  return '';
}

module.exports = dispatchMode;
