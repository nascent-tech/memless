'use strict';

const libcFromLdd = require('./libc-from-ldd');
const libcFromReport = require('./libc-from-report');

// 'glibc', 'musl' or null: /usr/bin/ldd first, then the diagnostic report.
// The bundled Linux libraries need glibc, so musl must not load them.
function detectLibc(probe) {
  return libcFromLdd(probe.ldd()) || libcFromReport(probe.report());
}

module.exports = detectLibc;
