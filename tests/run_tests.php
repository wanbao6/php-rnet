<?php

/**
 * php-rnet integration tests.
 *
 * Run with:
 *   php -d extension=./target/release/librnet.so tests/run_tests.php
 *
 * Requires network access (uses httpbin.org).
 * Exit code 0 = all passed, 1 = failure.
 */

declare(strict_types=1);

// ---------------------------------------------------------------------------
// Minimal test harness
// ---------------------------------------------------------------------------

$passed = 0;
$failed = 0;

function test(string $name, callable $fn): void {
    global $passed, $failed;
    try {
        $fn();
        echo "  PASS  $name\n";
        $passed++;
    } catch (Throwable $e) {
        echo "  FAIL  $name\n";
        echo "        " . $e->getMessage() . "\n";
        $failed++;
    }
}

function assert_eq(mixed $actual, mixed $expected, string $msg = ''): void {
    if ($actual !== $expected) {
        $a = var_export($actual, true);
        $e = var_export($expected, true);
        throw new RuntimeException("assert_eq failed{$msg}: got $a, expected $e");
    }
}

function assert_true(mixed $value, string $msg = ''): void {
    if (!$value) {
        throw new RuntimeException("assert_true failed: $msg");
    }
}

function assert_false(mixed $value, string $msg = ''): void {
    if ($value) {
        throw new RuntimeException("assert_false failed: $msg");
    }
}

