package memless

import (
	"errors"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
)

func TestAReloadPicksUpAnExternalEditOfTheFile(t *testing.T) {
	instance, dir := loadTemp(t)
	defer instance.Release()
	path := filepath.Join(dir, "start.yaml")
	edited := strings.Replace(startYAML, "name: Ada", "name: Zoe", 1)
	if err := os.WriteFile(path, []byte(edited), 0o600); err != nil {
		t.Fatalf("write: %v", err)
	}
	if err := instance.Reload(); err != nil {
		t.Fatalf("reload: %v", err)
	}
	rows, err := instance.Query("SELECT name FROM users WHERE id = '01H7B2'")
	if err != nil {
		t.Fatalf("query: %v", err)
	}
	if !reflect.DeepEqual(rows.Rows, [][]any{{"Zoe"}}) {
		t.Fatalf("reload did not pick up the edit: got %v", rows.Rows)
	}
}

func TestAReloadDuringAnOpenTransactionIsRefusedAndTheTransactionStaysUsable(t *testing.T) {
	instance, _ := loadTemp(t)
	defer instance.Release()
	if err := instance.Begin(); err != nil {
		t.Fatalf("begin: %v", err)
	}
	err := instance.Reload()
	var refusal *RefusalError
	if !errors.As(err, &refusal) || refusal.Message != "cannot reload while a transaction is open" {
		t.Fatalf("expected the open-transaction refusal, got %v", err)
	}
	if _, err := instance.Execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'"); err != nil {
		t.Fatalf("transaction not usable after refused reload: %v", err)
	}
	if err := instance.Commit(); err != nil {
		t.Fatalf("commit: %v", err)
	}
}

func TestAReloadOfAnIncoherentFileIsRefusedAndKeepsTheOldState(t *testing.T) {
	instance, dir := loadTemp(t)
	defer instance.Release()
	path := filepath.Join(dir, "start.yaml")
	if err := os.WriteFile(path, []byte("users:\n  - name: Ada\n"), 0o600); err != nil {
		t.Fatalf("write: %v", err)
	}
	err := instance.Reload()
	var refusal *RefusalError
	if !errors.As(err, &refusal) || refusal.Message == "" {
		t.Fatalf("expected a structure refusal, got %v", err)
	}
	rows, err := instance.Query("SELECT name FROM users WHERE id = '01H7B2'")
	if err != nil {
		t.Fatalf("query after failed reload: %v", err)
	}
	if !reflect.DeepEqual(rows.Rows, [][]any{{"Ada"}}) {
		t.Fatalf("old state was not kept: got %v", rows.Rows)
	}
}

func TestAReloadOfAMissingFileIsRefusedNamingThePathAndKeepsTheOldState(t *testing.T) {
	instance, dir := loadTemp(t)
	defer instance.Release()
	path := filepath.Join(dir, "start.yaml")
	if err := os.Remove(path); err != nil {
		t.Fatalf("remove: %v", err)
	}
	err := instance.Reload()
	var refusal *RefusalError
	if !errors.As(err, &refusal) || !strings.HasPrefix(refusal.Message, "no file at path") {
		t.Fatalf("expected a no-file refusal naming the path, got %v", err)
	}
	rows, err := instance.Query("SELECT name FROM users WHERE id = '01H7B2'")
	if err != nil {
		t.Fatalf("query after failed reload: %v", err)
	}
	if !reflect.DeepEqual(rows.Rows, [][]any{{"Ada"}}) {
		t.Fatalf("old state was not kept: got %v", rows.Rows)
	}
}

func TestAReloadOnAReleasedHandleIsAFault(t *testing.T) {
	instance, _ := loadTemp(t)
	instance.Release()
	err := instance.Reload()
	var fault *FaultError
	if !errors.As(err, &fault) || fault.Status != statusInvalidArgument {
		t.Fatalf("expected a FaultError with status 2, got %v", err)
	}
}
