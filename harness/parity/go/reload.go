package main

import (
	"os"
	"strings"

	memless "github.com/nascent-tech/memless-go"
)

const (
	reloadNameQuery = "SELECT name FROM users WHERE id = '01H7B2'"
	reloadUpdate    = "UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'"
)

type reloadPaths struct {
	fixture string
	dir     string
	dest    string
	base    string
}

func reloadOutcome(fixture string, scenario string) string {
	dir, dest, base := copyFixture(fixture)
	defer cleanupDir(dir)
	instance, failed := loadInstance(dest)
	if instance == nil {
		return failed
	}
	paths := reloadPaths{fixture: fixture, dir: dir, dest: dest, base: base}
	return reportReload(instance, scenario, paths)
}

func reportReload(instance *memless.Instance, scenario string, paths reloadPaths) string {
	defer instance.Release()
	var out strings.Builder
	for _, line := range reloadScenario(instance, scenario, paths) {
		out.WriteString(line)
		out.WriteString("\n")
	}
	out.WriteString("sha256:" + fileHash(paths.dest) + "\nresidue:" + residueFlag(paths.dir, paths.base) + "\n")
	return out.String()
}

var reloadScenarios = map[string]func(*memless.Instance, reloadPaths) []string{
	"restore": restoreLines,
	"open":    openLines,
	"broken":  brokenLines,
	"missing": missingLines,
}

func reloadScenario(instance *memless.Instance, scenario string, paths reloadPaths) []string {
	lines, known := reloadScenarios[scenario]
	if !known {
		return []string{"unknown reload scenario: " + scenario}
	}
	return lines(instance, paths)
}

func restoreLines(instance *memless.Instance, paths reloadPaths) []string {
	update := runWrite(instance, reloadUpdate, paths.dest, paths.base)
	data, _ := os.ReadFile(paths.fixture)
	_ = os.WriteFile(paths.dest, data, 0o600)
	reload := runReload(instance, paths.dest, paths.base)
	return []string{update, reload, queryInstruction(instance, reloadNameQuery)}
}

func openLines(instance *memless.Instance, paths reloadPaths) []string {
	begin := runWrite(instance, "BEGIN", paths.dest, paths.base)
	reload := runReload(instance, paths.dest, paths.base)
	update := runWrite(instance, reloadUpdate, paths.dest, paths.base)
	read := queryInstruction(instance, reloadNameQuery)
	rollback := runWrite(instance, "ROLLBACK", paths.dest, paths.base)
	return []string{begin, reload, update, read, rollback}
}

func brokenLines(instance *memless.Instance, paths reloadPaths) []string {
	_ = os.WriteFile(paths.dest, []byte("users:\n  - name: Ada\n"), 0o600)
	reload := runReload(instance, paths.dest, paths.base)
	return []string{reload, queryInstruction(instance, reloadNameQuery)}
}

func missingLines(instance *memless.Instance, paths reloadPaths) []string {
	_ = os.Remove(paths.dest)
	reload := runReload(instance, paths.dest, paths.base)
	return []string{reload, queryInstruction(instance, reloadNameQuery)}
}

func runReload(instance *memless.Instance, dest string, base string) string {
	err := instance.Reload()
	if err == nil {
		return "reloaded"
	}
	return strings.ReplaceAll(writeError(err), dest, base)
}
