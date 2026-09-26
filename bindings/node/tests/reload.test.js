'use strict';

const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');

const { load, MemlessRefusal, MemlessFault } = require('../src/index');
const { loadTemp } = require('./helpers');

test('a reload picks up an external edit of the file', () => {
  const { instance, dir, file } = loadTemp(load);
  try {
    const edited = fs.readFileSync(file, 'utf8').replace('name: Ada', 'name: Zoe');
    fs.writeFileSync(file, edited);
    instance.reload();
    assert.deepEqual(instance.query("SELECT name FROM users WHERE id = '01H7B2'").rows, [['Zoe']]);
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('a reload during an open transaction is refused and the transaction stays usable', () => {
  const { instance, dir } = loadTemp(load);
  try {
    instance.begin();
    assert.throws(
      () => instance.reload(),
      (error) => error instanceof MemlessRefusal
        && error.message === 'cannot reload while a transaction is open',
    );
    assert.equal(instance.execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'"), 1);
    instance.commit();
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('a reload of an incoherent file is refused and keeps the old state', () => {
  const { instance, dir, file } = loadTemp(load);
  try {
    fs.writeFileSync(file, 'users:\n  - name: Ada\n');
    assert.throws(() => instance.reload(), MemlessRefusal);
    assert.deepEqual(instance.query("SELECT name FROM users WHERE id = '01H7B2'").rows, [['Ada']]);
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('a reload of a missing file is refused naming the path and keeps the old state', () => {
  const { instance, dir, file } = loadTemp(load);
  try {
    fs.rmSync(file);
    assert.throws(
      () => instance.reload(),
      (error) => error instanceof MemlessRefusal && error.message.startsWith('no file at path'),
    );
    assert.deepEqual(instance.query("SELECT name FROM users WHERE id = '01H7B2'").rows, [['Ada']]);
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('a reload on a released instance is a fault', () => {
  const { instance, dir } = loadTemp(load);
  instance.release();
  try {
    assert.throws(
      () => instance.reload(),
      (error) => error instanceof MemlessFault && error.status === 2,
    );
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
