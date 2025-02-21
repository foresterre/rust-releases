use std::str::FromStr;

/// A Rust toolchain
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
///
/// [`target triple`]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md#detailed-design
/// [`RFC 0131: target specification`]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md#detailed-design
/// [`rustup concepts: toolchains`]: https://rust-lang.github.io/rustup/concepts/toolchains.html
/// [`rustup component history`]: https://rust-lang.github.io/rustup-components-history/
// Extra information may be found here:
// - https://doc.rust-lang.org/rustc/platform-support.html
// - https://rust-lang.github.io/rustup/concepts/toolchains.html
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Target {
    target: target_lexicon::Triple,
}

impl Target {
    /// Create a new `Target` instance which represents the `host` platform on which the compiler
    /// runs.
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
}
