'use strict';

const copyFixture = require('./copy-fixture');
const cleanupDir = require('./cleanup-dir');
const loadOrFail = require('./load-or-fail');
const writeReport = require('./write-report');

// Mode "write": copy, load, run the sql, report, clean up.
function writeOutcome(fixture, sql) {
  const paths = copyFixture(fixture);
  const { instance, failed } = loadOrFail(paths.dest);
  const report = instance ? writeReport(instance, sql, paths) : failed;
  cleanupDir(paths.dir);
  return report;
}

module.exports = writeOutcome;
