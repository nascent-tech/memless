---
title: Platforms and troubleshooting
description: Which platforms are covered, how the native library is found, and what to do when it is not.
sidebar:
  order: 3
---

The packages include the engine for macOS (Apple silicon, Intel) and for Linux
with glibc 2.39 or later (x86_64, aarch64), such as Ubuntu 24.04, Debian 13 or
Fedora 40 and later.

## How the library is found

Each bridge finds the native library on its own, in this order:

1. the file named by the `MEMLESS_LIB` environment variable, if it is set — it
   must exist, there is no fallback;
2. the library bundled in the package for your platform;
3. inside a clone of the repository, `target/release/`, then `target/debug/`.

:::caution
`MEMLESS_LIB` loads native code into your process: only point it at a library
you trust.
:::

## Your platform is not covered

On Alpine or another musl-based Linux, an older glibc, or a platform whose libc
Memless cannot identify, build the library yourself and point `MEMLESS_LIB` at
it:

```sh
git clone https://github.com/nascent-tech/memless.git
cd memless
cargo build --release -p memless-capi   # needs a Rust toolchain
export MEMLESS_LIB="$PWD/target/release/libmemless_capi.so"   # .dylib on macOS
```

You can also download a prebuilt library,
`memless-capi-<version>-<target>.tar.gz`, from the
[GitHub releases](https://github.com/nascent-tech/memless/releases), with its
checksums in `SHA256SUMS`.

## PHP says the `FFI` class does not exist

Enable the extension with `extension=ffi` in `php.ini`. On the command line,
FFI is allowed by default (`ffi.enable=preload` covers the CLI); for a web
server, see the
[PHP guide](https://github.com/nascent-tech/memless/blob/main/bindings/php/README.md).
