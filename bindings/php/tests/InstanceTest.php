<?php

declare(strict_types=1);

namespace Memless\Tests;

use Memless\Instance;
use Memless\MemlessFault;
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

    public function testThrowsAFaultOnABoundaryFault(): void
    {
        try {
            Instance::load("\xFFnot-utf-8");
            $this->fail('expected a MemlessFault');
        } catch (MemlessFault $fault) {
            $this->assertSame(2, $fault->status);
        }
    }

    public function testThrowsAFaultOnANulBytePath(): void
    {
        try {
            Instance::load("has\0nul");
            $this->fail('expected a MemlessFault');
        } catch (MemlessFault $fault) {
            $this->assertSame(2, $fault->status);
        }
    }

    public function testQueriesRowsInFileOrder(): void
    {
        $instance = Instance::load($this->fixture('start.yaml'));
        $rows = $instance->query('SELECT name FROM users');
        $this->assertSame([['name' => 'Ada'], ['name' => 'Grace']], $rows);
        $instance->release();
    }

    public function testQueriesAnAggregate(): void
    {
        $instance = Instance::load($this->fixture('start.yaml'));
        $rows = $instance->query('SELECT COUNT(*) FROM users');
        $this->assertSame([['COUNT(*)' => 2]], $rows);
        $instance->release();
    }

    public function testQueriesEachCellKind(): void
    {
        $instance = Instance::load($this->fixture('kinds.yaml'));
        $rows = $instance->query('SELECT ratio, active, label FROM things');
        $this->assertSame(1.5, $rows[0]['ratio']);
        $this->assertTrue($rows[0]['active']);
        $this->assertSame('hi', $rows[0]['label']);
        $this->assertFalse($rows[1]['active']);
        $this->assertNull($rows[1]['label']);
        $instance->release();
    }

    public function testTheTrapJoinsTheSameTypeIdOnly(): void
    {
        $instance = Instance::load($this->fixture('trap.yaml'));
        $sql = 'SELECT users.id FROM wallets JOIN users ON wallets.user_id = users.id';
        $rows = $instance->query($sql);
        $this->assertSame([['users.id' => 5]], $rows);
        $instance->release();
    }

    public function testFiltersUnderTheOneComparisonRule(): void
    {
        $instance = Instance::load($this->fixture('start.yaml'));
        $this->assertSame([['name' => 'Ada']], $instance->query("SELECT name FROM users WHERE id = '01H7B2'"));
        $this->assertSame([], $instance->query('SELECT name FROM users WHERE id = 999'));
        $instance->release();
    }

    public function testRefusesADuplicateOutputColumnFromTheCore(): void
    {
        $instance = Instance::load($this->fixture('start.yaml'));
        try {
            $this->expectException(MemlessRefusal::class);
            $this->expectExceptionMessage('duplicate output column is outside the supported SQL subset');
            $instance->query('SELECT id, id FROM users');
        } finally {
            $instance->release();
        }
    }

    public function testQueryRefusalCarriesTheExactDomainMessage(): void
    {
        $instance = Instance::load($this->fixture('start.yaml'));
        try {
            $this->expectException(MemlessRefusal::class);
            $this->expectExceptionMessage('no table "ghosts"');
            $instance->query('SELECT * FROM ghosts');
        } finally {
            $instance->release();
        }
    }

    public function testReleaseIsIdempotent(): void
    {
        $instance = Instance::load($this->fixture('start.yaml'));
        $instance->release();
        $instance->release();
        $this->expectNotToPerformAssertions();
    }

    public function testQueryAfterReleaseFaults(): void
    {
        $instance = Instance::load($this->fixture('start.yaml'));
        $instance->release();
        try {
            $instance->query('SELECT * FROM users');
            $this->fail('expected a MemlessFault');
        } catch (MemlessFault $fault) {
            $this->assertSame(2, $fault->status);
        }
    }

    public function testDestructReleasesWithoutError(): void
    {
        $instance = Instance::load($this->fixture('start.yaml'));
        unset($instance);
        $this->expectNotToPerformAssertions();
    }
}
