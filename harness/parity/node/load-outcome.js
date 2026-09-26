'use strict';

const { load } = require('@nascent-tech/memless');
const errorLine = require('./error-line');

// The plain-load block: "accepted" or the error tag.
function loadOutcome(path) {
  try {
    const instance = load(path);
    instance.release();
    return 'accepted';
  } catch (error) {
    return errorLine(error);
  }
}

module.exports = loadOutcome;
