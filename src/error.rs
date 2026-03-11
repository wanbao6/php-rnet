use ext_php_rs::exception::PhpException;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::ce;

// ---------- PHP exception classes ----------

#[php_class]
#[php(name = "RNet\\RequestException", extends(ce = ce::exception, stub = "\\Exception"))]
pub struct RequestException;

#[php_impl]
impl RequestException {}

#[php_class]
#[php(name = "RNet\\ConnectionException", extends(ce = ce::exception, stub = "\\Exception"))]
pub struct ConnectionException;

#[php_impl]
impl ConnectionException {}

#[php_class]
#[php(name = "RNet\\TlsException", extends(ce = ce::exception, stub = "\\Exception"))]
pub struct TlsException;

#[php_impl]
impl TlsException {}

#[php_class]
#[php(name = "RNet\\TimeoutException", extends(ce = ce::exception, stub = "\\Exception"))]
pub struct TimeoutException;

#[php_impl]
impl TimeoutException {}

#[php_class]
#[php(name = "RNet\\StatusException", extends(ce = ce::exception, stub = "\\Exception"))]
pub struct StatusException;

#[php_impl]
impl StatusException {}

#[php_class]
#[php(name = "RNet\\BodyException", extends(ce = ce::exception, stub = "\\Exception"))]
pub struct BodyException;

#[php_impl]
impl BodyException {}

#[php_class]
#[php(name = "RNet\\DecodingException", extends(ce = ce::exception, stub = "\\Exception"))]
pub struct DecodingException;

#[php_impl]
impl DecodingException {}

#[php_class]
#[php(name = "RNet\\RedirectException", extends(ce = ce::exception, stub = "\\Exception"))]
pub struct RedirectException;

#[php_impl]
impl RedirectException {}

#[php_class]
#[php(name = "RNet\\WebSocketException", extends(ce = ce::exception, stub = "\\Exception"))]
pub struct WebSocketException;

#[php_impl]
impl WebSocketException {}

// ---------- unified error type ----------

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request error: {0}")]
    Request(#[from] wreq::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("url-encoded error: {0}")]
    UrlEncoded(#[from] serde_urlencoded::ser::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(String),
}

impl Error {
    pub fn other(msg: impl Into<String>) -> Self {
        Error::Other(msg.into())
    }
}

impl From<Error> for PhpException {
    fn from(e: Error) -> Self {
        match e {
            Error::Request(ref re) => {
                if re.is_timeout() {
                    PhpException::from_class::<TimeoutException>(e.to_string())
                } else if re.is_connect() {
                    PhpException::from_class::<ConnectionException>(e.to_string())
                } else if re.is_body() {
                    PhpException::from_class::<BodyException>(e.to_string())
                } else if re.is_decode() {
                    PhpException::from_class::<DecodingException>(e.to_string())
                } else if re.is_redirect() {
                    PhpException::from_class::<RedirectException>(e.to_string())
                } else if re.is_status() {
                    PhpException::from_class::<StatusException>(e.to_string())
                } else {
                    PhpException::from_class::<RequestException>(e.to_string())
                }
            }
            Error::Json(_) | Error::UrlEncoded(_) => {
                PhpException::from_class::<RequestException>(e.to_string())
            }
            Error::Io(_) => PhpException::from_class::<ConnectionException>(e.to_string()),
            Error::Other(msg) => PhpException::from_class::<RequestException>(msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
