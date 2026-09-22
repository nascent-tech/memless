package main

import (
	"crypto/sha256"
	"encoding/hex"
	"os"
	"path/filepath"
)

func copyFixture(fixture string) (string, string, string) {
	dir, _ := os.MkdirTemp("", "memless-parity-")
	base := filepath.Base(fixture)
	dest := filepath.Join(dir, base)
	data, _ := os.ReadFile(fixture)
	_ = os.WriteFile(dest, data, 0o600)
	return dir, dest, base
}

func fileHash(dest string) string {
	data, err := os.ReadFile(dest)
	if err != nil {
		return "-"
	}
	sum := sha256.Sum256(data)
	return hex.EncodeToString(sum[:])
}

func residueFlag(dir string, base string) string {
	matches, _ := filepath.Glob(filepath.Join(dir, "."+base+".memless-tmp"))
	if len(matches) > 0 {
		return "yes"
	}
	return "no"
}

func cleanupDir(dir string) {
	_ = os.RemoveAll(dir)
}
