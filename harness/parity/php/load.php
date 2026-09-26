<?php

declare(strict_types=1);

require __DIR__ . '/vendor/autoload.php';
require __DIR__ . '/transaction.php';

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

function loadOrFail(string $path): array
{
    try {
        return [Instance::load($path), null];
    } catch (MemlessRefusal $refusal) {
        return [null, 'refused:' . $refusal->getMessage() . "\nsha256:-\nresidue:no\n"];
    } catch (\Throwable $fault) {
        return [null, 'fault:' . $fault->getMessage() . "\nsha256:-\nresidue:no\n"];
    }
}

function writeOutcome(string $fixture, string $sql): string
{
    $paths = copyFixture($fixture);
    [$instance, $failed] = loadOrFail($paths[1]);
    $report = $instance !== null ? writeReport($instance, $sql, $paths) : $failed;
    cleanupDir($paths[0]);

    return $report;
}

function writeDiskOutcome(string $fixture, string $sql): string
{
    $paths = copyFixture($fixture);
    [$instance, $failed] = loadOrFail($paths[1]);
    $report = $instance !== null
        ? withReadonlyDir($paths[0], fn () => writeReport($instance, $sql, $paths))
        : $failed;
    cleanupDir($paths[0]);

    return $report;
}

function withReadonlyDir(string $dir, callable $fn): mixed
{
    chmod($dir, 0555);
    try {
        return $fn();
    } finally {
        chmod($dir, 0755);
    }
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

function dispatchMode(string $path, string $sql, string $mode): string
{
    return match ($mode) {
        'write' => writeOutcome($path, $sql),
        'write-disk' => writeDiskOutcome($path, $sql),
        'transaction' => transactionOutcome($path, $sql),
        'transaction-disk' => transactionDiskOutcome($path, $sql),
        default => '',
    };
}

function dispatch(string $path, ?string $sql, ?string $mode): string
{
    if ($mode !== null) {
        return dispatchMode($path, $sql ?? '', $mode);
    }
    if ($sql !== null) {
        return queryOutcome($path, $sql);
    }

    return loadOutcome($path) . "\n";
}

echo dispatch($argv[1] ?? '', $argv[2] ?? null, $argv[3] ?? null);
