package memless

import (
	"strings"
	"testing"
)

func TestLibraryPathRejectsAMissingMemlessLib(t *testing.T) {
	t.Setenv("MEMLESS_LIB", "/nonexistent/path/lib.dylib")
	_, err := libraryPath()
	if err == nil {
		t.Fatal("expected an error for a missing MEMLESS_LIB file")
	}
	if !strings.Contains(err.Error(), "/nonexistent/path/lib.dylib") {
		t.Fatalf("expected the path in the message, got %q", err.Error())
	}
}

func TestLibraryPathAcceptsAnExistingMemlessLib(t *testing.T) {
	real, err := libraryPath()
	if err != nil {
		t.Fatalf("resolve the real cdylib: %v", err)
	}
	t.Setenv("MEMLESS_LIB", real)
	got, err := libraryPath()
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if got != real {
		t.Fatalf("got %q, want %q", got, real)
	}
}
