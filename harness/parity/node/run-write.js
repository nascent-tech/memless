'use strict';

const writeError = require('./write-error');

// One write verb's outcome: "accepted:<n>" or the error tag, with the temp
// path normalised back to the fixture's basename.
function runWrite(instance, sql, dest, base) {
  try {
    return 'accepted:' + instance.execute(sql);
  } catch (error) {
    return writeError(error).split(dest).join(base);
  }
}

module.exports = runWrite;
