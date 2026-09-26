<?php

declare(strict_types=1);

namespace Memless\Tests;

use Memless\Libc;
use PHPUnit\Framework\TestCase;

final class LibcTest extends TestCase
{
    public function testLddTellsMuslFromGlibcBeforeTheLoaders(): void
    {
        $loaders = static function (string $file): bool {
            self::fail('the loaders must not be probed');
        };

        $this->assertSame('musl', Libc::detect('musl libc (x86_64)', $loaders));
        $this->assertSame('glibc', Libc::detect('# GNU C Library', $loaders));
    }

    public function testWithoutAnAnswerFromLddTheLoadersAreProbed(): void
    {
        $this->assertSame('glibc', Libc::detect(null, self::only('/lib64/ld-linux-x86-64.so.2')));
        $this->assertSame('musl', Libc::detect('something else', self::only('/lib/ld-musl-aarch64.so.1')));
        $this->assertNull(Libc::detect(null, static fn (string $file): bool => false));
    }

    public function testLddAloneTellsMuslFromGlibc(): void
    {
        $this->assertSame('musl', Libc::fromLdd('musl libc (x86_64)'));
        $this->assertSame('glibc', Libc::fromLdd('# GNU C Library'));
        $this->assertNull(Libc::fromLdd('something else'));
        $this->assertNull(Libc::fromLdd(null));
    }

    public function testWithoutLddTheDynamicLoaderTellsMuslFromGlibc(): void
    {
        $this->assertSame('musl', Libc::fromLoaders(self::only('/lib/ld-musl-x86_64.so.1')));
        $this->assertSame('musl', Libc::fromLoaders(self::only('/lib/ld-musl-aarch64.so.1')));
        $this->assertSame('glibc', Libc::fromLoaders(self::only('/lib64/ld-linux-x86-64.so.2')));
        $this->assertSame('glibc', Libc::fromLoaders(self::only('/lib/ld-linux-aarch64.so.1')));
    }

    public function testTheMuslLoaderWinsOverAGlibcLoaderAsWithGcompatOnAlpine(): void
    {
        $both = static fn (string $file): bool => in_array(
            $file,
            ['/lib/ld-musl-x86_64.so.1', '/lib64/ld-linux-x86-64.so.2'],
            true,
        );

        $this->assertSame('musl', Libc::fromLoaders($both));
    }

    public function testNoLoaderLeavesTheLibcUnknown(): void
    {
        $this->assertNull(Libc::fromLoaders(static fn (string $file): bool => false));
    }

    private static function only(string $loader): callable
    {
        return static fn (string $file): bool => $file === $loader;
    }
}
