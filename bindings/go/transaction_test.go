package memless

import (
	"errors"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
)

func readWorkingFile(t *testing.T, path string) string {
	t.Helper()
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("read: %v", err)
	}
	return string(data)
}

func TestATransactionReadsItsOwnWritesAndCommitsOnceToDisk(t *testing.T) {
	instance, dir := loadTemp(t)
	defer instance.Release()
	path := filepath.Join(dir, "start.yaml")
	before := readWorkingFile(t, path)
	if err := instance.Begin(); err != nil {
		t.Fatalf("begin: %v", err)
	}
	affected, err := instance.Execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'")
	if err != nil || affected != 1 {
		t.Fatalf("update: %d %v", affected, err)
	}
	rows, err := instance.Query("SELECT name FROM users WHERE id = '01H7B2'")
	if err != nil {
		t.Fatalf("query: %v", err)
	}
	if !reflect.DeepEqual(rows.Rows, [][]any{{"Zoe"}}) {
		t.Fatalf("read-your-writes: got %v", rows.Rows)
	}
	if readWorkingFile(t, path) != before {
		t.Fatal("intermediate write touched the disk")
	}
	if err := instance.Commit(); err != nil {
		t.Fatalf("commit: %v", err)
	}
	after := readWorkingFile(t, path)
	if after == before || !strings.Contains(after, "Zoe") {
		t.Fatalf("commit did not rewrite the file: %q", after)
	}
}

func TestARollbackLeavesTheFileAndRestoresTheState(t *testing.T) {
	instance, dir := loadTemp(t)
	defer instance.Release()
	path := filepath.Join(dir, "start.yaml")
	before := readWorkingFile(t, path)
	if err := instance.Begin(); err != nil {
		t.Fatalf("begin: %v", err)
	}
	if _, err := instance.Execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'"); err != nil {
		t.Fatalf("update: %v", err)
	}
	if err := instance.Rollback(); err != nil {
		t.Fatalf("rollback: %v", err)
	}
	if readWorkingFile(t, path) != before {
		t.Fatal("rollback touched the disk")
	}
	rows, err := instance.Query("SELECT name FROM users WHERE id = '01H7B2'")
	if err != nil {
		t.Fatalf("query: %v", err)
	}
	if !reflect.DeepEqual(rows.Rows, [][]any{{"Ada"}}) {
		t.Fatalf("state not restored: got %v", rows.Rows)
	}
}

func TestASecondBeginIsRefused(t *testing.T) {
	instance, _ := loadTemp(t)
	defer instance.Release()
	if err := instance.Begin(); err != nil {
		t.Fatalf("begin: %v", err)
	}
	err := instance.Begin()
	var refusal *RefusalError
	if !errors.As(err, &refusal) || refusal.Message != "a transaction is already open" {
		t.Fatalf("expected an already-open refusal, got %v", err)
	}
}

func TestACommitWithoutATransactionIsRefused(t *testing.T) {
	instance, _ := loadTemp(t)
	defer instance.Release()
	err := instance.Commit()
	var refusal *RefusalError
	if !errors.As(err, &refusal) || refusal.Message != "no open transaction" {
		t.Fatalf("expected a no-transaction refusal, got %v", err)
	}
}

func TestARollbackWithoutATransactionIsRefused(t *testing.T) {
	instance, _ := loadTemp(t)
	defer instance.Release()
	err := instance.Rollback()
	var refusal *RefusalError
	if !errors.As(err, &refusal) || refusal.Message != "no open transaction" {
		t.Fatalf("expected a no-transaction refusal, got %v", err)
	}
}

func TestAFailedValidationIsRefusedAndClosesTheTransaction(t *testing.T) {
	instance, dir := loadTemp(t)
	defer instance.Release()
	path := filepath.Join(dir, "start.yaml")
	before := readWorkingFile(t, path)
	if err := instance.Begin(); err != nil {
		t.Fatalf("begin: %v", err)
	}
	if _, err := instance.Execute("INSERT INTO users (id, name) VALUES ('01H7B2', 'Dup')"); err != nil {
		t.Fatalf("insert: %v", err)
	}
	err := instance.Commit()
	var refusal *RefusalError
	if !errors.As(err, &refusal) || refusal.Message == "" {
		t.Fatalf("expected the commit to be refused, got %v", err)
	}
	if readWorkingFile(t, path) != before {
		t.Fatal("refused commit touched the disk")
	}
	if err := instance.Begin(); err != nil {
		t.Fatalf("transaction not closed, second begin failed: %v", err)
	}
}
