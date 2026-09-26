'use strict';

const copyFixture = require('./copy-fixture');
const cleanupDir = require('./cleanup-dir');
const loadOrFail = require('./load-or-fail');
const writeReport = require('./write-report');
const withReadonlyDir = require('./with-readonly-dir');

// Mode "write-disk": as write, but the dir is read-only while the atomic
// rewrite runs, forcing it to fail.
function writeDiskOutcome(fixture, sql) {
  const paths = copyFixture(fixture);
  const { instance, failed } = loadOrFail(paths.dest);
  const report = instance ? withReadonlyDir(paths.dir, () => writeReport(instance, sql, paths)) : failed;
  cleanupDir(paths.dir);
  return report;
}

module.exports = writeDiskOutcome;
