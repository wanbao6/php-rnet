<?php

if (!extension_loaded('php-rnet')) {
    die("rnet extension not loaded.\nRun: php -d extension=target/release/librnet.so examples/basic.php\n");
}

// --- Simple GET ---

$resp = RNet\get('https://httpbin.org/get');
echo "Status: " . $resp->status() . "\n";
echo "URL:    " . $resp->url() . "\n";

$data = $resp->json();
echo "Origin: " . $data['origin'] . "\n\n";

// --- Browser fingerprinting ---
// Note: builder methods return void, so they cannot be chained.

$b = new RNet\ClientBuilder();
$b->impersonate(RNet\Emulation::CHROME_136);
$b->timeout(30.0);
$client = $b->build();

$resp = $client->get('https://httpbin.org/headers');
echo "Headers seen by server:\n";
foreach ($resp->json()['headers'] as $k => $v) {
    echo "  $k: $v\n";
}
echo "\n";

// --- POST with JSON ---

$resp = $client->post('https://httpbin.org/post', [
    'json' => ['hello' => 'world', 'number' => 42],
]);
$body = $resp->json();
echo "Posted JSON: " . json_encode($body['json']) . "\n\n";

// --- POST with form data ---

$resp = $client->post('https://httpbin.org/post', [
    'form' => ['field' => 'value', 'foo' => 'bar'],
]);
$body = $resp->json();
echo "Posted form: " . json_encode($body['form']) . "\n\n";

// --- Query parameters ---

$resp = $client->get('https://httpbin.org/get', [
    'query' => ['page' => '1', 'limit' => '10'],
]);
echo "Query args: " . json_encode($resp->json()['args']) . "\n\n";

// --- Error handling ---

try {
    $resp = $client->get('https://httpbin.org/status/404');
    $resp->raiseForStatus();
} catch (RNet\StatusException $e) {
    echo "Caught status error: " . $e->getMessage() . "\n";
}
