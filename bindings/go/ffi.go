package memless

import (
	"fmt"
	"sync"

	"github.com/ebitengine/purego"
)

var (
	loadOnce          sync.Once
	loadErr           error
	memlessAbiVersion func() uint32
	memlessLoad       func(path string, outHandle *uint64, outMessage **byte) int32
	memlessRelease    func(handle uint64)
	memlessFreeString func(message *byte)
)

func ensureLoaded() error {
	loadOnce.Do(func() {
		loadErr = openLibrary()
	})
	return loadErr
}

func openLibrary() (err error) {
	defer func() { err = recovered(recover(), err) }()
	path, err := libraryPath()
	if err != nil {
		return err
	}
	handle, err := purego.Dlopen(path, purego.RTLD_NOW|purego.RTLD_LOCAL)
	if err != nil {
		return err
	}
	registerFunctions(handle)
	return checkAbi()
}

// recovered turns a purego RegisterLibFunc panic (a missing symbol) into an
// error instead of crashing Load.
func recovered(panicked any, err error) error {
	if panicked != nil {
		return fmt.Errorf("memless library load failed: %v", panicked)
	}
	return err
}

func registerFunctions(handle uintptr) {
	purego.RegisterLibFunc(&memlessAbiVersion, handle, "memless_abi_version")
	purego.RegisterLibFunc(&memlessLoad, handle, "memless_load")
	purego.RegisterLibFunc(&memlessRelease, handle, "memless_release")
	purego.RegisterLibFunc(&memlessFreeString, handle, "memless_free_string")
}

func checkAbi() error {
	return checkVersion(memlessAbiVersion())
}

func checkVersion(version uint32) error {
	if version != abiVersion {
		return fmt.Errorf("memless ABI mismatch: expected %d, got %d", abiVersion, version)
	}
	return nil
}
