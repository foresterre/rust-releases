use std::str::FromStr;

/// The platform of a toolchain.
///
/// Commonly represented as a [`target triple`]. A target triple consists of three (or four) components: the
/// architecture component, the vendor component, the operating system component and optionally
/// a fourth component representing the environment (e.g. gnu or msvc).
///
/// [`target triple`]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md#detailed-design
// Extra information may be found here:
// - https://doc.rust-lang.org/rustc/platform-support.html
// - https://rust-lang.github.io/rustup/concepts/toolchains.html
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Target {
    target: target_lexicon::Triple,
}

impl Target {
    /// Create a new `Platform` instance which represents the `host` platform on which the compiler
    /// is ran.
    pub const fn host() -> Self {
        Self {
            target: target_lexicon::HOST,
        }
    }

    /// Create a new `Platform` instance from a [`target triple`].
    ///
    /// * See also: [Rustc platform support](https://doc.rust-lang.org/rustc/platform-support.html)
    ///
    /// [`target triple`]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md#detailed-design
    pub fn try_from_target_triple(triple: &str) -> Result<Self, ()> {
        let platform = target_lexicon::Triple::from_str(triple).map_err(|_err| ())?;

        Ok(Self { target: platform })
    }

    /// Create a new `Platform` instance from a [`target triple`], defaults to
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
}
