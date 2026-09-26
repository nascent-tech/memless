'use strict';

const libcFromLdd = require('./libc-from-ldd');
const libcFromReport = require('./libc-from-report');

// 'glibc', 'musl' or null: /usr/bin/ldd first, then the diagnostic report.
// The bridge checks the libc itself because an older npm ignores the libc
// field of the platform packages.
function detectLibc(probe) {
  return libcFromLdd(probe.ldd()) || libcFromReport(probe.report());
}

module.exports = detectLibc;
