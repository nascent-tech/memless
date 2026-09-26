'use strict';

const fs = require('node:fs');
const path = require('node:path');

// Whether the atomic-write residue file (D16) is still sitting next to base.
function residueFlag(dir, base) {
  return fs.existsSync(path.join(dir, '.' + base + '.memless-tmp')) ? 'yes' : 'no';
}

module.exports = residueFlag;
