use crate::URL;
use std::borrow::Cow;

/// URL to the RELEASES.md file
pub struct Url {
    url: Cow<'static, str>,
}

impl Default for Url {
    fn default() -> Self {
        Self {
            url: Cow::Borrowed(URL),
        }
    }
}

impl Url {
    /// HTTP `url` to the RELEASED.md file
    pub fn new(url: impl Into<Cow<'static, str>>) -> Self {
        Self { url: url.into() }
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        self.url.as_ref()
    }
}
