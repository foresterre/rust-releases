use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::Arc;
use std::{fmt, slice};

/// A target platform
///
/// Commonly represented as a [`target triple`]. A target triple consists of three (or four) components: the
/// architecture component, the vendor component, the operating system component and optionally
/// a fourth component representing the environment (e.g. gnu or msvc).
///
/// # Reading materials
///
/// - [`RFC 0131: target specification`]
/// - [`rustup concepts: toolchains`]
/// - [`rustup component history`]
/// - [`rustc platform support`]
///
/// [`target triple`]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md#detailed-design
/// [`RFC 0131: target specification`]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md#detailed-design
/// [`rustup concepts: toolchains`]: https://rust-lang.github.io/rustup/concepts/toolchains.html
/// [`rustup component history`]: https://rust-lang.github.io/rustup-components-history/
/// [`rustc platform support`]: https://doc.rust-lang.org/rustc/platform-support.html
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Target {
    target: target_lexicon::Triple,
}

impl Target {
    /// Create a new `Target` instance which represents the `host` platform.
    ///
    /// The platform on which this library is compiled, will be the `host` platform.
    pub const fn host() -> Self {
        Self {
            target: target_lexicon::HOST,
        }
    }

    /// Create a new `Target` instance from a [`target triple`].
    ///
    /// * See also: [Rustc platform support](https://doc.rust-lang.org/rustc/platform-support.html)
    ///
    /// [`target triple`]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md#detailed-design
    pub fn try_from_target_triple(triple: &str) -> Result<Self, ParseError> {
        let platform = target_lexicon::Triple::from_str(triple).map_err(ParseError::from)?;

        Ok(Self { target: platform })
    }

    /// Create a new `Target` instance from a [`target triple`], defaults to
    /// `unknown-unknown-unknown` if the give triple is not recognized.
    ///
    /// * See also: [Rustc platform support](https://doc.rust-lang.org/rustc/platform-support.html)
    ///
    /// [`target triple`]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md#detailed-design
    pub fn from_target_triple_or_unknown(triple: &str) -> Self {
        let platform = target_lexicon::Triple::from_str(triple)
            .unwrap_or_else(|_| target_lexicon::Triple::unknown());

        Self { target: platform }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.target)
    }
}

/// A set of unique [`Target`]s, ordered by target triple.
///
/// The targets are held behind an [`Arc`], so cloning a set, and with it cloning
/// the [`Toolchain`] which holds it, does not copy its targets.
///
/// Two toolchains which support the same targets can share a single set. As you can imagine,
/// this happens a lot in Rust distribution land, so it's well worth vs a regular BTreeSet ;).
///
/// [`Toolchain`]: crate::Toolchain
#[derive(Clone, Debug, Eq)]
pub struct TargetSet {
    targets: Arc<[Target]>,
}

impl TargetSet {
    /// The amount of targets in the set.
    pub fn len(&self) -> usize {
        self.targets.len()
    }

    /// Returns true if the set holds no targets, and false otherwise.
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    /// Returns true if the given target is a member of the set.
    pub fn contains(&self, target: &Target) -> bool {
        self.targets.contains(target)
    }

    /// Iterate over the targets of the set.
    pub fn iter(&self) -> slice::Iter<'_, Target> {
        self.targets.iter()
    }

    /// The targets of the set, as a slice ordered by target triple.
    pub fn as_slice(&self) -> &[Target] {
        &self.targets
    }
}

impl Default for TargetSet {
    fn default() -> Self {
        Self {
            targets: Vec::new().into(),
        }
    }
}

impl PartialEq for TargetSet {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.targets, &other.targets) || self.targets == other.targets
    }
}

impl FromIterator<Target> for TargetSet {
    fn from_iter<I: IntoIterator<Item = Target>>(iter: I) -> Self {
        // A target is identified by its triple, which is also what the set is ordered by
        let unique = iter
            .into_iter()
            .map(|target| (target.to_string(), target))
            .collect::<BTreeMap<_, _>>();

        Self {
            targets: unique.into_values().collect(),
        }
    }
}

