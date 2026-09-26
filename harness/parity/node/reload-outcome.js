'use strict';

const copyFixture = require('./copy-fixture');
const cleanupDir = require('./cleanup-dir');
const loadOrFail = require('./load-or-fail');
const reportReload = require('./report-reload');

// Mode "reload": copy, load, run the named scenario, clean up.
function reloadOutcome(fixture, scenario) {
  const paths = copyFixture(fixture);
  const { instance, failed } = loadOrFail(paths.dest);
  const report = instance ? reportReload(instance, scenario, fixture, paths) : failed;
  cleanupDir(paths.dir);
  return report;
}

module.exports = reloadOutcome;
