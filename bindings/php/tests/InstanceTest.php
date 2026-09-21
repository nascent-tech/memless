<?php

declare(strict_types=1);

namespace Memless\Tests;

use Memless\Instance;
use Memless\MemlessRefusal;
use PHPUnit\Framework\TestCase;

final class InstanceTest extends TestCase
{
    private function fixture(string $name): string
    {
        return dirname(__DIR__, 3) . '/harness/parity/fixtures/' . $name;
    }

    public function testLoadsAValidFileAndReturnsAnInstance(): void
    {
        $instance = Instance::load($this->fixture('start.yaml'));
        $this->assertInstanceOf(Instance::class, $instance);
        $instance->release();
    }

    public function testThrowsRefusalCarryingTheExactDomainMessage(): void
    {
        $this->expectException(MemlessRefusal::class);
        $this->expectExceptionMessage('row 1 in "users" has no id');
        Instance::load($this->fixture('missing-id.yaml'));
    }

    public function testRefusesANonExistentPathAsARefusal(): void
    {
        $this->expectException(MemlessRefusal::class);
        Instance::load($this->fixture('does-not-exist.yaml'));
    }

    public function testThrowsLogicExceptionOnABoundaryFault(): void
    {
        $this->expectException(\LogicException::class);
        Instance::load("\xFFnot-utf-8");
    }

    public function testThrowsLogicExceptionOnANulBytePath(): void
    {
        $this->expectException(\LogicException::class);
        Instance::load("has\0nul");
    }

    public function testReleaseIsIdempotent(): void
    {
        $instance = Instance::load($this->fixture('start.yaml'));
        $instance->release();
        $instance->release();
        $this->expectNotToPerformAssertions();
    }

    public function testDestructReleasesWithoutError(): void
    {
        $instance = Instance::load($this->fixture('start.yaml'));
        unset($instance);
        $this->expectNotToPerformAssertions();
    }
}
