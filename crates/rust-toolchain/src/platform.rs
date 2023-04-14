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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Platform {
    platform: target_lexicon::Triple,
}

impl Platform {
    /// Create a new `Platform` instance which represents the `host` platform on which the compiler
    /// is ran.
    pub const fn host() -> Self {
        Self {
            platform: target_lexicon::HOST,
        }
    }

    /// Create a new `Platform` instance from a [`target triple`].
    ///
    /// * See also: [Rustc platform support](https://doc.rust-lang.org/rustc/platform-support.html)
    ///
    /// [`target triple`]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md#detailed-design
    pub fn try_from_target_triple(triple: &str) -> Result<Self, ()> {
        let platform = target_lexicon::Triple::from_str(triple).map_err(|_err| ())?;

        Ok(Self { platform })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
