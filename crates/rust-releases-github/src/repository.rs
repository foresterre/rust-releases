use std::borrow::Cow;
use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Repository {
    owner: Cow<'static, str>,
    name: Cow<'static, str>,
}

impl Repository {
    pub const RUST_LANG_RUST: Self = Self {
        owner: Cow::Borrowed("rust-lang"),
        name: Cow::Borrowed("rust"),
    };

    pub fn new(owner: impl Into<Cow<'static, str>>, name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            owner: owner.into(),
            name: name.into(),
        }
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for Repository {
    fn default() -> Self {
        Self::RUST_LANG_RUST
    }
}

impl fmt::Display for Repository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_the_rust_repository() {
        assert_eq!(Repository::default(), Repository::RUST_LANG_RUST);
        assert_eq!(Repository::default().to_string(), "rust-lang/rust");
    }

    #[test]
    fn custom_repo() {
        let repository = Repository::new("foresterre".to_string(), "rust-releases");

        assert_eq!(repository.owner(), "foresterre");
        assert_eq!(repository.name(), "rust-releases");
        assert_eq!(repository.to_string(), "foresterre/rust-releases");
    }
}
