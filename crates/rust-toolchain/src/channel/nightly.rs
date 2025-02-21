use crate::ShortDate;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Nightly {
    pub date: ShortDate,
}

#[cfg(test)]
mod tests {
    use crate::{Nightly, ShortDate};

    #[yare::parameterized(
        patch1 = { ToolchainDate::new(0, 0, 0), ToolchainDate::new(0, 0, 1) },
        minor1 = { ToolchainDate::new(0, 0, 0), ToolchainDate::new(0, 1, 0) },
        major1 = { ToolchainDate::new(0, 0, 0), ToolchainDate::new(1, 0, 0) },
        minor_trumps_patch = { ToolchainDate::new(0, 0, 99), ToolchainDate::new(0, 1, 0) },
        major_trumps_patch = { ToolchainDate::new(0, 0, 99), ToolchainDate::new(1, 0, 0) },
        major_trumps_minor = { ToolchainDate::new(0, 99, 0), ToolchainDate::new(1, 0, 0) },
    )]
    fn ord(left: ShortDate, right: ShortDate) {
        let left = Nightly { date: left };
        let right = Nightly { date: right };

        assert!(left < right);
    }
}
