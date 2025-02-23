use std::borrow::Cow;

/// A toolchain component
///
/// # Reading materials
///
/// - [`rustup component history`]
///
/// [`rustup concepts: components`]: https://rust-lang.github.io/rustup/concepts/components.html
/// [`rustup component history`]: https://rust-lang.github.io/rustup-components-history/
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Component {
    name: Cow<'static, str>,
}

impl Component {
    /// Create a new Component instance
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self { name: name.into() }
    }

    /// The name of the component
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_instance() {
        let c = Component::new("sample");
        assert_eq!(c.name(), "sample");
    }
}
