'use strict';

const loadOutcome = require('./load-outcome');
const queryOutcome = require('./query-outcome');
const dispatchMode = require('./dispatch-mode');

// argv: [path], [path, sql], or [path, sql, mode] — same thresholds as the
// PHP and Go drivers' own argc dispatch.
function dispatchArgv(argv) {
  const [path, sql, mode] = argv;
  if (mode !== undefined) return process.stdout.write(dispatchMode(path, sql, mode));
  if (sql !== undefined) return process.stdout.write(queryOutcome(path, sql));
  process.stdout.write(loadOutcome(path) + '\n');
}

module.exports = dispatchArgv;
