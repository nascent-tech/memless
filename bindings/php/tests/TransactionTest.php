<?php

declare(strict_types=1);

namespace Memless\Tests;

use Memless\Instance;
use Memless\MemlessRefusal;
use PHPUnit\Framework\TestCase;

final class TransactionTest extends TestCase
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
        $dir = sys_get_temp_dir() . '/memless-php-txn-' . bin2hex(random_bytes(6));
        mkdir($dir, 0755, true);
        $this->workdirs[] = $dir;
        $dest = $dir . '/' . $name;
        copy($this->fixture($name), $dest);

        return [Instance::load($dest), $dest];
    }

    public function testATransactionReadsItsOwnWritesAndCommitsOnceToDisk(): void
    {
        [$instance, $dest] = $this->loadCopy('start.yaml');
        $before = file_get_contents($dest);
        $instance->begin();
        $this->assertSame(1, $instance->execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'"));
        $this->assertSame([['name' => 'Zoe']], $instance->query("SELECT name FROM users WHERE id = '01H7B2'"));
        $this->assertSame($before, file_get_contents($dest));
        $instance->commit();
        $after = file_get_contents($dest);
        $this->assertNotSame($before, $after);
        $this->assertStringContainsString('Zoe', $after);
        $instance->release();
    }

    public function testARollbackLeavesTheFileAndRestoresTheState(): void
    {
        [$instance, $dest] = $this->loadCopy('start.yaml');
        $before = file_get_contents($dest);
        $instance->begin();
        $instance->execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'");
        $instance->rollback();
        $this->assertSame($before, file_get_contents($dest));
        $this->assertSame([['name' => 'Ada']], $instance->query("SELECT name FROM users WHERE id = '01H7B2'"));
        $instance->release();
    }

    public function testASecondBeginIsRefused(): void
    {
        [$instance] = $this->loadCopy('start.yaml');
        $instance->begin();
        try {
            $this->expectException(MemlessRefusal::class);
            $this->expectExceptionMessage('a transaction is already open');
            $instance->begin();
        } finally {
            $instance->release();
        }
    }

    public function testACommitWithoutATransactionIsRefused(): void
    {
        [$instance] = $this->loadCopy('start.yaml');
        try {
            $this->expectException(MemlessRefusal::class);
            $this->expectExceptionMessage('no open transaction');
            $instance->commit();
        } finally {
            $instance->release();
        }
    }

    public function testARollbackWithoutATransactionIsRefused(): void
    {
        [$instance] = $this->loadCopy('start.yaml');
        try {
            $this->expectException(MemlessRefusal::class);
            $this->expectExceptionMessage('no open transaction');
            $instance->rollback();
        } finally {
            $instance->release();
        }
    }

    public function testAFailedValidationIsRefusedAndClosesTheTransaction(): void
    {
        [$instance, $dest] = $this->loadCopy('start.yaml');
        $before = file_get_contents($dest);
        $instance->begin();
        $instance->execute("INSERT INTO users (id, name) VALUES ('01H7B2', 'Dup')");
        $message = '';
        try {
            $instance->commit();
        } catch (MemlessRefusal $refusal) {
            $message = $refusal->getMessage();
        }
        $this->assertStringContainsString('duplicate id', $message);
        $this->assertSame($before, file_get_contents($dest));
        $instance->begin();
        $instance->release();
    }
}
