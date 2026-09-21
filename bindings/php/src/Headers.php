<?php

declare(strict_types=1);

namespace Memless;

/**
 * Reads the result column names. A duplicate name cannot be an associative key,
 * so the bridge refuses to drop one silently and raises instead of deciding.
 */
final class Headers
{
    public static function of(\FFI $ffi, int $result): array
    {
        $count = $ffi->memless_result_column_count($result);
        $names = array_fill(0, $count, '');
        for ($index = 0; $index < $count; $index++) {
            $names[$index] = $ffi->memless_result_column($result, $index);
        }

        return self::unique($names);
    }

    private static function unique(array $names): array
    {
        if (count($names) !== count(array_unique($names))) {
            throw new \LogicException('duplicate column name in result');
        }

        return $names;
    }
}
