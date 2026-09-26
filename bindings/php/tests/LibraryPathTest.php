<?php

declare(strict_types=1);

namespace Memless\Tests;

use Memless\LibraryPath;
use PHPUnit\Framework\TestCase;

final class LibraryPathTest extends TestCase
{
    public function testRejectsAMissingMemlessLib(): void
    {
        putenv('MEMLESS_LIB=/nonexistent/path/lib.dylib');
        try {
            $this->expectException(\LogicException::class);
            $this->expectExceptionMessageMatches('#/nonexistent/path/lib\.dylib#');
            LibraryPath::library();
        } finally {
            putenv('MEMLESS_LIB');
        }
    }

    public function testAcceptsAnExistingMemlessLib(): void
    {
        putenv('MEMLESS_LIB');
        $real = LibraryPath::library();
        putenv("MEMLESS_LIB={$real}");
        try {
            $this->assertSame($real, LibraryPath::library());
        } finally {
            putenv('MEMLESS_LIB');
        }
    }
}
