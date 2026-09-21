<?php

declare(strict_types=1);

namespace Memless;

/**
 * A live memless instance behind an opaque handle. Translates the C ABI status
 * into a return value or a thrown error, and releases the handle once.
 */
final class Instance
{
    private const STATUS_OK = 0;

    private const STATUS_REFUSED = 1;

    private int $handle;

    private bool $released = false;

    private function __construct(int $handle)
    {
        $this->handle = $handle;
    }

    public static function load(string $path): self
    {
        if (strpos($path, "\0") !== false) {
            throw new \InvalidArgumentException('path contains a NUL byte');
        }

        [$status, $handle, $message] = Call::load(Library::ffi(), $path);
        if ($status === self::STATUS_OK) {
            return new self($handle);
        }

        if ($status === self::STATUS_REFUSED) {
            throw new MemlessRefusal($message);
        }

        throw new \LogicException("memless load fault ({$status}): {$message}");
    }

    public function release(): void
    {
        if ($this->released) {
            return;
        }

        $this->released = true;
        Library::ffi()->memless_release($this->handle);
    }

    public function __destruct()
    {
        $this->release();
    }
}
