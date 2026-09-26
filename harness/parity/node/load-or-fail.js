'use strict';

const { load } = require('@nascent-tech/memless');
const writeError = require('./write-error');

// Loads a copied fixture for a write or transaction; a load failure is
// already a well-formed block (sha256:- and residue:no, nothing was opened).
function loadOrFail(dest) {
  try {
    return { instance: load(dest), failed: null };
  } catch (error) {
    return { instance: null, failed: writeError(error) + '\nsha256:-\nresidue:no\n' };
  }
}

module.exports = loadOrFail;
