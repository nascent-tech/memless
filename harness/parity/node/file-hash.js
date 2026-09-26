'use strict';

const fs = require('node:fs');
const crypto = require('node:crypto');

// The rewritten file's sha256, or "-" when it no longer exists.
function fileHash(dest) {
  if (!fs.existsSync(dest)) return '-';
  return crypto.createHash('sha256').update(fs.readFileSync(dest)).digest('hex');
}

module.exports = fileHash;
