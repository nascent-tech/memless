'use strict';

const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');

const root = path.resolve(__dirname, '..', '..', '..');

function fixture(name) {
  return path.join(root, 'harness', 'parity', 'fixtures', name);
}

const START_YAML =
  'users:\n  - id: 01H7B2\n    name: Ada\n  - id: 01H7B3\n    name: Grace\n'
  + 'wallets:\n  - id: w_123\n    user_id: 01H7B3\n    amount: 100\n'
  + '  - id: w_124\n    user_id: 01H7B2\n    amount: 250\n';

function loadTemp(load) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'memless-node-'));
  const file = path.join(dir, 'start.yaml');
  fs.writeFileSync(file, START_YAML, { mode: 0o600 });
  return { instance: load(file), dir, file };
}

module.exports = { fixture, loadTemp, START_YAML };
