'use strict';

const runInstruction = require('./run-instruction');
const fileHash = require('./file-hash');
const residueFlag = require('./residue-flag');

// A whole ';;'-separated suite on one instance: one line per instruction,
// then sha256 and residue. Releases the instance once the file has been read.
function reportSuite(instance, suite, paths) {
  const lines = suite.split(';;').map((sql) => runInstruction(instance, sql, paths.dest, paths.base));
  const footer = 'sha256:' + fileHash(paths.dest) + '\nresidue:' + residueFlag(paths.dir, paths.base) + '\n';
  instance.release();
  return lines.join('\n') + '\n' + footer;
}

module.exports = reportSuite;
