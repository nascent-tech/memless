'use strict';

const fs = require('node:fs');

// Runs fn while dir is read-only, then always restores it, even if fn
// raises — otherwise a raise would leave the temp dir stuck read-only,
// and cleanup after it would fail too.
function withReadonlyDir(dir, fn) {
  fs.chmodSync(dir, 0o555);
  try {
    return fn();
  } finally {
    fs.chmodSync(dir, 0o755);
  }
}

module.exports = withReadonlyDir;
