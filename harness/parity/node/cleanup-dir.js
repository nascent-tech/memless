'use strict';

const fs = require('node:fs');

// Removes a fixture's temp dir once a write or transaction is done with it.
function cleanupDir(dir) {
  fs.rmSync(dir, { recursive: true, force: true });
}

module.exports = cleanupDir;
