<?php

declare(strict_types=1);

use Memless\Instance;
use Memless\MemlessRefusal;

const RELOAD_NAME_QUERY = "SELECT name FROM users WHERE id = '01H7B2'";

const RELOAD_UPDATE = "UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'";

function reloadOutcome(string $fixture, string $scenario): string
{
    $paths = copyFixture($fixture);
    [$instance, $failed] = loadOrFail($paths[1]);
    $report = $instance !== null ? reportReload($instance, $scenario, $fixture, $paths) : $failed;
    cleanupDir($paths[0]);

    return $report;
}

function reportReload(Instance $instance, string $scenario, string $fixture, array $paths): string
{
    [$dir, $dest, $base] = $paths;
    $out = implode("\n", reloadScenario($instance, $scenario, $fixture, $paths)) . "\n";
    $out .= 'sha256:' . fileHash($dest) . "\nresidue:" . residueFlag($dir, $base) . "\n";
    $instance->release();

    return $out;
}

function reloadScenario(Instance $instance, string $scenario, string $fixture, array $paths): array
{
    return match ($scenario) {
        'restore' => restoreLines($instance, $fixture, $paths),
        'open' => openLines($instance, $paths),
        'broken' => brokenLines($instance, $paths),
        'missing' => missingLines($instance, $paths),
        default => ["unknown reload scenario: {$scenario}"],
    };
}

function restoreLines(Instance $instance, string $fixture, array $paths): array
{
    [, $dest, $base] = $paths;
    $update = runWrite($instance, RELOAD_UPDATE, $dest, $base);
    copy($fixture, $dest);
    $reload = runReload($instance, $dest, $base);

    return [$update, $reload, queryInstruction($instance, RELOAD_NAME_QUERY)];
}

function openLines(Instance $instance, array $paths): array
{
    [, $dest, $base] = $paths;
    $begin = runWrite($instance, 'BEGIN', $dest, $base);
    $reload = runReload($instance, $dest, $base);
    $update = runWrite($instance, RELOAD_UPDATE, $dest, $base);
    $read = queryInstruction($instance, RELOAD_NAME_QUERY);
    $rollback = runWrite($instance, 'ROLLBACK', $dest, $base);

    return [$begin, $reload, $update, $read, $rollback];
}

function brokenLines(Instance $instance, array $paths): array
{
    [, $dest, $base] = $paths;
    file_put_contents($dest, "users:\n  - name: Ada\n");
    $reload = runReload($instance, $dest, $base);

    return [$reload, queryInstruction($instance, RELOAD_NAME_QUERY)];
}

function missingLines(Instance $instance, array $paths): array
{
    [, $dest, $base] = $paths;
    unlink($dest);
    $reload = runReload($instance, $dest, $base);

    return [$reload, queryInstruction($instance, RELOAD_NAME_QUERY)];
}

function runReload(Instance $instance, string $dest, string $base): string
{
    try {
        $instance->reload();

        return 'reloaded';
    } catch (MemlessRefusal $refusal) {
        return 'refused:' . str_replace($dest, $base, $refusal->getMessage());
    } catch (\Throwable $fault) {
        return 'fault:' . str_replace($dest, $base, $fault->getMessage());
    }
}
