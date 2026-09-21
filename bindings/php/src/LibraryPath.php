<?php

declare(strict_types=1);

namespace Memless;

/**
 * Resolves the header and cdylib paths, from the environment or the workspace.
 */
final class LibraryPath
{
    public static function header(): string
    {
        $env = getenv('MEMLESS_HEADER');
        $default = dirname(__DIR__, 3) . '/crates/memless-capi/include/memless.h';
        $path = is_string($env) && $env !== '' ? $env : $default;
        if (!is_file($path)) {
            throw new \LogicException("memless header not found at {$path}; set MEMLESS_HEADER");
        }

        return $path;
    }

    public static function library(): string
    {
        $env = getenv('MEMLESS_LIB');
        if (!is_string($env) || $env === '') {
            return self::search();
        }
        if (!is_file($env)) {
            throw new \LogicException("memless cdylib not found at {$env}; set MEMLESS_LIB");
        }

        return $env;
    }

    private static function search(): string
    {
        $found = array_filter(self::candidates(), 'is_file');
        if ($found === []) {
            throw new \LogicException('memless cdylib not found; set MEMLESS_LIB');
        }

        return (string) reset($found);
    }

    private static function candidates(): array
    {
        $root = dirname(__DIR__, 3);

        return [
            "{$root}/target/release/libmemless_capi.dylib",
            "{$root}/target/debug/libmemless_capi.dylib",
            "{$root}/target/release/libmemless_capi.so",
            "{$root}/target/debug/libmemless_capi.so",
        ];
    }
}
