'use strict';

const { load } = require('@nascent-tech/memless');
const errorLine = require('./error-line');
const runQuery = require('./run-query');

// The load-then-query block for one fixture and one sql line.
function queryOutcome(path, sql) {
  let instance;
  try {
    instance = load(path);
  } catch (error) {
    return errorLine(error) + '\n';
  }
  return runQuery(instance, sql);
}

module.exports = queryOutcome;
