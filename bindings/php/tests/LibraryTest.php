<?php

declare(strict_types=1);

namespace Memless\Tests;

use Memless\Library;
use PHPUnit\Framework\TestCase;

final class LibraryTest extends TestCase
{
    public function testAcceptsTheExpectedAbiVersion(): void
    {
        Library::checkVersion(3);
        $this->expectNotToPerformAssertions();
    }

    public function testRejectsAnUnexpectedAbiVersion(): void
    {
        $this->expectException(\LogicException::class);
        Library::checkVersion(1);
    }
}
