package memless

import (
	"errors"
	"os"
	"path/filepath"
	"reflect"
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

func queryRows(t *testing.T, fixtureName, sql string) Rows {
	t.Helper()
	instance, err := Load(fixture(fixtureName))
	if err != nil {
		t.Fatalf("load %s: %v", fixtureName, err)
	}
	defer instance.Release()
	rows, err := instance.Query(sql)
	if err != nil {
		t.Fatalf("query %q: %v", sql, err)
	}
	return rows
}

func TestQueriesRowsInFileOrder(t *testing.T) {
	rows := queryRows(t, "start.yaml", "SELECT name FROM users")
	want := [][]any{{"Ada"}, {"Grace"}}
	if !reflect.DeepEqual(rows.Columns, []string{"name"}) {
		t.Fatalf("columns: got %v", rows.Columns)
	}
	if !reflect.DeepEqual(rows.Rows, want) {
		t.Fatalf("rows: got %v, want %v", rows.Rows, want)
	}
}

func TestQueriesAnAggregate(t *testing.T) {
	rows := queryRows(t, "start.yaml", "SELECT COUNT(*) FROM users")
	if !reflect.DeepEqual(rows.Rows, [][]any{{int64(2)}}) {
		t.Fatalf("rows: got %v", rows.Rows)
	}
}

func TestQueriesEachCellKind(t *testing.T) {
	rows := queryRows(t, "kinds.yaml", "SELECT ratio, active, label FROM things")
	if rows.Rows[0][0] != 1.5 || rows.Rows[0][1] != true || rows.Rows[0][2] != "hi" {
		t.Fatalf("row 0: got %v", rows.Rows[0])
	}
	if rows.Rows[1][1] != false || rows.Rows[1][2] != nil {
		t.Fatalf("row 1: got %v", rows.Rows[1])
	}
}

func TestTheTrapJoinsTheSameTypeIdOnly(t *testing.T) {
	path := filepath.Join(t.TempDir(), "trap.yaml")
	content := "users:\n  - id: 5\n  - id: \"5\"\nwallets:\n  - id: w1\n    user_id: 5\n"
	if err := os.WriteFile(path, []byte(content), 0o600); err != nil {
		t.Fatalf("write fixture: %v", err)
	}
	instance, err := Load(path)
	if err != nil {
		t.Fatalf("load: %v", err)
	}
	defer instance.Release()
	rows, err := instance.Query("SELECT users.id FROM wallets JOIN users ON wallets.user_id = users.id")
	if err != nil {
		t.Fatalf("query: %v", err)
	}
	if !reflect.DeepEqual(rows.Rows, [][]any{{int64(5)}}) {
		t.Fatalf("rows: got %v", rows.Rows)
	}
}

func TestReturnsRefusalErrorFromAQuery(t *testing.T) {
	instance, err := Load(fixture("start.yaml"))
	if err != nil {
		t.Fatalf("load: %v", err)
	}
	defer instance.Release()
	_, err = instance.Query("SELECT * FROM ghosts")
	var refusal *RefusalError
	if !errors.As(err, &refusal) {
		t.Fatalf("expected a RefusalError, got %T", err)
	}
	if refusal.Message != `no table "ghosts"` {
		t.Fatalf("message: got %q", refusal.Message)
	}
}

func TestFiltersUnderTheOneComparisonRule(t *testing.T) {
	rows := queryRows(t, "start.yaml", "SELECT name FROM users WHERE id = '01H7B2'")
	if !reflect.DeepEqual(rows.Rows, [][]any{{"Ada"}}) {
		t.Fatalf("rows: got %v", rows.Rows)
	}
	empty := queryRows(t, "start.yaml", "SELECT name FROM users WHERE id = 999")
	if len(empty.Rows) != 0 {
		t.Fatalf("expected no rows, got %v", empty.Rows)
	}
}

func TestQueryAfterReleaseFaults(t *testing.T) {
	instance, err := Load(fixture("start.yaml"))
	if err != nil {
		t.Fatalf("load: %v", err)
	}
	instance.Release()
	_, err = instance.Query("SELECT * FROM users")
	var fault *FaultError
	if !errors.As(err, &fault) {
		t.Fatalf("expected a FaultError, got %T", err)
	}
	if fault.Status != statusInvalidArgument {
		t.Fatalf("status: got %d", fault.Status)
	}
}

func TestJoinQualifiesHeadersAndKeepsColumnOrder(t *testing.T) {
	rows := queryRows(t, "start.yaml", "SELECT * FROM wallets JOIN users ON wallets.user_id = users.id")
	want := []string{"wallets.id", "wallets.user_id", "wallets.amount", "users.id", "users.name"}
	if !reflect.DeepEqual(rows.Columns, want) {
		t.Fatalf("columns: got %v, want %v", rows.Columns, want)
	}
	if len(rows.Rows) != 2 {
		t.Fatalf("rows: got %d", len(rows.Rows))
	}
}

func TestRejectsANulByteInSql(t *testing.T) {
	instance, err := Load(fixture("start.yaml"))
	if err != nil {
		t.Fatalf("load: %v", err)
	}
	defer instance.Release()
	_, err = instance.Query("SELECT * FROM users\x00")
	var fault *FaultError
	if !errors.As(err, &fault) {
		t.Fatalf("expected a FaultError, got %T", err)
	}
}

func TestReturnsRefusalErrorForAnOutOfSubsetQuery(t *testing.T) {
	instance, err := Load(fixture("start.yaml"))
	if err != nil {
		t.Fatalf("load: %v", err)
	}
	defer instance.Release()
	_, err = instance.Query("SELECT * FROM users LIMIT 1")
	var refusal *RefusalError
	if !errors.As(err, &refusal) {
		t.Fatalf("expected a RefusalError, got %T", err)
	}
	if refusal.Message != "LIMIT is outside the supported SQL subset" {
		t.Fatalf("message: got %q", refusal.Message)
	}
}
