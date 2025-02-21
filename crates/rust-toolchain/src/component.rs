use std::borrow::Cow;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Component {
    name: Cow<'static, str>,
}

impl Component {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self { name: name.into() }
    }

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
