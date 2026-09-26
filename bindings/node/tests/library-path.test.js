'use strict';

const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const resolveLibrary = require('../src/resolve-library');
const embeddedLibrary = require('../src/embedded-library');
const platformFor = require('../src/platform-for');
const detectLibc = require('../src/detect-libc');

function tempDir() {
  return fs.realpathSync(fs.mkdtempSync(path.join(os.tmpdir(), 'memless-path-')));
}

function writeFile(file, content) {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, content);
  return file;
}

function workspaceWithRelease() {
  const root = tempDir();
  const release = writeFile(path.join(root, 'target', 'release', 'libmemless_capi.dylib'), 'release');
  return { root, release };
}

function bundledLibrary(name, file) {
  const lib = path.join(tempDir(), 'lib');
  const library = writeFile(path.join(lib, name, file), 'bundled');
  return { library, lib };
}

test('MEMLESS_LIB wins over the bundled library and target/', () => {
  const { root, release } = workspaceWithRelease();
  const got = resolveLibrary({ env: release, embedded: () => '/bundled/libmemless_capi.dylib', root });
  assert.equal(got, release);
});

test('the bundled library comes before target/', () => {
  const { root } = workspaceWithRelease();
  const { library, lib } = bundledLibrary('darwin-arm64', 'libmemless_capi.dylib');
  const embedded = () => embeddedLibrary(platformFor('darwin', 'arm64', null), lib);
  assert.equal(resolveLibrary({ env: undefined, embedded, root }), library);
});

test('the bundled library is the file under lib/ named after the platform', () => {
  const { library, lib } = bundledLibrary('linux-x64-gnu', 'libmemless_capi.so');
  assert.equal(embeddedLibrary(platformFor('linux', 'x64', 'glibc'), lib), library);
});

test('a platform with nothing bundled falls back to target/', () => {
  const { root, release } = workspaceWithRelease();
  const { lib } = bundledLibrary('darwin-arm64', 'libmemless_capi.dylib');
  const embedded = () => embeddedLibrary(platformFor('win32', 'x64', null), lib);
  assert.equal(resolveLibrary({ env: undefined, embedded, root }), release);
});

test('a lib/ without the current platform falls back to target/', () => {
  const { root, release } = workspaceWithRelease();
  const { lib } = bundledLibrary('darwin-x64', 'libmemless_capi.dylib');
  const embedded = () => embeddedLibrary(platformFor('darwin', 'arm64', null), lib);
  assert.equal(resolveLibrary({ env: undefined, embedded, root }), release);
});

test('an empty lib/, as in a checked-out workspace, falls back to target/', () => {
  const { root, release } = workspaceWithRelease();
  const embedded = () => embeddedLibrary(platformFor('darwin', 'arm64', null), path.join(tempDir(), 'lib'));
  assert.equal(resolveLibrary({ env: undefined, embedded, root }), release);
});

test('nothing anywhere fails with a message naming MEMLESS_LIB', () => {
  const embedded = () => null;
  assert.throws(() => resolveLibrary({ env: undefined, embedded, root: tempDir() }), /set MEMLESS_LIB/);
});

test('the four bundled platforms map to their lib/ directories, the others to none', () => {
  assert.equal(platformFor('darwin', 'arm64', null).name, 'darwin-arm64');
  assert.equal(platformFor('darwin', 'x64', null).name, 'darwin-x64');
  assert.equal(platformFor('linux', 'x64', 'glibc').name, 'linux-x64-gnu');
  assert.equal(platformFor('linux', 'arm64', 'glibc').name, 'linux-arm64-gnu');
  assert.equal(platformFor('linux', 'x64', 'musl'), null);
  assert.equal(platformFor('linux', 'x64', null), null);
  assert.equal(platformFor('win32', 'x64', null), null);
});

test('ldd tells musl from glibc before the report is read', () => {
  const report = () => assert.fail('the report must not be read');
  assert.equal(detectLibc({ ldd: () => 'musl libc (x86_64)', report }), 'musl');
  assert.equal(detectLibc({ ldd: () => '# GNU C Library', report }), 'glibc');
});

test('without ldd, the report tells musl from glibc', () => {
  const glibc = { header: { glibcVersionRuntime: '2.39' }, sharedObjects: [] };
  const musl = { header: {}, sharedObjects: ['/lib/ld-musl-x86_64.so.1'] };
  assert.equal(detectLibc({ ldd: () => null, report: () => glibc }), 'glibc');
  assert.equal(detectLibc({ ldd: () => null, report: () => musl }), 'musl');
});

test('without ldd nor report, the dynamic loader tells musl from glibc', () => {
  const only = (file) => (candidate) => candidate === file;
  const probe = (exists) => ({ ldd: () => null, report: () => null, exists });
  assert.equal(detectLibc(probe(only('/lib/ld-musl-x86_64.so.1'))), 'musl');
  assert.equal(detectLibc(probe(only('/lib/ld-musl-aarch64.so.1'))), 'musl');
  assert.equal(detectLibc(probe(only('/lib64/ld-linux-x86-64.so.2'))), 'glibc');
  assert.equal(detectLibc(probe(only('/lib/ld-linux-aarch64.so.1'))), 'glibc');
});

test('the musl loader wins over a glibc loader, as with gcompat on Alpine', () => {
  const both = (file) => file === '/lib/ld-musl-x86_64.so.1' || file === '/lib64/ld-linux-x86-64.so.2';
  assert.equal(detectLibc({ ldd: () => null, report: () => null, exists: both }), 'musl');
});

test('an unknown libc is null, which leaves no bundled library', () => {
  const nothing = () => false;
  assert.equal(detectLibc({ ldd: () => 'something else', report: () => ({ header: {} }), exists: nothing }), null);
  assert.equal(detectLibc({ ldd: () => null, report: () => null, exists: nothing }), null);
  assert.equal(platformFor('linux', 'x64', null), null);
});
