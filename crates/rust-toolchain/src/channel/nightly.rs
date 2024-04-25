use crate::ReleaseDate;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Nightly {
    pub date: ReleaseDate,
}

#[cfg(test)]
mod tests {
    use crate::{Nightly, ReleaseDate};

    #[yare::parameterized(
        patch1 = { ReleaseDate::new(0, 0, 0), ReleaseDate::new(0, 0, 1) },
        minor1 = { ReleaseDate::new(0, 0, 0), ReleaseDate::new(0, 1, 0) },
        major1 = { ReleaseDate::new(0, 0, 0), ReleaseDate::new(1, 0, 0) },
        minor_trumps_patch = { ReleaseDate::new(0, 0, 99), ReleaseDate::new(0, 1, 0) },
        major_trumps_patch = { ReleaseDate::new(0, 0, 99), ReleaseDate::new(1, 0, 0) },
        major_trumps_minor = { ReleaseDate::new(0, 99, 0), ReleaseDate::new(1, 0, 0) },
    )]
    fn ord(left: ReleaseDate, right: ReleaseDate) {
        let left = Nightly { date: left };
        let right = Nightly { date: right };

        assert!(left < right);
    }
}
