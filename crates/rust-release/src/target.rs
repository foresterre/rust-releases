use crate::component::Component;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Triple {
    triple: TargetTriple,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct TargetTriple;

pub struct Target {
    components: Vec<Component>,
}
