//! php-rnet: High-performance PHP HTTP client with browser TLS/H2 fingerprinting.
//!
//! Built on top of `wreq` (BoringSSL-based, 100+ browser profiles) and exposed
//! to PHP via `ext-php-rs`.
//!
//! # Usage
//! ```php
//! // Quick functions
//! $resp = RNet\get('https://httpbin.org/get');
//! echo $resp->status();   // 200
//! echo $resp->text();
//!
//! // Reusable client with browser emulation
//! $client = (new RNet\ClientBuilder())
//!     ->impersonate(RNet\Emulation::CHROME_136)
//!     ->timeout(30.0)
//!     ->build();
//!
//! $resp = $client->post('https://httpbin.org/post', [
//!     'json' => ['hello' => 'world'],
//! ]);
//! $data = $resp->json();
//! ```

#![allow(clippy::new_without_default)]

#[cfg(feature = "mimalloc")]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::sync::Arc;

use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
use once_cell::sync::OnceCell;

pub mod client;
pub mod cookie;
pub mod emulation;
pub mod error;
pub mod proxy;
pub mod response;

use client::{Client, ClientBuilder};
use cookie::Cookie;
use emulation::Emulation;
use error::{
    BodyException, ConnectionException, DecodingException, RedirectException, RequestException,
    StatusException, TimeoutException, TlsException, WebSocketException,
};
use proxy::Proxy;
use response::Response;

// ---------- default client (for top-level functions) ----------

static DEFAULT_CLIENT: OnceCell<Arc<Client>> = OnceCell::new();

fn default_client() -> PhpResult<Arc<Client>> {
    DEFAULT_CLIENT
        .get_or_try_init(|| {
            let b = ClientBuilder::__construct();
            Client::from_builder(&b).map(Arc::new)
        })
        .map(Arc::clone)
}

// ---------- top-level convenience functions ----------

/// Send a GET request using the default client.
#[php_function]
#[php(name = "RNet\\get")]
pub fn get(url: &str, options: Option<&ZendHashTable>) -> PhpResult<Response> {
    default_client()?.get(url, options)
}

/// Send a POST request using the default client.
#[php_function]
#[php(name = "RNet\\post")]
pub fn post(url: &str, options: Option<&ZendHashTable>) -> PhpResult<Response> {
    default_client()?.post(url, options)
}

/// Send a PUT request using the default client.
#[php_function]
#[php(name = "RNet\\put")]
pub fn put(url: &str, options: Option<&ZendHashTable>) -> PhpResult<Response> {
    default_client()?.put(url, options)
}

/// Send a PATCH request using the default client.
#[php_function]
#[php(name = "RNet\\patch")]
pub fn patch(url: &str, options: Option<&ZendHashTable>) -> PhpResult<Response> {
    default_client()?.patch(url, options)
}

/// Send a DELETE request using the default client.
#[php_function]
#[php(name = "RNet\\delete")]
pub fn delete(url: &str, options: Option<&ZendHashTable>) -> PhpResult<Response> {
    default_client()?.delete(url, options)
}

/// Send a HEAD request using the default client.
#[php_function]
#[php(name = "RNet\\head")]
pub fn head(url: &str, options: Option<&ZendHashTable>) -> PhpResult<Response> {
    default_client()?.head(url, options)
}

/// Send a request with a custom HTTP method using the default client.
#[php_function]
#[php(name = "RNet\\request")]
pub fn request(
    method: &str,
    url: &str,
    options: Option<&ZendHashTable>,
) -> PhpResult<Response> {
    default_client()?.request(method, url, options)
}

// ---------- module registration ----------

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        // classes
        .class::<ClientBuilder>()
        .class::<Client>()
        .class::<Cookie>()
        .class::<Emulation>()
        .class::<Proxy>()
        .class::<Response>()
        // exception classes
        .class::<RequestException>()
        .class::<ConnectionException>()
        .class::<TlsException>()
        .class::<TimeoutException>()
        .class::<StatusException>()
        .class::<BodyException>()
        .class::<DecodingException>()
        .class::<RedirectException>()
        .class::<WebSocketException>()
        // top-level functions
        .function(wrap_function!(get))
        .function(wrap_function!(post))
        .function(wrap_function!(put))
        .function(wrap_function!(patch))
        .function(wrap_function!(delete))
        .function(wrap_function!(head))
        .function(wrap_function!(request))
}
