'use strict';

const copyFixture = require('./copy-fixture');
const cleanupDir = require('./cleanup-dir');
const loadOrFail = require('./load-or-fail');
const reportSuite = require('./report-suite');
const withReadonlyDir = require('./with-readonly-dir');

// Mode "transaction-disk": as transaction, but the dir is read-only while the
// atomic rewrite at COMMIT runs, forcing it to fail.
function transactionDiskOutcome(fixture, suite) {
  const paths = copyFixture(fixture);
  const { instance, failed } = loadOrFail(paths.dest);
  const report = instance ? withReadonlyDir(paths.dir, () => reportSuite(instance, suite, paths)) : failed;
  cleanupDir(paths.dir);
  return report;
}

module.exports = transactionDiskOutcome;
