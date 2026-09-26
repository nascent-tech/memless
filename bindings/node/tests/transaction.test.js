'use strict';

const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');

const { load, MemlessRefusal } = require('../src/index');
const { loadTemp } = require('./helpers');

test('a transaction reads its own writes and commits once to disk', () => {
  const { instance, dir, file } = loadTemp(load);
  try {
    const before = fs.readFileSync(file, 'utf8');
    instance.begin();
    assert.equal(instance.execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'"), 1);
    assert.deepEqual(instance.query("SELECT name FROM users WHERE id = '01H7B2'").rows, [['Zoe']]);
    assert.equal(fs.readFileSync(file, 'utf8'), before);
    instance.commit();
    const after = fs.readFileSync(file, 'utf8');
    assert.notEqual(after, before);
    assert.ok(after.includes('Zoe'));
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('a rollback leaves the file and restores the state', () => {
  const { instance, dir, file } = loadTemp(load);
  try {
    const before = fs.readFileSync(file, 'utf8');
    instance.begin();
    instance.execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'");
    instance.rollback();
    assert.equal(fs.readFileSync(file, 'utf8'), before);
    assert.deepEqual(instance.query("SELECT name FROM users WHERE id = '01H7B2'").rows, [['Ada']]);
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('a second begin is refused', () => {
  const { instance, dir } = loadTemp(load);
  try {
    instance.begin();
    assert.throws(
      () => instance.begin(),
      (error) => error instanceof MemlessRefusal && error.message === 'a transaction is already open',
    );
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('a commit without a transaction is refused', () => {
  const { instance, dir } = loadTemp(load);
  try {
    assert.throws(
      () => instance.commit(),
      (error) => error instanceof MemlessRefusal && error.message === 'no open transaction',
    );
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('a failed validation is refused and closes the transaction', () => {
  const { instance, dir, file } = loadTemp(load);
  try {
    const before = fs.readFileSync(file, 'utf8');
    instance.begin();
    instance.execute("INSERT INTO users (id, name) VALUES ('01H7B2', 'Dup')");
    assert.throws(() => instance.commit(), MemlessRefusal);
    assert.equal(fs.readFileSync(file, 'utf8'), before);
    instance.begin();
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
