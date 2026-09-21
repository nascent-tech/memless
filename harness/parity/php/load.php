<?php

declare(strict_types=1);

require __DIR__ . '/vendor/autoload.php';

use Memless\Instance;
use Memless\MemlessRefusal;

function outcome(string $path): string
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

echo outcome($argv[1] ?? '') . "\n";
