use std::borrow::Cow;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Component {
    pub name: Cow<'static, str>,
}

impl Component {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self { name: name.into() }
    }
}
