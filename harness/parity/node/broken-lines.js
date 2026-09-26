'use strict';

const fs = require('node:fs');
const runReload = require('./run-reload');
const queryInstruction = require('./query-instruction');
const nameQuery = require('./reload-name-query');

// The "broken" reload scenario: the file is made incoherent, the reload is
// refused with the structure message, and the old state stays readable.
function brokenLines(instance, paths) {
  fs.writeFileSync(paths.dest, 'users:\n  - name: Ada\n');
  const reload = runReload(instance, paths.dest, paths.base);
  return [reload, queryInstruction(instance, nameQuery)];
}

module.exports = brokenLines;
