'use strict';

const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');

const { load, MemlessRefusal, MemlessFault } = require('../src/index');
const { loadTemp } = require('./helpers');

test('executes a write and counts the rows', () => {
  const { instance, dir } = loadTemp(load);
  try {
    assert.equal(instance.execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'"), 1);
    assert.deepEqual(instance.query("SELECT name FROM users WHERE id = '01H7B2'").rows, [['Zoe']]);
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('a refused write throws the domain message', () => {
  const { instance, dir } = loadTemp(load);
  try {
    assert.throws(
      () => instance.execute('DELETE FROM ghosts'),
      (error) => error instanceof MemlessRefusal && error.message === 'no table "ghosts"',
    );
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('a SELECT through execute is refused', () => {
  const { instance, dir } = loadTemp(load);
  try {
    assert.throws(
      () => instance.execute('SELECT * FROM users'),
      (error) => error instanceof MemlessRefusal && error.message.includes('a SELECT in execute'),
    );
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('a NUL byte in the sql is a fault', () => {
  const { instance, dir } = loadTemp(load);
  try {
    assert.throws(() => instance.execute('DELETE FROM wallets\x00'), MemlessFault);
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('an execute after release faults with status 2', () => {
  const { instance, dir } = loadTemp(load);
  instance.release();
  try {
    assert.throws(
      () => instance.execute('DELETE FROM wallets'),
      (error) => error instanceof MemlessFault && error.status === 2,
    );
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('an integer beyond the safe range stays exact as a BigInt', () => {
  const { instance, dir } = loadTemp(load);
  try {
    instance.execute('UPDATE wallets SET amount = 9007199254740993 WHERE id = \'w_123\'');
    const rows = instance.query("SELECT amount FROM wallets WHERE id = 'w_123'").rows;
    assert.deepEqual(rows, [[9007199254740993n]]);
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('2^53 itself is already beyond the safe range and stays a BigInt', () => {
  const { instance, dir } = loadTemp(load);
  try {
    instance.execute('UPDATE wallets SET amount = 9007199254740992 WHERE id = \'w_123\'');
    const rows = instance.query("SELECT amount FROM wallets WHERE id = 'w_123'").rows;
    assert.deepEqual(rows, [[9007199254740992n]]);
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('a negative integer beyond the safe range stays exact as a BigInt', () => {
  const { instance, dir } = loadTemp(load);
  try {
    instance.execute('UPDATE wallets SET amount = -9007199254740993 WHERE id = \'w_123\'');
    const rows = instance.query("SELECT amount FROM wallets WHERE id = 'w_123'").rows;
    assert.deepEqual(rows, [[-9007199254740993n]]);
  } finally {
    instance.release();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('a disk failure is refused and leaves memory intact', () => {
  const { instance, dir } = loadTemp(load);
  try {
    fs.rmSync(dir, { recursive: true, force: true });
    assert.throws(
      () => instance.execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'"),
      (error) => error instanceof MemlessRefusal && error.message.includes('cannot write file'),
    );
    assert.deepEqual(instance.query("SELECT name FROM users WHERE id = '01H7B2'").rows, [['Ada']]);
  } finally {
    instance.release();
  }
});
