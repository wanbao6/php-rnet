# php-rnet

HTTP client for PHP with browser TLS/H2 fingerprinting. Built in Rust on top of [wreq](https://github.com/0x676e67/wreq) (BoringSSL).

Lets you make HTTP requests that look like they came from a real browser at the TLS/JA3/H2 level — useful when dealing with sites that fingerprint clients.

---

## Building from source

### System requirements

- Linux x86_64
- PHP 8.1 or newer (with development headers)
- Rust 1.85 or newer
- cmake 3.14 or newer (to compile BoringSSL)
- libclang 18 (used at build time only by bindgen — not needed at runtime)
- A C compiler: gcc or clang

> **Note:** libclang is only needed during `cargo build`. The compiled `.so` file has no dependency on it.

---

### Step 1 — Install Rust

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Verify:

```bash
rustc --version   # should be 1.85 or newer
cargo --version
```

---

### Step 2 — Install system dependencies

**Debian / Ubuntu:**

```bash
sudo apt update
sudo apt install build-essential cmake pkg-config libclang-18-dev
```

If `libclang-18-dev` is not available in your repo (e.g. Ubuntu 22.04), add the LLVM apt repository first:

```bash
wget -qO- https://apt.llvm.org/llvm-snapshot.gpg.key | sudo tee /etc/apt/trusted.gpg.d/apt.llvm.org.asc
echo "deb http://apt.llvm.org/jammy/ llvm-toolchain-jammy-18 main" | sudo tee /etc/apt/sources.list.d/llvm-18.list
sudo apt update
sudo apt install libclang-18-dev
```

Replace `jammy` with your Ubuntu codename (`focal`, `noble`, etc.) if needed.

**Fedora / RHEL:**

```bash
sudo dnf install gcc cmake clang-devel
```

**Arch Linux:**

```bash
sudo pacman -S base-devel cmake clang
```

---

### Step 3 — Install PHP development headers

You need the PHP headers for the version you want to target.

**Debian / Ubuntu:**

```bash
# PHP 8.4
sudo apt install php8.4-dev

# or if you only have a generic package available
sudo apt install php-dev
```

**Fedora / RHEL:**

```bash
sudo dnf install php-devel
```

**Arch Linux:**

```bash
sudo pacman -S php
```

Verify that `php-config` works and points to your intended PHP version:

```bash
php-config --version
```

If you have multiple PHP versions installed and `php-config` points to the wrong one, set it explicitly:

```bash
export PHP_CONFIG=/usr/bin/php-config8.4
```

---

### Step 4 — Set environment variables

ext-php-rs uses bindgen to parse PHP headers at build time. You need to tell it where libclang is:

```bash
# Adjust the path to match your system
export LIBCLANG_PATH=/usr/lib/llvm-18/lib

# Only needed if libclang is not in a standard location
export LD_LIBRARY_PATH="$LIBCLANG_PATH:$LD_LIBRARY_PATH"
```

Check where libclang was installed:

```bash
find /usr -name "libclang.so*" 2>/dev/null
```

If cmake is not in your PATH (e.g. installed to a custom location), add it:

```bash
export PATH="/path/to/cmake/bin:$PATH"
```

---

### Step 5 — Build

Clone the repository and build:

```bash
git clone https://github.com/yourname/php-rnet.git
cd php-rnet
cargo build --release
```

The first build will take a few minutes because it compiles BoringSSL from source. Subsequent builds are much faster.

The output file is:

```
target/release/librnet.so
```

---

### Step 6 — Load the extension

**Permanently (add to php.ini):**

Find your php.ini:

```bash
php --ini
```

Add the following line:

```ini
extension=/absolute/path/to/librnet.so
```

**Per script (without modifying php.ini):**

```bash
php -d extension=/absolute/path/to/librnet.so your_script.php
```

**Verify it loaded:**

```bash
php -d extension=/absolute/path/to/librnet.so -r "var_dump(extension_loaded('php-rnet'));"
# bool(true)
```

---

### Troubleshooting

**`error: could not find tool "cc"`**

Your C compiler is not in `PATH`. Install gcc and make sure `/usr/bin` is in `PATH`:

```bash
sudo apt install gcc
export PATH="/usr/bin:$PATH"
```

**`could not find libclang`**

bindgen cannot find libclang. Set `LIBCLANG_PATH` to the directory containing `libclang.so`:

```bash
find / -name "libclang.so*" 2>/dev/null
export LIBCLANG_PATH=/usr/lib/llvm-18/lib
```

**`Error: Could not find PHP executable`**

ext-php-rs needs the `php` binary to detect build flags. Make sure `php` is in your PATH:

```bash
which php
# if missing:
sudo apt install php-cli
```

**`php-config: not found`**

The PHP dev package is not installed, or `php-config` is versioned:

```bash
sudo apt install php-dev
# or set it manually:
export PHP_CONFIG=/usr/bin/php-config8.4
```

**cmake version too old**

BoringSSL requires cmake 3.14+. Check your version:

```bash
cmake --version
```

If too old, install a newer one via [cmake.org](https://cmake.org/download/) or mise:

```bash
mise install cmake@latest
export PATH="$HOME/.local/share/mise/installs/cmake/latest/bin:$PATH"
```

---

## Usage

### Quick functions

```php
$resp = RNet\get('https://example.com');
echo $resp->status();  // 200
echo $resp->text();

$resp = RNet\post('https://httpbin.org/post', [
    'json' => ['key' => 'value'],
]);
$data = $resp->json();
```

### Reusable client

Builder methods return void and cannot be chained. Call them separately:

```php
$b = new RNet\ClientBuilder();
$b->impersonate(RNet\Emulation::CHROME_136);
$b->timeout(30.0);
$client = $b->build();

$resp = $client->get('https://httpbin.org/get');
```

### Request options

All request methods accept an optional array as the second argument:

```php
$resp = $client->post('https://httpbin.org/post', [
    'headers' => ['X-Custom' => 'value'],
    'query'   => ['page' => '2'],
    'json'    => ['name' => 'test'],   // sets Content-Type: application/json
    'form'    => ['field' => 'value'], // sets Content-Type: application/x-www-form-urlencoded
    'body'    => 'raw string body',
    'timeout' => 10.0,
]);
```

### Response

```php
$resp->status();       // int, e.g. 200
$resp->ok();           // bool, true if 2xx
$resp->text();         // body as string
$resp->json();         // body decoded as PHP array
$resp->bytes();        // body as raw bytes
$resp->headers();      // associative array of all headers
$resp->header('content-type'); // single header (case-insensitive)
$resp->cookies();      // array of Cookie objects
$resp->url();          // final URL after redirects
$resp->remoteAddr();   // server IP:port

$resp->raiseForStatus(); // throws StatusException on 4xx/5xx
```

### Browser emulation

```php
$b = new RNet\ClientBuilder();
$b->impersonate(RNet\Emulation::CHROME_136);
$client = $b->build();
```

Available profiles include Chrome, Firefox, Safari, Edge, Opera, and OkHttp. See `RNet\Emulation` for the full list.

### TLS / certificates

```php
$b = new RNet\ClientBuilder();
$b->verify(false);
$client = $b->build();
```

### Proxy

```php
$proxy = RNet\Proxy::all('socks5://127.0.0.1:1080');
// or RNet\Proxy::http(...) / RNet\Proxy::https(...)

$b = new RNet\ClientBuilder();
$b->proxy($proxy);
$client = $b->build();
```

### Cookies

```php
$b = new RNet\ClientBuilder();
$b->cookieStore(true);
$client = $b->build();

// Cookies are persisted across requests on the same client instance.
```

### Other options

```php
$b = new RNet\ClientBuilder();
$b->timeout(30.0);           // total request timeout in seconds
$b->connectTimeout(5.0);     // connection timeout
$b->maxRedirects(5);         // 0 to disable redirects
$b->http1Only(true);         // disable HTTP/2
$b->userAgent('MyBot/1.0');
$b->defaultHeader('Accept-Language', 'en-US');
$client = $b->build();
```

### Error handling

```php
try {
    $resp = RNet\get('https://example.com');
    $resp->raiseForStatus();
} catch (RNet\TimeoutException $e) {
    // request timed out
} catch (RNet\ConnectionException $e) {
    // could not connect
} catch (RNet\StatusException $e) {
    // 4xx or 5xx response
} catch (RNet\RequestException $e) {
    // other request error
}
```

## License

Apache-2.0
