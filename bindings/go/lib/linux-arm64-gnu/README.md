# linux-arm64-gnu

The release workflow places `libmemless_capi.so` here, in the tree it
publishes to the mirror repository `nascent-tech/memless-go` (where this file
is dropped). This file keeps the directory in this repository so the module
compiles without the library, which it then looks for under `MEMLESS_LIB` or
`target/`.
