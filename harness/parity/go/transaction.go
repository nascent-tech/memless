package main

import (
	"strings"

	memless "github.com/nascent-tech/memless-go"
)

func transactionOutcome(fixture string, suite string) string {
	dir, dest, base := copyFixture(fixture)
	defer cleanupDir(dir)
	instance, failed := loadInstance(dest)
	if instance == nil {
		return failed
	}
	return reportSuite(instance, dir, dest, base, suite)
}

func transactionDiskOutcome(fixture string, suite string) string {
	dir, dest, base := copyFixture(fixture)
	defer cleanupDir(dir)
	instance, failed := loadInstance(dest)
	if instance == nil {
		return failed
	}
	return withReadonlyDir(dir, func() string { return reportSuite(instance, dir, dest, base, suite) })
}

func reportSuite(instance *memless.Instance, dir string, dest string, base string, suite string) string {
	defer instance.Release()
	var out strings.Builder
	for _, sql := range strings.Split(suite, ";;") {
		out.WriteString(runInstruction(instance, sql, dest, base))
		out.WriteString("\n")
	}
	out.WriteString("sha256:" + fileHash(dest) + "\nresidue:" + residueFlag(dir, base) + "\n")
	return out.String()
}

func runInstruction(instance *memless.Instance, sql string, dest string, base string) string {
	if isSelect(sql) {
		return queryInstruction(instance, sql)
	}
	return runWrite(instance, sql, dest, base)
}

func isSelect(sql string) bool {
	return strings.HasPrefix(strings.ToUpper(strings.TrimSpace(sql)), "SELECT")
}

func queryInstruction(instance *memless.Instance, sql string) string {
	rows, err := instance.Query(sql)
	if err != nil {
		return errorLine(err)
	}
	return strings.TrimRight(render(rows), "\n")
}
