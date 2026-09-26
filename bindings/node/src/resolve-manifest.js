'use strict';

function resolveManifest(resolve, request) {
  try {
    return resolve(request);
  } catch {
    return null;
  }
}

module.exports = resolveManifest;
