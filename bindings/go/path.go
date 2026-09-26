package memless

import (
	"errors"
	"os"
	"path/filepath"
	"runtime"
)

func libraryPath() (string, error) {
	if env := os.Getenv("MEMLESS_LIB"); env != "" {
		return envLibrary(env)
	}
	return searchLibrary()
}

func envLibrary(env string) (string, error) {
	if !exists(env) {
		return "", errors.New("memless cdylib not found at " + env + "; set MEMLESS_LIB")
	}
	return env, nil
}

func searchLibrary() (string, error) {
	found := existingLibrary(workspaceRoot())
	if found == "" {
		return "", errors.New("memless cdylib not found; set MEMLESS_LIB")
	}
	return found, nil
}

func existingLibrary(root string) string {
	found := ""
	for _, name := range libraryNames() {
		found = orExisting(found, filepath.Join(root, name))
	}
	return found
}

func orExisting(found, candidate string) string {
	if found != "" {
		return found
	}
	if !exists(candidate) {
		return ""
	}
	return candidate
}

func exists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

// Same order as the PHP and Node bridges: release before debug, .dylib
// before .so.
func libraryNames() []string {
	return []string{
		"target/release/libmemless_capi.dylib",
		"target/debug/libmemless_capi.dylib",
		"target/release/libmemless_capi.so",
		"target/debug/libmemless_capi.so",
	}
}

func workspaceRoot() string {
	_, file, _, _ := runtime.Caller(0)
	return filepath.Join(filepath.Dir(file), "..", "..")
}
