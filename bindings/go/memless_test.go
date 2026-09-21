package memless

import (
	"errors"
	"path/filepath"
	"sync"
	"testing"
)

const (
	testWorkers          = 8
	unknownHandle uint64 = 1 << 62
	badVersion    uint32 = 99
)

func fixture(name string) string {
	return filepath.Join(workspaceRoot(), "harness", "parity", "fixtures", name)
}

func TestLoadsAValidFileAndReturnsAnInstance(t *testing.T) {
	instance, err := Load(fixture("start.yaml"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if instance == nil {
		t.Fatal("expected an instance")
	}
	instance.Release()
}

func TestReturnsRefusalErrorWithTheExactDomainMessage(t *testing.T) {
	instance, err := Load(fixture("missing-id.yaml"))
	if instance != nil {
		t.Fatal("expected no instance on a refusal")
	}
	var refusal *RefusalError
	if !errors.As(err, &refusal) {
		t.Fatalf("expected a RefusalError, got %T", err)
	}
	if refusal.Message != `row 1 in "users" has no id` {
		t.Fatalf("unexpected message: %q", refusal.Message)
	}
}

func TestRefusesANonExistentPathAsARefusal(t *testing.T) {
	_, err := Load(fixture("does-not-exist.yaml"))
	var refusal *RefusalError
	if !errors.As(err, &refusal) {
		t.Fatalf("expected a RefusalError, got %T", err)
	}
}

func TestReturnsAFaultErrorOnABoundaryFault(t *testing.T) {
	instance, err := Load("\xffnot-utf-8")
	if instance != nil {
		t.Fatal("expected no instance on a boundary fault")
	}
	var fault *FaultError
	if !errors.As(err, &fault) {
		t.Fatalf("expected a FaultError, got %T", err)
	}
	if fault.Status != statusInvalidArgument {
		t.Fatalf("expected InvalidArgument, got %d", fault.Status)
	}
}

func TestReturnsAFaultErrorOnANulBytePath(t *testing.T) {
	_, err := Load("has\x00nul")
	var fault *FaultError
	if !errors.As(err, &fault) {
		t.Fatalf("expected a FaultError, got %T", err)
	}
}

func TestReleaseIgnoresAnUnknownHandle(t *testing.T) {
	if err := ensureLoaded(); err != nil {
		t.Fatalf("load library: %v", err)
	}
	instance := &Instance{handle: unknownHandle}
	instance.Release()
}

func TestReleaseIsIdempotent(t *testing.T) {
	instance, err := Load(fixture("start.yaml"))
	if err != nil {
		t.Fatalf("load: %v", err)
	}
	instance.Release()
	instance.Release()
}

func TestChecksTheAbiVersion(t *testing.T) {
	if err := checkVersion(abiVersion); err != nil {
		t.Fatalf("expected the pinned version to pass, got %v", err)
	}
	if checkVersion(badVersion) == nil {
		t.Fatal("expected a mismatched version to fail")
	}
}

func TestParallelLoadsAllocateSafely(t *testing.T) {
	var group sync.WaitGroup
	group.Add(testWorkers)
	for i := 0; i < testWorkers; i++ {
		go loadAndRelease(t, &group)
	}
	group.Wait()
}

func loadAndRelease(t *testing.T, group *sync.WaitGroup) {
	defer group.Done()
	instance, err := Load(fixture("start.yaml"))
	if err != nil {
		t.Errorf("parallel load: %v", err)
		return
	}
	instance.Release()
}