function assert_throws(string $class, callable $fn): void {
    try {
        $fn();
        throw new RuntimeException("expected $class to be thrown, but nothing was thrown");
    } catch (Throwable $e) {
        if (!($e instanceof $class)) {
            throw new RuntimeException(
                "expected $class, got " . get_class($e) . ': ' . $e->getMessage()
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Guard: extension must be loaded
// ---------------------------------------------------------------------------

if (!extension_loaded('php-rnet')) {
    echo "ERROR: php-rnet extension is not loaded.\n";
    echo "Run: php -d extension=./target/release/librnet.so tests/run_tests.php\n";
    exit(1);
}

// ---------------------------------------------------------------------------
// 1. Extension / class existence
// ---------------------------------------------------------------------------

echo "\n--- Extension presence ---\n";

test('extension_loaded returns true', function () {
    assert_true(extension_loaded('php-rnet'));
});

test('RNet\\ClientBuilder class exists', function () {
    assert_true(class_exists('RNet\\ClientBuilder'));
});

test('RNet\\Client class exists', function () {
    assert_true(class_exists('RNet\\Client'));
});

test('RNet\\Response class exists', function () {
    assert_true(class_exists('RNet\\Response'));
});

test('RNet\\Cookie class exists', function () {
    assert_true(class_exists('RNet\\Cookie'));
});

test('RNet\\Proxy class exists', function () {
    assert_true(class_exists('RNet\\Proxy'));
});

test('RNet\\Emulation class exists', function () {
    assert_true(class_exists('RNet\\Emulation'));
});

test('top-level RNet\\get function exists', function () {
    assert_true(function_exists('RNet\\get'));
});

// ---------------------------------------------------------------------------
// 2. Emulation constants
// ---------------------------------------------------------------------------

echo "\n--- Emulation constants ---\n";

test('CHROME_136 constant is defined', function () {
    assert_eq(RNet\Emulation::CHROME_136, 'Chrome136');
});

test('FIREFOX_139 constant is defined', function () {
    assert_eq(RNet\Emulation::FIREFOX_139, 'Firefox139');
});

test('SAFARI_26 constant is defined', function () {
    assert_eq(RNet\Emulation::SAFARI_26, 'Safari26');
});

test('EDGE_137 constant is defined', function () {
    assert_eq(RNet\Emulation::EDGE_137, 'Edge137');
});

// ---------------------------------------------------------------------------
// 3. ClientBuilder + Client construction
// ---------------------------------------------------------------------------

echo "\n--- ClientBuilder ---\n";

test('ClientBuilder constructs without error', function () {
    $b = new RNet\ClientBuilder();
    assert_true($b instanceof RNet\ClientBuilder);
});

test('build() returns a Client', function () {
    $client = (new RNet\ClientBuilder())->build();
    assert_true($client instanceof RNet\Client);
});

test('impersonate() with valid profile does not throw', function () {
    $b = new RNet\ClientBuilder();
    $b->impersonate(RNet\Emulation::CHROME_136);
    $client = $b->build();
    assert_true($client instanceof RNet\Client);
});

test('impersonate() with unknown profile throws', function () {
    assert_throws(Exception::class, function () {
        $b = new RNet\ClientBuilder();
        $b->impersonate('NotABrowser999');
    });
});

test('verify(false) builds without error', function () {
    $b = new RNet\ClientBuilder();
    $b->verify(false);
    $client = $b->build();
    assert_true($client instanceof RNet\Client);
});

test('timeout(30.0) builds without error', function () {
    $b = new RNet\ClientBuilder();
    $b->timeout(30.0);
    $client = $b->build();
    assert_true($client instanceof RNet\Client);
});

test('maxRedirects(0) builds without error', function () {
    $b = new RNet\ClientBuilder();
    $b->maxRedirects(0);
    $client = $b->build();
    assert_true($client instanceof RNet\Client);
});

test('cookieStore(true) builds without error', function () {
    $b = new RNet\ClientBuilder();
    $b->cookieStore(true);
    $client = $b->build();
    assert_true($client instanceof RNet\Client);
});

test('http1Only(true) builds without error', function () {
    $b = new RNet\ClientBuilder();
    $b->http1Only(true);
    $client = $b->build();
    assert_true($client instanceof RNet\Client);
});

test('defaultHeader() builds without error', function () {
    $b = new RNet\ClientBuilder();
    $b->defaultHeader('X-Test', 'value');
    $client = $b->build();
    assert_true($client instanceof RNet\Client);
});

test('userAgent() builds without error', function () {
    $b = new RNet\ClientBuilder();
    $b->userAgent('TestAgent/1.0');
    $client = $b->build();
    assert_true($client instanceof RNet\Client);
});

// ---------------------------------------------------------------------------
// 4. Proxy construction
// ---------------------------------------------------------------------------

echo "\n--- Proxy ---\n";

test('Proxy::all() with socks5 URL', function () {
    $p = RNet\Proxy::all('socks5://127.0.0.1:1080');
    assert_true($p instanceof RNet\Proxy);
});

test('Proxy::http() with HTTP URL', function () {
    $p = RNet\Proxy::http('http://127.0.0.1:8080');
    assert_true($p instanceof RNet\Proxy);
});

test('Proxy::https() with HTTPS URL', function () {
    $p = RNet\Proxy::https('https://127.0.0.1:8080');
    assert_true($p instanceof RNet\Proxy);
});

test('Proxy::all() with invalid URL throws', function () {
    assert_throws(Exception::class, function () {
        RNet\Proxy::all('not-a-url');
    });
});

// ---------------------------------------------------------------------------
// 5. HTTP requests (requires network)
// ---------------------------------------------------------------------------

echo "\n--- HTTP requests (httpbin.org) ---\n";

test('GET request returns 200', function () {
    $resp = RNet\get('https://httpbin.org/get');
    assert_eq($resp->status(), 200);
    assert_true($resp->ok());
});

test('GET response has text body', function () {
    $resp = RNet\get('https://httpbin.org/get');
    $text = $resp->text();
    assert_true(is_string($text) && strlen($text) > 0);
});

test('GET response json() returns array', function () {
    $resp = RNet\get('https://httpbin.org/get');
    $data = $resp->json();
    assert_true(is_array($data), 'expected array from json()');
    assert_true(isset($data['url']), 'expected url key in response');
});

test('POST with json body is received by server', function () {
    $resp = RNet\post('https://httpbin.org/post', [
        'json' => ['hello' => 'world'],
    ]);
    assert_eq($resp->status(), 200);
    $data = $resp->json();
    assert_eq($data['json']['hello'] ?? null, 'world');
});

test('POST with form body is received by server', function () {
    $resp = RNet\post('https://httpbin.org/post', [
        'form' => ['key' => 'value'],
    ]);
    assert_eq($resp->status(), 200);
    $data = $resp->json();
    assert_eq($data['form']['key'] ?? null, 'value');
});

test('PUT request returns 200', function () {
    $resp = RNet\put('https://httpbin.org/put');
    assert_eq($resp->status(), 200);
});

test('PATCH request returns 200', function () {
    $resp = RNet\patch('https://httpbin.org/patch');
    assert_eq($resp->status(), 200);
});

test('DELETE request returns 200', function () {
    $resp = RNet\delete('https://httpbin.org/delete');
    assert_eq($resp->status(), 200);
});

test('custom method via RNet\\request()', function () {
    $resp = RNet\request('GET', 'https://httpbin.org/get');
    assert_eq($resp->status(), 200);
});

test('query params are sent', function () {
    $resp = RNet\get('https://httpbin.org/get', [
        'query' => ['foo' => 'bar'],
    ]);
    $data = $resp->json();
    assert_eq($data['args']['foo'] ?? null, 'bar');
});

test('custom headers are sent', function () {
    $resp = RNet\get('https://httpbin.org/headers', [
        'headers' => ['X-Custom-Header' => 'test-value'],
    ]);
    $data = $resp->json();
    $headers = $data['headers'] ?? [];
    assert_eq($headers['X-Custom-Header'] ?? null, 'test-value');
});

test('response url() returns a string', function () {
    $resp = RNet\get('https://httpbin.org/get');
    assert_true(is_string($resp->url()) && strlen($resp->url()) > 0);
});

test('response headers() returns associative array', function () {
    $resp = RNet\get('https://httpbin.org/get');
    $headers = $resp->headers();
    assert_true(is_array($headers) && count($headers) > 0);
});

test('response header() case-insensitive lookup', function () {
    $resp = RNet\get('https://httpbin.org/get');
    $ct = $resp->header('content-type') ?? $resp->header('Content-Type');
    assert_true($ct !== null, 'expected content-type header');
});

// ---------------------------------------------------------------------------
// 6. Status helpers
// ---------------------------------------------------------------------------

echo "\n--- Status helpers ---\n";

test('ok() true for 200', function () {
    $resp = RNet\get('https://httpbin.org/status/200');
    assert_true($resp->ok());
});

test('ok() false for 404', function () {
    $resp = RNet\get('https://httpbin.org/status/404');
    assert_false($resp->ok());
});

test('raiseForStatus() passes on 200', function () {
    $resp = RNet\get('https://httpbin.org/status/200');
    $resp->raiseForStatus(); // should not throw
    assert_true(true);
});

test('raiseForStatus() throws StatusException on 404', function () {
    assert_throws(RNet\StatusException::class, function () {
        $resp = RNet\get('https://httpbin.org/status/404');
        $resp->raiseForStatus();
    });
});

test('raiseForStatus() throws StatusException on 500', function () {
    assert_throws(RNet\StatusException::class, function () {
        $resp = RNet\get('https://httpbin.org/status/500');
        $resp->raiseForStatus();
    });
});

// ---------------------------------------------------------------------------
// 7. Exception hierarchy
// ---------------------------------------------------------------------------

echo "\n--- Exception hierarchy ---\n";

test('ConnectionException extends Exception', function () {
    $r = new ReflectionClass(RNet\ConnectionException::class);
    assert_true($r->isSubclassOf(Exception::class));
});

test('TimeoutException extends Exception', function () {
    $r = new ReflectionClass(RNet\TimeoutException::class);
    assert_true($r->isSubclassOf(Exception::class));
});

test('StatusException extends Exception', function () {
    $r = new ReflectionClass(RNet\StatusException::class);
    assert_true($r->isSubclassOf(Exception::class));
});

// ---------------------------------------------------------------------------
// 8. Browser emulation via Client
// ---------------------------------------------------------------------------

echo "\n--- Browser emulation ---\n";

test('Chrome136 emulation request succeeds', function () {
    $b = new RNet\ClientBuilder();
    $b->impersonate(RNet\Emulation::CHROME_136);
    $b->timeout(30.0);
    $client = $b->build();
    $resp = $client->get('https://httpbin.org/get');
    assert_eq($resp->status(), 200);
});

test('Firefox139 emulation request succeeds', function () {
    $b = new RNet\ClientBuilder();
    $b->impersonate(RNet\Emulation::FIREFOX_139);
    $b->timeout(30.0);
    $client = $b->build();
    $resp = $client->get('https://httpbin.org/get');
    assert_eq($resp->status(), 200);
});

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

echo "\n";
echo str_repeat('-', 40) . "\n";
echo "Results: $passed passed, $failed failed\n";

exit($failed > 0 ? 1 : 0);
