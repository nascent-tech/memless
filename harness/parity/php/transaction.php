<?php

declare(strict_types=1);

use Memless\Instance;
use Memless\MemlessRefusal;

function transactionOutcome(string $fixture, string $suite): string
{
    $paths = copyFixture($fixture);
    [$instance, $failed] = loadOrFail($paths[1]);
    $report = $instance !== null ? reportSuite($instance, $suite, $paths) : $failed;
    cleanupDir($paths[0]);

    return $report;
}

function transactionDiskOutcome(string $fixture, string $suite): string
{
    $paths = copyFixture($fixture);
    [$instance, $failed] = loadOrFail($paths[1]);
    $report = $instance !== null
        ? withReadonlyDir($paths[0], fn () => reportSuite($instance, $suite, $paths))
        : $failed;
    cleanupDir($paths[0]);

    return $report;
}

function reportSuite(Instance $instance, string $suite, array $paths): string
{
    [$dir, $dest, $base] = $paths;
    $out = '';
    foreach (explode(';;', $suite) as $sql) {
        $out .= runInstruction($instance, $sql, $dest, $base) . "\n";
    }
    $out .= 'sha256:' . fileHash($dest) . "\nresidue:" . residueFlag($dir, $base) . "\n";
    $instance->release();

    return $out;
}

function runInstruction(Instance $instance, string $sql, string $dest, string $base): string
{
    if (str_starts_with(strtoupper(trim($sql)), 'SELECT')) {
        return queryInstruction($instance, $sql);
    }

    return runWrite($instance, $sql, $dest, $base);
}

function queryInstruction(Instance $instance, string $sql): string
{
    try {
        return rtrim(render($instance->query($sql)), "\n");
    } catch (MemlessRefusal $refusal) {
        return 'refused: ' . $refusal->getMessage();
    } catch (\Throwable $fault) {
        return 'fault: ' . $fault->getMessage();
    }
}
