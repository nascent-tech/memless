package memless

import (
	"errors"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
)

const startYAML = "users:\n  - id: 01H7B2\n    name: Ada\n  - id: 01H7B3\n    name: Grace\n" +
	"wallets:\n  - id: w_123\n    user_id: 01H7B3\n    amount: 100\n" +
	"  - id: w_124\n    user_id: 01H7B2\n    amount: 250\n"

func loadTemp(t *testing.T) (*Instance, string) {
	t.Helper()
	dir := t.TempDir()
	path := filepath.Join(dir, "start.yaml")
	if err := os.WriteFile(path, []byte(startYAML), 0o600); err != nil {
		t.Fatalf("write fixture: %v", err)
	}
	instance, err := Load(path)
	if err != nil {
		t.Fatalf("load: %v", err)
	}
	return instance, dir
}

func TestExecutesAWriteAndCountsTheRows(t *testing.T) {
	instance, _ := loadTemp(t)
	defer instance.Release()
	affected, err := instance.Execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'")
	if err != nil {
		t.Fatalf("execute: %v", err)
	}
	if affected != 1 {
		t.Fatalf("affected: got %d, want 1", affected)
	}
	rows, err := instance.Query("SELECT name FROM users WHERE id = '01H7B2'")
	if err != nil {
		t.Fatalf("query: %v", err)
	}
	if !reflect.DeepEqual(rows.Rows, [][]any{{"Zoe"}}) {
		t.Fatalf("rows: got %v", rows.Rows)
	}
}

func TestExecuteRefusesAWriteWithTheDomainMessage(t *testing.T) {
	instance, _ := loadTemp(t)
	defer instance.Release()
	_, err := instance.Execute("DELETE FROM ghosts")
	var refusal *RefusalError
	if !errors.As(err, &refusal) {
		t.Fatalf("expected a RefusalError, got %T", err)
	}
	if refusal.Message != `no table "ghosts"` {
		t.Fatalf("message: got %q", refusal.Message)
	}
}

func TestExecuteRefusesASelect(t *testing.T) {
	instance, _ := loadTemp(t)
	defer instance.Release()
	_, err := instance.Execute("SELECT * FROM users")
	var refusal *RefusalError
	if !errors.As(err, &refusal) {
		t.Fatalf("expected a RefusalError, got %T", err)
	}
	if !strings.Contains(refusal.Message, "a SELECT in execute") {
		t.Fatalf("message: got %q", refusal.Message)
	}
}

func TestExecuteAfterReleaseFaults(t *testing.T) {
	instance, _ := loadTemp(t)
	instance.Release()
	_, err := instance.Execute("DELETE FROM wallets")
	var fault *FaultError
	if !errors.As(err, &fault) {
		t.Fatalf("expected a FaultError, got %T", err)
	}
	if fault.Status != statusInvalidArgument {
		t.Fatalf("status: got %d", fault.Status)
	}
}

func TestExecuteRejectsANulByteInSql(t *testing.T) {
	instance, _ := loadTemp(t)
	defer instance.Release()
	_, err := instance.Execute("DELETE FROM wallets\x00")
	var fault *FaultError
	if !errors.As(err, &fault) {
		t.Fatalf("expected a FaultError, got %T", err)
	}
}

func TestExecuteDiskFailureIsRefusedAndLeavesMemory(t *testing.T) {
	instance, dir := loadTemp(t)
	defer instance.Release()
	if err := os.RemoveAll(dir); err != nil {
		t.Fatalf("remove workdir: %v", err)
	}
	_, err := instance.Execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'")
	var refusal *RefusalError
	if !errors.As(err, &refusal) {
		t.Fatalf("expected a RefusalError, got %T", err)
	}
	if !strings.Contains(refusal.Message, "cannot write file") {
		t.Fatalf("message: got %q", refusal.Message)
	}
	rows, err := instance.Query("SELECT name FROM users WHERE id = '01H7B2'")
	if err != nil {
		t.Fatalf("query: %v", err)
	}
	if !reflect.DeepEqual(rows.Rows, [][]any{{"Ada"}}) {
		t.Fatalf("rows: got %v", rows.Rows)
	}
}
