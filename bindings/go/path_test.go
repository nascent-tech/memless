package memless

import (
	"bytes"
	"os"
	"path/filepath"
	"runtime/debug"
	"strings"
	"testing"
	"testing/fstest"
)

const bundledFile = "lib/test-platform/libmemless_capi.dylib"

var bundledBytes = []byte("bundled library bytes")

func bundled() embeddedLibrary {
	files := fstest.MapFS{bundledFile: &fstest.MapFile{Data: bundledBytes}}
	return embeddedLibrary{files: files, file: bundledFile, version: "v9.9.9"}
}

func workspaceWithRelease(t *testing.T) (string, string) {
	t.Helper()
	root := t.TempDir()
	release := filepath.Join(root, "target", "release", "libmemless_capi.dylib")
	if err := os.MkdirAll(filepath.Dir(release), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(release, []byte("release"), 0o644); err != nil {
		t.Fatal(err)
	}
	return root, release
}

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

func TestMemlessLibWinsOverTheBundledLibrary(t *testing.T) {
	root, release := workspaceWithRelease(t)
	plan := libraryPlan{env: release, embedded: bundled(), cacheDir: t.TempDir(), root: root}
	got, err := resolveLibrary(plan)
	if err != nil || got != release {
		t.Fatalf("got %q, %v; want MEMLESS_LIB %q", got, err, release)
	}
}

func TestTheBundledLibraryComesBeforeTarget(t *testing.T) {
	root, release := workspaceWithRelease(t)
	cache := t.TempDir()
	got, err := resolveLibrary(libraryPlan{embedded: bundled(), cacheDir: cache, root: root})
	if err != nil || got == release || !strings.HasPrefix(got, cache) {
		t.Fatalf("got %q, %v; want a copy under %q", got, err, cache)
	}
	data, err := os.ReadFile(got)
	if err != nil || !bytes.Equal(data, bundledBytes) {
		t.Fatalf("extracted copy differs from the bundled bytes: %q, %v", data, err)
	}
}

func TestTheExtractedLibraryIsExecutableInAPrivateDirectory(t *testing.T) {
	got, err := resolveLibrary(libraryPlan{embedded: bundled(), cacheDir: t.TempDir(), root: t.TempDir()})
	if err != nil {
		t.Fatal(err)
	}
	file, _ := os.Stat(got)
	dir, _ := os.Stat(filepath.Dir(got))
	if file.Mode().Perm() != 0o755 || dir.Mode().Perm() != 0o700 {
		t.Fatalf("got file %v, dir %v; want 0755 and 0700", file.Mode().Perm(), dir.Mode().Perm())
	}
	if !strings.HasPrefix(filepath.Base(filepath.Dir(got)), "v9.9.9-") {
		t.Fatalf("expected a <version>-<sha256> directory, got %q", got)
	}
}

func TestATamperedExtractedLibraryIsRewrittenBeforeLoading(t *testing.T) {
	plan := libraryPlan{embedded: bundled(), cacheDir: t.TempDir(), root: t.TempDir()}
	first, err := resolveLibrary(plan)
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(first, []byte("tampered"), 0o755); err != nil {
		t.Fatal(err)
	}
	second, err := resolveLibrary(plan)
	if err != nil || second != first {
		t.Fatalf("got %q, %v; want the same path %q", second, err, first)
	}
	data, _ := os.ReadFile(second)
	if !bytes.Equal(data, bundledBytes) {
		t.Fatalf("tampered copy was not rewritten: %q", data)
	}
}

func TestAPlatformWithoutABundledLibraryFallsBackToTarget(t *testing.T) {
	root, release := workspaceWithRelease(t)
	none := embeddedLibrary{files: fstest.MapFS{}, file: ""}
	got, err := resolveLibrary(libraryPlan{embedded: none, cacheDir: t.TempDir(), root: root})
	if err != nil || got != release {
		t.Fatalf("got %q, %v; want %q", got, err, release)
	}
}

func TestABundleMissingItsLibraryFallsBackToTarget(t *testing.T) {
	root, release := workspaceWithRelease(t)
	readmeOnly := fstest.MapFS{"lib/test-platform/README.md": &fstest.MapFile{Data: []byte("#")}}
	lib := embeddedLibrary{files: readmeOnly, file: bundledFile}
	got, err := resolveLibrary(libraryPlan{embedded: lib, cacheDir: t.TempDir(), root: root})
	if err != nil || got != release {
		t.Fatalf("got %q, %v; want %q", got, err, release)
	}
}

func TestAnUnavailableCacheFallsBackToTarget(t *testing.T) {
	root, release := workspaceWithRelease(t)
	got, err := resolveLibrary(libraryPlan{embedded: bundled(), cacheDir: "", root: root})
	if err != nil || got != release {
		t.Fatalf("got %q, %v; want %q", got, err, release)
	}
}

func TestNoLibraryAnywhereNamesMemlessLib(t *testing.T) {
	_, err := resolveLibrary(libraryPlan{embedded: embeddedLibrary{}, root: t.TempDir()})
	if err == nil || !strings.Contains(err.Error(), "set MEMLESS_LIB") {
		t.Fatalf("expected an error naming MEMLESS_LIB, got %v", err)
	}
}

func TestTheCacheDirectoryIsNamedAfterTheModuleVersion(t *testing.T) {
	modules := []*debug.Module{{Path: "other", Version: "v1.0.0"}, {Path: modulePath, Version: "v0.2.0"}}
	if got := versionIn(modules); got != "v0.2.0" {
		t.Fatalf("got %q, want v0.2.0", got)
	}
	devel := []*debug.Module{{Path: modulePath, Version: "(devel)"}}
	if got := versionIn(devel); got != develVersion {
		t.Fatalf("got %q, want %q", got, develVersion)
	}
}
