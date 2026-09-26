'use strict';

const fs = require('node:fs');
const path = require('node:path');
const resolveManifest = require('./resolve-manifest');

// The library shipped by the platform package, such as
// @nascent-tech/memless-darwin-arm64, when npm installed it; null otherwise.
function embeddedLibrary(platform, resolve) {
  if (!platform) {
    return null;
  }
  const manifest = resolveManifest(resolve, `@nascent-tech/memless-${platform.name}/package.json`);
  if (!manifest) {
    return null;
  }
  const file = path.join(path.dirname(manifest), platform.file);
  return fs.existsSync(file) ? file : null;
}

module.exports = embeddedLibrary;
