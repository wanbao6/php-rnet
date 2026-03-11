  <?php
  // php -d extension=./target/release/librnet.so tests/check_fingerprint.php

  $profiles = [
      'Chrome136'  => RNet\Emulation::CHROME_136,
      'Firefox139' => RNet\Emulation::FIREFOX_139,
      'Safari26'   => RNet\Emulation::SAFARI_26,
      'No profile' => null,
  ];

  foreach ($profiles as $name => $profile) {
      $b = new RNet\ClientBuilder();
      if ($profile !== null) {
          $b->impersonate($profile);
      }
      $client = $b->build();

      $resp = $client->get('https://tls.browserleaks.com/json');
      $data = $resp->json();

      echo "\n=== $name ===\n";
      echo "JA3  hash : " . ($data['ja3_hash']  ?? 'n/a') . "\n";
      echo "JA4  hash : " . ($data['ja4']        ?? 'n/a') . "\n";
      echo "HTTP/2    : " . ($data['http2_hash'] ?? 'n/a') . "\n";
      echo "Akamai h2 : " . ($data['akamai_hash'] ?? 'n/a') . "\n";
  }
