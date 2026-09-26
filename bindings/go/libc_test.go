package memless

import (
	"errors"
	"testing"
)

func lddSays(text string) func(string) ([]byte, error) {
	return func(string) ([]byte, error) { return []byte(text), nil }
}

func noLdd(string) ([]byte, error) { return nil, errors.New("no ldd") }

func only(loaders ...string) func(string) bool {
	return func(path string) bool {
		for _, loader := range loaders {
			if loader == path {
				return true
			}
		}
		return false
	}
}

func TestLddTellsMuslFromGlibcBeforeTheLoaders(t *testing.T) {
	loaders := func(string) bool { t.Fatal("the loaders must not be probed"); return false }
	if got := detectLibc(lddSays("musl libc (x86_64)"), loaders); got != "musl" {
		t.Fatalf("got %q, want musl", got)
	}
	if got := detectLibc(lddSays("# GNU C Library"), loaders); got != "glibc" {
		t.Fatalf("got %q, want glibc", got)
	}
}

func TestWithoutLddTheDynamicLoaderTellsMuslFromGlibc(t *testing.T) {
	cases := map[string]string{
		"/lib/ld-musl-x86_64.so.1":    "musl",
		"/lib/ld-musl-aarch64.so.1":   "musl",
		"/lib64/ld-linux-x86-64.so.2": "glibc",
		"/lib/ld-linux-aarch64.so.1":  "glibc",
	}
	for loader, want := range cases {
		if got := detectLibc(noLdd, only(loader)); got != want {
			t.Fatalf("%s: got %q, want %q", loader, got, want)
		}
	}
}

func TestTheMuslLoaderWinsOverAGlibcLoaderAsWithGcompatOnAlpine(t *testing.T) {
	both := only("/lib/ld-musl-x86_64.so.1", "/lib64/ld-linux-x86-64.so.2")
	if got := detectLibc(noLdd, both); got != "musl" {
		t.Fatalf("got %q, want musl", got)
	}
}

func TestAnUnknownLibcIsEmpty(t *testing.T) {
	if got := detectLibc(lddSays("something else"), only()); got != "" {
		t.Fatalf("got %q, want an unknown libc", got)
	}
	if got := detectLibc(noLdd, only()); got != "" {
		t.Fatalf("got %q, want an unknown libc", got)
	}
}

func TestALinuxWhoseLibcCannotBeToldHasNoBundledLibrary(t *testing.T) {
	for _, libc := range []string{"", "musl"} {
		if got := bundleFor("linux", libc); got.files != nil || got.file != "" {
			t.Fatalf("linux with libc %q: got a bundled library %q", libc, got.file)
		}
	}
	if got := bundleFor("linux", "glibc"); got.file != embeddedFile {
		t.Fatalf("linux with glibc: got %q, want %q", got.file, embeddedFile)
	}
	if got := bundleFor("darwin", ""); got.file != embeddedFile {
		t.Fatalf("darwin: got %q, want %q", got.file, embeddedFile)
	}
}
