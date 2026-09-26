'use strict';

const runWrite = require('./run-write');
const fileHash = require('./file-hash');
const residueFlag = require('./residue-flag');

// One write's full block: its line, then sha256, then residue. Releases the
// instance once the file has been read for both.
function writeReport(instance, sql, paths) {
  const line = runWrite(instance, sql, paths.dest, paths.base);
  const report = line + '\nsha256:' + fileHash(paths.dest) + '\nresidue:' + residueFlag(paths.dir, paths.base) + '\n';
  instance.release();
  return report;
}

module.exports = writeReport;
