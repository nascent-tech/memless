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

$mode = $argv[2] ?? null;
if ($mode !== null) {
    echo queryOutcome($argv[1] ?? '', $mode);
} else {
    echo loadOutcome($argv[1] ?? '') . "\n";
}
