<?php

declare(strict_types=1);

namespace Memless\Tests;

use Memless\Instance;
use Memless\MemlessFault;
use Memless\MemlessRefusal;
use PHPUnit\Framework\TestCase;

final class ExecuteTest extends TestCase
{
    /** @var string[] */
    private array $workdirs = [];

    protected function tearDown(): void
    {
        foreach ($this->workdirs as $dir) {
            foreach (glob($dir . '/*') ?: [] as $file) {
                unlink($file);
            }
            if (is_dir($dir)) {
                rmdir($dir);
            }
        }
        $this->workdirs = [];
    }

    private function fixture(string $name): string
    {
        return dirname(__DIR__, 3) . '/harness/parity/fixtures/' . $name;
    }

    private function loadCopy(string $name): array
    {
        $dir = sys_get_temp_dir() . '/memless-php-exec-' . bin2hex(random_bytes(6));
        mkdir($dir, 0755, true);
        $this->workdirs[] = $dir;
        $dest = $dir . '/' . $name;
        copy($this->fixture($name), $dest);

        return [Instance::load($dest), $dir, $dest];
    }

    public function testExecutesAWriteAndCountsTheRows(): void
    {
        [$instance] = $this->loadCopy('start.yaml');
        $affected = $instance->execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'");
        $this->assertSame(1, $affected);
        $this->assertSame([['name' => 'Zoe']], $instance->query("SELECT name FROM users WHERE id = '01H7B2'"));
        $instance->release();
    }

    public function testRefusesAWriteCarryingTheDomainMessage(): void
    {
        [$instance] = $this->loadCopy('start.yaml');
        try {
            $this->expectException(MemlessRefusal::class);
            $this->expectExceptionMessage('no table "ghosts"');
            $instance->execute('DELETE FROM ghosts');
        } finally {
            $instance->release();
        }
    }

    public function testRefusesASelectPassedToExecute(): void
    {
        [$instance] = $this->loadCopy('start.yaml');
        try {
            $this->expectException(MemlessRefusal::class);
            $this->expectExceptionMessage('a SELECT in execute');
            $instance->execute('SELECT * FROM users');
        } finally {
            $instance->release();
        }
    }

    public function testExecuteAfterReleaseFaults(): void
    {
        [$instance] = $this->loadCopy('start.yaml');
        $instance->release();
        try {
            $instance->execute('DELETE FROM users');
            $this->fail('expected a MemlessFault');
        } catch (MemlessFault $fault) {
            $this->assertSame(2, $fault->status);
        }
    }

    public function testADiskFailureIsRefusedAndLeavesMemoryIntact(): void
    {
        [$instance, $dir, $dest] = $this->loadCopy('start.yaml');
        unlink($dest);
        rmdir($dir);
        $message = '';
        try {
            $instance->execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'");
        } catch (MemlessRefusal $refusal) {
            $message = $refusal->getMessage();
        }
        $this->assertStringContainsString('cannot write file', $message);
        $this->assertSame([['name' => 'Ada']], $instance->query("SELECT name FROM users WHERE id = '01H7B2'"));
        $instance->release();
    }
}
