'use strict';

const copyFixture = require('./copy-fixture');
const cleanupDir = require('./cleanup-dir');
const loadOrFail = require('./load-or-fail');
const reportSuite = require('./report-suite');

// Mode "transaction": copy, load, run the ';;'-separated suite, clean up.
function transactionOutcome(fixture, suite) {
  const paths = copyFixture(fixture);
  const { instance, failed } = loadOrFail(paths.dest);
  const report = instance ? reportSuite(instance, suite, paths) : failed;
  cleanupDir(paths.dir);
  return report;
}

module.exports = transactionOutcome;
