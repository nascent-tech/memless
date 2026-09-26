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

function installedPlatformPackage(name, file) {
  const project = tempDir();
  const dir = path.join(project, 'node_modules', '@nascent-tech', `memless-${name}`);
  writeFile(path.join(dir, 'package.json'), JSON.stringify({ name: `@nascent-tech/memless-${name}` }));
  const library = writeFile(path.join(dir, file), 'bundled');
  const resolve = (request) => require.resolve(request, { paths: [project] });
  return { library, resolve };
}

test('MEMLESS_LIB wins over the bundled library and target/', () => {
  const { root, release } = workspaceWithRelease();
  const got = resolveLibrary({ env: release, embedded: () => '/bundled/libmemless_capi.dylib', root });
  assert.equal(got, release);
});

test('the bundled library comes before target/', () => {
  const { root } = workspaceWithRelease();
  const { library, resolve } = installedPlatformPackage('darwin-arm64', 'libmemless_capi.dylib');
  const embedded = () => embeddedLibrary(platformFor('darwin', 'arm64', null), resolve);
  assert.equal(resolveLibrary({ env: undefined, embedded, root }), library);
});

test('the bundled library is the file next to the platform package manifest', () => {
  const { library, resolve } = installedPlatformPackage('linux-x64-gnu', 'libmemless_capi.so');
  assert.equal(embeddedLibrary(platformFor('linux', 'x64', 'glibc'), resolve), library);
});

test('a platform with no platform package falls back to target/', () => {
  const { root, release } = workspaceWithRelease();
  const embedded = () => embeddedLibrary(platformFor('win32', 'x64', null), require.resolve);
  assert.equal(resolveLibrary({ env: undefined, embedded, root }), release);
});

test('a platform package npm did not install falls back to target/', () => {
  const { root, release } = workspaceWithRelease();
  const { resolve } = installedPlatformPackage('darwin-x64', 'libmemless_capi.dylib');
  const embedded = () => embeddedLibrary(platformFor('darwin', 'arm64', null), resolve);
  assert.equal(resolveLibrary({ env: undefined, embedded, root }), release);
});

test('nothing anywhere fails with a message naming MEMLESS_LIB', () => {
  const embedded = () => null;
  assert.throws(() => resolveLibrary({ env: undefined, embedded, root: tempDir() }), /set MEMLESS_LIB/);
});

test('the four published platforms map to their packages, the others to none', () => {
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

test('an unknown libc is null, which leaves no platform package', () => {
  assert.equal(detectLibc({ ldd: () => 'something else', report: () => ({ header: {} }) }), null);
  assert.equal(detectLibc({ ldd: () => null, report: () => null }), null);
});
