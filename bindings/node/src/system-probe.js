'use strict';

const fs = require('node:fs');

function ldd() {
  try {
    return fs.readFileSync('/usr/bin/ldd', 'latin1');
  } catch {
    return null;
  }
}

function report() {
  try {
    process.report.excludeNetwork = true;
    return process.report.getReport();
  } catch {
    return null;
  }
}

module.exports = { ldd, report };
