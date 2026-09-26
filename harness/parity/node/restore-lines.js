'use strict';

const fs = require('node:fs');
const runWrite = require('./run-write');
const runReload = require('./run-reload');
const queryInstruction = require('./query-instruction');
const nameQuery = require('./reload-name-query');

// The "restore" reload scenario: an UPDATE, the original bytes written back
// externally, a reload, then a SELECT reading the restored value.
function restoreLines(instance, fixture, paths) {
  const update = runWrite(instance, "UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'", paths.dest, paths.base);
  fs.copyFileSync(fixture, paths.dest);
  const reload = runReload(instance, paths.dest, paths.base);
  return [update, reload, queryInstruction(instance, nameQuery)];
}

module.exports = restoreLines;
