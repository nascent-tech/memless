# darwin-x64

The release workflow places `libmemless_capi.dylib` here, in a commit tagged
`bindings/go/vX.Y.Z` that never lands on `main`. This file keeps the directory
in the repository so the module compiles without the library, which it then
looks for under `MEMLESS_LIB` or `target/`.
