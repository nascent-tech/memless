<?php

declare(strict_types=1);

require __DIR__ . '/vendor/autoload.php';

use Memless\Instance;
use Memless\MemlessRefusal;

function loadOutcome(string $path): string
{
    try {
        $instance = Instance::load($path);
        $instance->release();

        return 'accepted';
    } catch (MemlessRefusal $refusal) {
        return 'refused: ' . $refusal->getMessage();
    } catch (\Throwable $fault) {
        return 'fault: ' . $fault->getMessage();
    }
}

function queryOutcome(string $path, string $sql): string
{
    try {
        $instance = Instance::load($path);
    } catch (MemlessRefusal $refusal) {
        return 'refused: ' . $refusal->getMessage() . "\n";
    } catch (\Throwable $fault) {
        return 'fault: ' . $fault->getMessage() . "\n";
    }

    return runQuery($instance, $sql);
}

function runQuery(Instance $instance, string $sql): string
{
    try {
        $rows = $instance->query($sql);

        return render($rows);
    } catch (MemlessRefusal $refusal) {
        return 'refused: ' . $refusal->getMessage() . "\n";
    } catch (\Throwable $fault) {
        return 'fault: ' . $fault->getMessage() . "\n";
    } finally {
        $instance->release();
    }
}

function render(array $rows): string
{
    if ($rows === []) {
        return "ok\nempty\n";
    }
    $out = 'ok' . "\n" . 'cols';
    foreach (array_keys($rows[0]) as $name) {
        $out .= "\t" . escape((string) $name);
    }
    $out .= "\n";
    foreach ($rows as $row) {
        $out .= renderRow($row);
    }

    return $out;
}

function renderRow(array $row): string
{
    $out = 'row';
    foreach ($row as $value) {
        $out .= "\t" . renderCell($value);
    }

    return $out . "\n";
}

function renderCell(int|float|bool|string|null $value): string
{
    if ($value === null) {
        return 'null';
    }
    if (is_int($value)) {
        return 'int:' . $value;
    }
    if (is_float($value)) {
        return 'dec:' . renderFloat($value);
    }
    if (is_bool($value)) {
        return $value ? 'bool:true' : 'bool:false';
    }

    return 'str:' . escape($value);
}

function renderFloat(float $value): string
{
    return sprintf('%016x', unpack('J', pack('E', $value))[1]);
}

function escape(string $text): string
{
    return str_replace(["\\", "\t", "\n"], ['\\\\', '\t', '\n'], $text);
}

function copyFixture(string $fixture): array
{
    $dir = sys_get_temp_dir() . '/memless-parity-' . bin2hex(random_bytes(6));
    mkdir($dir, 0755, recursive: true);
    $base = basename($fixture);
    $dest = $dir . '/' . $base;
    copy($fixture, $dest);

    return [$dir, $dest, $base];
}

function writeReport(Instance $instance, string $sql, array $paths): string
{
    [$dir, $dest, $base] = $paths;
    $line = runWrite($instance, $sql, $dest, $base);
    $report = $line . "\nsha256:" . fileHash($dest) . "\nresidue:" . residueFlag($dir, $base) . "\n";
    $instance->release();

    return $report;
}

function writeOutcome(string $fixture, string $sql): string
{
    $paths = copyFixture($fixture);
    $report = writeReport(Instance::load($paths[1]), $sql, $paths);
    cleanupDir($paths[0]);

    return $report;
}

function writeDiskOutcome(string $fixture, string $sql): string
{
    $paths = copyFixture($fixture);
    $instance = Instance::load($paths[1]);
    chmod($paths[0], 0555);
    $report = writeReport($instance, $sql, $paths);
    chmod($paths[0], 0755);
    cleanupDir($paths[0]);

    return $report;
}

function runWrite(Instance $instance, string $sql, string $dest, string $base): string
{
    try {
        return 'accepted:' . $instance->execute($sql);
    } catch (MemlessRefusal $refusal) {
        return 'refused:' . str_replace($dest, $base, $refusal->getMessage());
    } catch (\Throwable $fault) {
        return 'fault:' . str_replace($dest, $base, $fault->getMessage());
    }
}

function fileHash(string $dest): string
{
    return is_file($dest) ? hash_file('sha256', $dest) : '-';
}

function residueFlag(string $dir, string $base): string
{
    return glob($dir . '/.' . $base . '.memless-tmp') ? 'yes' : 'no';
}

function cleanupDir(string $dir): void
{
    array_map('unlink', array_filter(glob($dir . '/{,.}*', GLOB_BRACE) ?: [], 'is_file'));
    if (is_dir($dir)) {
        rmdir($dir);
    }
}

function dispatch(string $path, ?string $sql, ?string $mode): string
{
    if ($mode === 'write') {
        return writeOutcome($path, $sql ?? '');
    }
    if ($mode === 'write-disk') {
        return writeDiskOutcome($path, $sql ?? '');
    }
    if ($sql !== null) {
        return queryOutcome($path, $sql);
    }

    return loadOutcome($path) . "\n";
}

echo dispatch($argv[1] ?? '', $argv[2] ?? null, $argv[3] ?? null);
