'use strict';

const restoreLines = require('./restore-lines');
const openLines = require('./open-lines');
const brokenLines = require('./broken-lines');
const missingLines = require('./missing-lines');

// Dispatches one named reload scenario to its fixed script, returning its
// lines before the sha256/residue footer.
function reloadScenario(instance, scenario, fixture, paths) {
  if (scenario === 'restore') return restoreLines(instance, fixture, paths);
  if (scenario === 'open') return openLines(instance, paths);
  if (scenario === 'broken') return brokenLines(instance, paths);
  if (scenario === 'missing') return missingLines(instance, paths);
  return ['unknown reload scenario: ' + scenario];
}

module.exports = reloadScenario;
