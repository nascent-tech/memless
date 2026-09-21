// Package memless bridges the memless C ABI through purego, without cgo.
//
// The cdylib is found through MEMLESS_LIB, a trusted (ideally absolute) path,
// or, inside a checked-out workspace, under target/. MEMLESS_LIB loads
// arbitrary native code, like any FFI library path.
package memless
