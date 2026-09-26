<?php

declare(strict_types=1);

namespace Memless\Tests;

use Memless\Bundle;
use Memless\LibrarySearch;
use PHPUnit\Framework\TestCase;

final class LibrarySearchTest extends TestCase
{
    private const BUNDLE = 'darwin-arm64/libmemless_capi.dylib';

    private string $package;

    private string $workspace;

    protected function setUp(): void
    {
        $base = sys_get_temp_dir() . '/memless-search-' . bin2hex(random_bytes(6));
        $this->package = "{$base}/package";
        $this->workspace = "{$base}/workspace";
    }

    protected function tearDown(): void
    {
        $base = dirname($this->package);
        if (!is_dir($base)) {
            return;
        }
        $entries = new \RecursiveIteratorIterator(
            new \RecursiveDirectoryIterator($base, \FilesystemIterator::SKIP_DOTS),
            \RecursiveIteratorIterator::CHILD_FIRST,
        );
        foreach ($entries as $entry) {
            $entry->isDir() ? rmdir($entry->getPathname()) : unlink($entry->getPathname());
        }
        rmdir($base);
    }

    private function write(string $file): string
    {
        if (!is_dir(dirname($file))) {
            mkdir(dirname($file), 0777, true);
        }
        file_put_contents($file, 'library');

        return $file;
    }

    private function search(): LibrarySearch
    {
        return new LibrarySearch($this->package, $this->workspace);
    }

    public function testMemlessLibWinsOverTheBundledLibraryAndTarget(): void
    {
        $this->write("{$this->package}/lib/" . self::BUNDLE);
        $this->write("{$this->workspace}/target/release/libmemless_capi.dylib");
        $env = $this->write("{$this->workspace}/elsewhere/libmemless_capi.dylib");

        $this->assertSame($env, $this->search()->library($env, self::BUNDLE));
    }

    public function testTheBundledLibraryComesBeforeTarget(): void
    {
        $bundled = $this->write("{$this->package}/lib/" . self::BUNDLE);
        $this->write("{$this->workspace}/target/release/libmemless_capi.dylib");

        $this->assertSame($bundled, $this->search()->library(null, self::BUNDLE));
    }

    public function testAPlatformWithoutABundledLibraryFallsBackToTarget(): void
    {
        $this->write("{$this->package}/lib/" . self::BUNDLE);
        $release = $this->write("{$this->workspace}/target/release/libmemless_capi.dylib");

        $this->assertSame($release, $this->search()->library(null, null));
    }

    public function testAMissingBundledLibraryFallsBackToTarget(): void
    {
        $release = $this->write("{$this->workspace}/target/release/libmemless_capi.so");

        $this->assertSame($release, $this->search()->library(null, 'linux-x64-gnu/libmemless_capi.so'));
    }

    public function testReleaseComesBeforeDebug(): void
    {
        $this->write("{$this->workspace}/target/debug/libmemless_capi.dylib");
        $release = $this->write("{$this->workspace}/target/release/libmemless_capi.dylib");

        $this->assertSame($release, $this->search()->library(null, null));
    }

    public function testNoLibraryAnywhereNamesMemlessLib(): void
    {
        $this->expectException(\LogicException::class);
        $this->expectExceptionMessageMatches('/set MEMLESS_LIB/');
        $this->search()->library(null, self::BUNDLE);
    }

    public function testTheBundledHeaderComesBeforeTheWorkspaceHeader(): void
    {
        $bundled = $this->write("{$this->package}/lib/memless.h");
        $this->write("{$this->workspace}/crates/memless-capi/include/memless.h");

        $this->assertSame($bundled, $this->search()->header(null));
    }

    public function testMemlessHeaderWinsOverTheBundledHeader(): void
    {
        $this->write("{$this->package}/lib/memless.h");
        $env = $this->write("{$this->workspace}/elsewhere/memless.h");

        $this->assertSame($env, $this->search()->header($env));
    }

    public function testTheFourPublishedPlatformsHaveABundleAndTheOthersNone(): void
    {
        $this->assertSame('darwin-arm64/libmemless_capi.dylib', Bundle::library('Darwin', 'arm64', false));
        $this->assertSame('darwin-x64/libmemless_capi.dylib', Bundle::library('Darwin', 'x86_64', false));
        $this->assertSame('linux-x64-gnu/libmemless_capi.so', Bundle::library('Linux', 'x86_64', false));
        $this->assertSame('linux-arm64-gnu/libmemless_capi.so', Bundle::library('Linux', 'aarch64', false));
        $this->assertNull(Bundle::library('Linux', 'x86_64', true));
        $this->assertNull(Bundle::library('Windows', 'AMD64', false));
    }
}
