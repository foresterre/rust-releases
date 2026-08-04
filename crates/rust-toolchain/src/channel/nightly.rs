use crate::Date;

/// The `Nightly` release [`channel`]
///
/// [`channel`]: https://rust-lang.github.io/rustup/concepts/channels.html
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Nightly {
    /// A short YYYY-MM-DD associated date
    pub date: Date,
}

impl Nightly {
    /// Instantiate a new `Nightly` struct, representing the version of a release channel.
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self {
            date: Date::new(year, month, day),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Date, channel::Nightly};

    #[yare::parameterized(
        patch1 = { Date::new(0, 0, 0), Date::new(0, 0, 1) },
        minor1 = { Date::new(0, 0, 0), Date::new(0, 1, 0) },
        major1 = { Date::new(0, 0, 0), Date::new(1, 0, 0) },
        minor_over_patch = { Date::new(0, 0, 99), Date::new(0, 1, 0) },
        major_over_patch = { Date::new(0, 0, 99), Date::new(1, 0, 0) },
        major_over_minor = { Date::new(0, 99, 0), Date::new(1, 0, 0) },
    )]
    fn ord(left: Date, right: Date) {
        let left = Nightly { date: left };
        let right = Nightly { date: right };

        assert!(left < right);
    }
}
