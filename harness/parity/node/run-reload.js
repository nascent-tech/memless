'use strict';

const writeError = require('./write-error');

// One reload's outcome: "reloaded" on Ok, or the error tag (no space), with
// the temp path normalised back to the fixture's basename.
function runReload(instance, dest, base) {
  try {
    instance.reload();
    return 'reloaded';
  } catch (error) {
    return writeError(error).split(dest).join(base);
  }
}

module.exports = runReload;
