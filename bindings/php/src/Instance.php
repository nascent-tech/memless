<?php

declare(strict_types=1);

namespace Memless;

/**
 * A live memless instance behind an opaque handle. Loading, querying and the
 * release call live in Loader, Query and Native; this facade holds the handle.
 */
final class Instance
{
    private int $handle;

    private bool $released = false;

    private function __construct(int $handle)
    {
        $this->handle = $handle;
    }

    public static function load(string $path): self
    {
        return new self(Loader::open($path));
    }

    public function query(string $sql): array
    {
        return Query::run($this->handle, $sql);
    }

    public function __destruct()
    {
        $this->release();
    }

    public function release(): void
    {
        if ($this->released) {
            return;
        }

        $this->released = true;
        Native::release($this->handle);
    }
}
