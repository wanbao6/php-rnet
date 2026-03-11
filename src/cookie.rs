use ext_php_rs::prelude::*;
use wreq::cookie::Cookie as WreqCookie;

/// A single HTTP cookie.
#[php_class]
#[php(name = "RNet\\Cookie")]
#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub http_only: bool,
    pub secure: bool,
}

#[php_impl]
impl Cookie {
    pub fn __construct(
        name: String,
        value: String,
        domain: Option<String>,
        path: Option<String>,
        http_only: Option<bool>,
        secure: Option<bool>,
    ) -> Self {
        Self {
            name,
            value,
            domain,
            path,
            http_only: http_only.unwrap_or(false),
            secure: secure.unwrap_or(false),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }

    pub fn get_domain(&self) -> Option<String> {
        self.domain.clone()
    }

    pub fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    pub fn is_http_only(&self) -> bool {
        self.http_only
    }

    pub fn is_secure(&self) -> bool {
        self.secure
    }

    pub fn __to_string(&self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

impl Cookie {
    pub fn from_wreq(c: &WreqCookie<'_>) -> Self {
        Self {
            name: c.name().to_owned(),
            value: c.value().to_owned(),
            domain: None,
            path: None,
            http_only: c.http_only(),
            secure: c.secure(),
        }
    }
}
