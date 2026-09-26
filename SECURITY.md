# Security policy

## Supported versions

memless is in its `0.x` series. A security fix ships as a new version on npm,
Packagist and the Go module proxy at once, since the three bridges share one
native library; upgrade to it.

## Reporting a vulnerability

Please **do not open a public issue**. Report it privately through GitHub:
[**Report a vulnerability**](https://github.com/nascent-tech/memless/security/advisories/new)
(the *Security* tab of this repository). Include the version, the platform,
the bridge, and the smallest YAML file and SQL statement that show the
problem.

You will get an answer in that private advisory. The advisory is normally
published once the fix is out.

## What is in scope

memless loads a native library into your process and reads a YAML file you
give it, so the most important reports are about memory safety at the C ABI
boundary, a crash or hang triggered by a file or a statement, a write that
corrupts or leaks data outside the file, and the integrity of the published
packages (npm, Packagist, the Go module, the GitHub release).

By design, and not vulnerabilities:

- `MEMLESS_LIB` loads whatever native library it names, like any FFI library
  path; only point it at a library you trust;
- memless is meant for test fixtures and demos in a single process, not for
  untrusted input in production (see
  [What Memless is not](README.md#what-memless-is-not)).
