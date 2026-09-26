'use strict';

const test = require('node:test');
const assert = require('node:assert/strict');
const path = require('node:path');
const { spawnSync } = require('node:child_process');

test('the cdylib loads lazily: requiring the bridge never touches MEMLESS_LIB', () => {
  const script = "require('" + path.join(__dirname, '..', 'src', 'index') + "'); console.log('required')";
  const result = spawnSync(process.execPath, ['-e', script], {
    env: { ...process.env, MEMLESS_LIB: '/nonexistent/path/lib.dylib' },
    encoding: 'utf8',
  });
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /required/);
});

test('a bogus MEMLESS_LIB only fails on the first real call', () => {
  const bridge = path.join(__dirname, '..', 'src', 'index');
  const script = `const { load } = require('${bridge}'); load('does-not-matter.yaml');`;
  const result = spawnSync(process.execPath, ['-e', script], {
    env: { ...process.env, MEMLESS_LIB: '/nonexistent/path/lib.dylib' },
    encoding: 'utf8',
  });
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /memless cdylib not found at \/nonexistent\/path\/lib\.dylib/);
});
