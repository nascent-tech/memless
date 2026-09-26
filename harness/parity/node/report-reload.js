'use strict';

const reloadScenario = require('./reload-scenario');
const fileHash = require('./file-hash');
const residueFlag = require('./residue-flag');

// One named reload scenario's full block: its lines, then sha256, then
// residue. Releases the instance once the file has been read for both.
function reportReload(instance, scenario, fixture, paths) {
  const lines = reloadScenario(instance, scenario, fixture, paths);
  const footer = 'sha256:' + fileHash(paths.dest) + '\nresidue:' + residueFlag(paths.dir, paths.base) + '\n';
  instance.release();
  return lines.join('\n') + '\n' + footer;
}

module.exports = reportReload;
