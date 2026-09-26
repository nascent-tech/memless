'use strict';

const isSelect = require('./is-select');
const queryInstruction = require('./query-instruction');
const runWrite = require('./run-write');

// One instruction in a transaction suite: a SELECT reads, anything else writes.
function runInstruction(instance, sql, dest, base) {
  if (isSelect(sql)) return queryInstruction(instance, sql);
  return runWrite(instance, sql, dest, base);
}

module.exports = runInstruction;
