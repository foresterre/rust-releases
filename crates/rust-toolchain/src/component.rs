use std::borrow::Cow;
use std::collections::BTreeSet;
use std::slice;
use std::sync::Arc;

/// A toolchain component
///
/// # Reading materials
///
/// - [`rustup component history`]
///
/// [`rustup concepts: components`]: https://rust-lang.github.io/rustup/concepts/components.html
/// [`rustup component history`]: https://rust-lang.github.io/rustup-components-history/
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
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

/// A set of unique [`Component`]s, ordered by name.
///
/// The components are held behind an [`Arc`], so cloning a set, and with it cloning
/// the [`Toolchain`] which holds it, does not copy its components. Two toolchains
/// which ship the same components can share a single set.
///
/// [`Toolchain`]: crate::Toolchain
#[derive(Clone, Debug, Eq)]
pub struct ComponentSet {
    components: Arc<[Component]>,
}

impl ComponentSet {
    /// The amount of components in the set.
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns true if the set holds no components, and false otherwise.
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Returns true if the given component is a member of the set.
    pub fn contains(&self, component: &Component) -> bool {
        self.components.contains(component)
    }

    /// Iterate over the components of the set.
    pub fn iter(&self) -> slice::Iter<'_, Component> {
        self.components.iter()
    }

    /// The components of the set, as a slice ordered by name.
    pub fn as_slice(&self) -> &[Component] {
        &self.components
    }
}

impl Default for ComponentSet {
    fn default() -> Self {
        Self {
            components: Vec::new().into(),
        }
    }
}

impl PartialEq for ComponentSet {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.components, &other.components) || self.components == other.components
    }
}

impl FromIterator<Component> for ComponentSet {
    fn from_iter<I: IntoIterator<Item = Component>>(iter: I) -> Self {
        let unique = iter.into_iter().collect::<BTreeSet<_>>();

        Self {
            components: unique.into_iter().collect(),
        }
    }
}

impl From<Vec<Component>> for ComponentSet {
    fn from(components: Vec<Component>) -> Self {
        components.into_iter().collect()
    }
}

impl<'a> IntoIterator for &'a ComponentSet {
    type Item = &'a Component;
    type IntoIter = slice::Iter<'a, Component>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.iter()
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

    mod component_set {
        use super::*;
        use std::ptr;

        #[test]
        fn an_empty_set() {
            let set = ComponentSet::default();

            assert!(set.is_empty());
            assert_eq!(set.len(), 0);
        }

        #[test]
        fn a_set_is_ordered_by_name() {
            let set = [Component::new("rustc"), Component::new("cargo")]
                .into_iter()
                .collect::<ComponentSet>();

            let names = set.iter().map(|c| c.name()).collect::<Vec<_>>();

            assert_eq!(names, ["cargo", "rustc"]);
        }

        #[test]
        fn a_set_holds_every_component_once() {
            let set = [Component::new("cargo"), Component::new("cargo")]
                .into_iter()
                .collect::<ComponentSet>();

            assert_eq!(set.len(), 1);
            assert!(set.contains(&Component::new("cargo")));
        }

        #[test]
        fn a_set_is_equal_regardless_of_the_order_it_was_built_in() {
            let left = [Component::new("rustc"), Component::new("cargo")]
                .into_iter()
                .collect::<ComponentSet>();
            let right = [Component::new("cargo"), Component::new("rustc")]
                .into_iter()
                .collect::<ComponentSet>();

            assert_eq!(left, right);
        }

        #[test]
        fn a_clone_shares_the_components_of_the_set_it_was_cloned_from() {
            let set = [Component::new("cargo")]
                .into_iter()
                .collect::<ComponentSet>();
            let clone = set.clone();

            assert!(ptr::eq(set.as_slice(), clone.as_slice()));
        }
    }
}