impl From<Vec<Target>> for TargetSet {
    fn from(targets: Vec<Target>) -> Self {
        targets.into_iter().collect()
    }
}

impl<'a> IntoIterator for &'a TargetSet {
    type Item = &'a Target;
    type IntoIter = slice::Iter<'a, Target>;

    fn into_iter(self) -> Self::IntoIter {
        self.targets.iter()
    }
}

/// Errors which may occur while parsing a [`Target`].
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ParseError {
    #[error("Unknown architecture `{0}`")]
    Architecture(String),
    #[error("Unknown vendor `{0}`")]
    Vendor(String),
    #[error("Unknown operating system `{0}`")]
    OperatingSystem(String),
    #[error("Unknown environment `{0}`")]
    Environment(String),
    #[error("Unknown binary format `{0}`")]
    BinaryFormat(String),
    #[error("Unknown field `{0}`")]
    Field(String),
}

impl From<target_lexicon::ParseError> for ParseError {
    fn from(value: target_lexicon::ParseError) -> Self {
        match value {
            target_lexicon::ParseError::UnrecognizedArchitecture(v) => ParseError::Architecture(v),
            target_lexicon::ParseError::UnrecognizedVendor(v) => ParseError::Vendor(v),
            target_lexicon::ParseError::UnrecognizedOperatingSystem(v) => {
                ParseError::OperatingSystem(v)
            }
            target_lexicon::ParseError::UnrecognizedEnvironment(v) => ParseError::Environment(v),
            target_lexicon::ParseError::UnrecognizedBinaryFormat(v) => ParseError::BinaryFormat(v),
            target_lexicon::ParseError::UnrecognizedField(v) => ParseError::Field(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_platform() {
        let this_platform = Target::host();

        let expected = Target {
            target: target_lexicon::HOST,
        };

        assert_eq!(this_platform, expected);
    }

    #[test]
    fn to_string() {
        let target = Target::try_from_target_triple("x86_64-unknown-linux-gnu").unwrap();

        assert_eq!(target.to_string(), "x86_64-unknown-linux-gnu");
    }

    mod target_set {
        use super::*;
        use std::ptr;

        fn target(triple: &str) -> Target {
            Target::from_target_triple_or_unknown(triple)
        }

        #[test]
        fn an_empty_set() {
            let set = TargetSet::default();

            assert!(set.is_empty());
            assert_eq!(set.len(), 0);
        }

        #[test]
        fn a_set_is_ordered_by_target_triple() {
            let set = [
                target("x86_64-unknown-linux-gnu"),
                target("aarch64-apple-darwin"),
            ]
            .into_iter()
            .collect::<TargetSet>();

            let triples = set.iter().map(|t| t.to_string()).collect::<Vec<_>>();

            assert_eq!(
                triples,
                ["aarch64-apple-darwin", "x86_64-unknown-linux-gnu"]
            );
        }

        #[test]
        fn a_set_holds_every_target_once() {
            let set = vec![
                target("x86_64-unknown-linux-gnu"),
                target("x86_64-unknown-linux-gnu"),
            ]
            .into_iter()
            .collect::<TargetSet>();

            assert_eq!(set.len(), 1);
            assert!(set.contains(&target("x86_64-unknown-linux-gnu")));
        }

        #[test]
        fn a_set_is_equal_regardless_of_the_order_it_was_built_in() {
            let left = [
                target("wasm32-unknown-unknown"),
                target("aarch64-apple-darwin"),
            ]
            .into_iter()
            .collect::<TargetSet>();
            let right = [
                target("aarch64-apple-darwin"),
                target("wasm32-unknown-unknown"),
            ]
            .into_iter()
            .collect::<TargetSet>();

            assert_eq!(left, right);
        }

        #[test]
        fn a_clone_shares_the_targets_of_the_set_it_was_cloned_from() {
            let set = [target("aarch64-apple-darwin")]
                .into_iter()
                .collect::<TargetSet>();
            let clone = set.clone();

            assert!(ptr::eq(set.as_slice(), clone.as_slice()));
        }
    }
}
