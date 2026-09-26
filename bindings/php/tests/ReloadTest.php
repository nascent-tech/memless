<?php

declare(strict_types=1);

namespace Memless\Tests;

use Memless\Instance;
use Memless\MemlessFault;
use Memless\MemlessRefusal;
use PHPUnit\Framework\TestCase;

final class ReloadTest extends TestCase
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
        $dir = sys_get_temp_dir() . '/memless-php-reload-' . bin2hex(random_bytes(6));
        mkdir($dir, 0755, true);
        $this->workdirs[] = $dir;
        $dest = $dir . '/' . $name;
        copy($this->fixture($name), $dest);

        return [Instance::load($dest), $dest];
    }

    public function testAReloadPicksUpAnExternalEditOfTheFile(): void
    {
        [$instance, $dest] = $this->loadCopy('start.yaml');
        file_put_contents($dest, str_replace('name: Ada', 'name: Zoe', file_get_contents($dest)));
        $instance->reload();
        $this->assertSame([['name' => 'Zoe']], $instance->query("SELECT name FROM users WHERE id = '01H7B2'"));
        $instance->release();
    }

    public function testAReloadDuringAnOpenTransactionIsRefusedAndTheTransactionStaysUsable(): void
    {
        [$instance] = $this->loadCopy('start.yaml');
        $instance->begin();
        try {
            $instance->reload();
            $this->fail('expected the open-transaction refusal');
        } catch (MemlessRefusal $refusal) {
            $this->assertSame('cannot reload while a transaction is open', $refusal->getMessage());
        }
        $this->assertSame(1, $instance->execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'"));
        $instance->commit();
        $instance->release();
    }

    public function testAReloadOfAnIncoherentFileIsRefusedAndKeepsTheOldState(): void
    {
        [$instance, $dest] = $this->loadCopy('start.yaml');
        file_put_contents($dest, "users:\n  - name: Ada\n");
        try {
            $instance->reload();
            $this->fail('expected an incoherent-file refusal');
        } catch (MemlessRefusal $refusal) {
            $this->assertNotSame('', $refusal->getMessage());
        }
        $this->assertSame([['name' => 'Ada']], $instance->query("SELECT name FROM users WHERE id = '01H7B2'"));
        $instance->release();
    }

    public function testAReloadOfAMissingFileIsRefusedNamingThePathAndKeepsTheOldState(): void
    {
        [$instance, $dest] = $this->loadCopy('start.yaml');
        unlink($dest);
        try {
            $instance->reload();
            $this->fail('expected a no-file refusal');
        } catch (MemlessRefusal $refusal) {
            $this->assertStringStartsWith('no file at path', $refusal->getMessage());
        }
        $this->assertSame([['name' => 'Ada']], $instance->query("SELECT name FROM users WHERE id = '01H7B2'"));
        $instance->release();
    }

    public function testAReloadOnAReleasedHandleIsAFault(): void
    {
        [$instance] = $this->loadCopy('start.yaml');
        $instance->release();
        try {
            $instance->reload();
            $this->fail('expected a MemlessFault');
        } catch (MemlessFault $fault) {
            $this->assertSame(2, $fault->status);
        }
    }
}
