package main

import (
	"errors"
	"strconv"
	"strings"

	memless "github.com/nascent-tech/memless/bindings/go"
)

func writeOutcome(fixture string, sql string) string {
	dir, dest, base := copyFixture(fixture)
	defer cleanupDir(dir)
	instance, failed := loadInstance(dest)
	if instance == nil {
		return failed
	}
	return report(instance, dir, dest, base, sql)
}

func writeDiskOutcome(fixture string, sql string) string {
	dir, dest, base := copyFixture(fixture)
	defer cleanupDir(dir)
	instance, failed := loadInstance(dest)
	if instance == nil {
		return failed
	}
	return withReadonlyDir(dir, func() string { return report(instance, dir, dest, base, sql) })
}

func loadInstance(dest string) (*memless.Instance, string) {
	instance, err := memless.Load(dest)
	if err != nil {
		return nil, writeError(err) + "\nsha256:-\nresidue:no\n"
	}
	return instance, ""
}

func report(instance *memless.Instance, dir string, dest string, base string, sql string) string {
	defer instance.Release()
	line := runWrite(instance, sql, dest, base)
	return line + "\nsha256:" + fileHash(dest) + "\nresidue:" + residueFlag(dir, base) + "\n"
}

func runWrite(instance *memless.Instance, sql string, dest string, base string) string {
	affected, err := instance.Execute(sql)
	if err == nil {
		return "accepted:" + strconv.FormatUint(affected, 10)
	}
	return strings.ReplaceAll(writeError(err), dest, base)
}

func writeError(err error) string {
	var refusal *memless.RefusalError
	if errors.As(err, &refusal) {
		return "refused:" + refusal.Message
	}
	return "fault:" + err.Error()
}
