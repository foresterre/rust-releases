use crate::Date;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Nightly {
    pub date: Date,
}

#[cfg(test)]
mod tests {
    use crate::{Date, Nightly};

    #[yare::parameterized(
        patch1 = { Date::new(0, 0, 0), Date::new(0, 0, 1) },
        minor1 = { Date::new(0, 0, 0), Date::new(0, 1, 0) },
        major1 = { Date::new(0, 0, 0), Date::new(1, 0, 0) },
        minor_trumps_patch = { Date::new(0, 0, 99), Date::new(0, 1, 0) },
        major_trumps_patch = { Date::new(0, 0, 99), Date::new(1, 0, 0) },
        major_trumps_minor = { Date::new(0, 99, 0), Date::new(1, 0, 0) },
    )]
    fn ord(left: Date, right: Date) {
        let left = Nightly { date: left };
        let right = Nightly { date: right };

        assert!(left < right);
    }
}
