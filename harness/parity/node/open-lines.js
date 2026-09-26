'use strict';

const runWrite = require('./run-write');
const runReload = require('./run-reload');
const queryInstruction = require('./query-instruction');
const nameQuery = require('./reload-name-query');

const UPDATE = "UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'";

// The "open" reload scenario: BEGIN, a refused reload, then an UPDATE and a
// SELECT on the still-open transaction, then ROLLBACK — the transaction stays
// open and usable across the refusal.
function openLines(instance, paths) {
  const begin = runWrite(instance, 'BEGIN', paths.dest, paths.base);
  const reload = runReload(instance, paths.dest, paths.base);
  const update = runWrite(instance, UPDATE, paths.dest, paths.base);
  const read = queryInstruction(instance, nameQuery);
  const rollback = runWrite(instance, 'ROLLBACK', paths.dest, paths.base);
  return [begin, reload, update, read, rollback];
}

module.exports = openLines;
