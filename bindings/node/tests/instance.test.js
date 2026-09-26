'use strict';

const test = require('node:test');
const assert = require('node:assert/strict');

const { load, Instance, MemlessRefusal, MemlessFault } = require('../src/index');
const { fixture } = require('./helpers');

function query(fixtureName, sql) {
  const instance = load(fixture(fixtureName));
  try {
    return instance.query(sql);
  } finally {
    instance.release();
  }
}

test('loads a valid file and returns an instance', () => {
  const instance = load(fixture('start.yaml'));
  assert.ok(instance instanceof Instance);
  instance.release();
});

test('a refusal carries the exact domain message', () => {
  assert.throws(
    () => load(fixture('missing-id.yaml')),
    (error) => error instanceof MemlessRefusal && error.message === 'row 1 in "users" has no id',
  );
});

test('a non-existent path is a refusal', () => {
  assert.throws(() => load(fixture('does-not-exist.yaml')), MemlessRefusal);
});

test('a NUL byte in the path is a fault', () => {
  assert.throws(() => load('has\x00nul'), MemlessFault);
});

test('release is idempotent', () => {
  const instance = load(fixture('start.yaml'));
  instance.release();
  instance.release();
});

test('a query after release faults with status 2', () => {
  const instance = load(fixture('start.yaml'));
  instance.release();
  assert.throws(
    () => instance.query('SELECT * FROM users'),
    (error) => error instanceof MemlessFault && error.status === 2,
  );
});

test('queries rows in file order', () => {
  const rows = query('start.yaml', 'SELECT name FROM users');
  assert.deepEqual(rows.columns, ['name']);
  assert.deepEqual(rows.rows, [['Ada'], ['Grace']]);
});

test('queries an aggregate', () => {
  assert.deepEqual(query('start.yaml', 'SELECT COUNT(*) FROM users').rows, [[2]]);
});

test('reads each cell kind', () => {
  const rows = query('kinds.yaml', 'SELECT ratio, active, label FROM things').rows;
  assert.deepEqual(rows[0], [1.5, true, 'hi']);
  assert.equal(rows[1][1], false);
  assert.equal(rows[1][2], null);
});

test('a refused query throws the domain message', () => {
  assert.throws(
    () => query('start.yaml', 'SELECT * FROM ghosts'),
    (error) => error instanceof MemlessRefusal && error.message === 'no table "ghosts"',
  );
});

test('an out-of-subset query is refused', () => {
  assert.throws(
    () => query('start.yaml', 'SELECT * FROM users LIMIT 1'),
    (error) => error instanceof MemlessRefusal
      && error.message === 'LIMIT is outside the supported SQL subset',
  );
});

test('a join qualifies headers and keeps column order', () => {
  const rows = query('start.yaml', 'SELECT * FROM wallets JOIN users ON wallets.user_id = users.id');
  assert.deepEqual(rows.columns, [
    'wallets.id', 'wallets.user_id', 'wallets.amount', 'users.id', 'users.name',
  ]);
  assert.equal(rows.rows.length, 2);
});

test('a NUL byte in the sql is a fault', () => {
  const instance = load(fixture('start.yaml'));
  try {
    assert.throws(() => instance.query('SELECT * FROM users\x00'), MemlessFault);
  } finally {
    instance.release();
  }
});
