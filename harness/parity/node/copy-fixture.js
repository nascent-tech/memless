'use strict';

const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');

// Copies a fixture into a fresh temp dir so a write cannot touch the original.
function copyFixture(fixture) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'memless-parity-'));
  const base = path.basename(fixture);
  const dest = path.join(dir, base);
  fs.copyFileSync(fixture, dest);
  return { dir, dest, base };
}

module.exports = copyFixture;
