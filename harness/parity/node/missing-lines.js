'use strict';

const fs = require('node:fs');
const runReload = require('./run-reload');
const queryInstruction = require('./query-instruction');
const nameQuery = require('./reload-name-query');

// The "missing" reload scenario: the file is deleted, the reload is refused
// naming the path, and the old state stays readable.
function missingLines(instance, paths) {
  fs.rmSync(paths.dest);
  const reload = runReload(instance, paths.dest, paths.base);
  return [reload, queryInstruction(instance, nameQuery)];
}

module.exports = missingLines;
