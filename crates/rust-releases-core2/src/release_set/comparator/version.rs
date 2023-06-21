use crate::Release;
use std::cmp::Ordering;

/// Orders releases by their version.
///
/// Ordering follows the following rules:
/// 1. `stable > beta > nightly`.
/// 2. if comparing any two stable or beta releases `a` and `b`, and `version(a) > version(b)`, then `a > b`.
/// 3. if comparing any two nightly releases `a` and `b`, and `date(a) > date(b)`, then `a > b`.
#[derive(Clone, Debug)]
pub struct VersionComparator(pub Release);

impl PartialEq for VersionComparator {
    fn eq(&self, other: &Self) -> bool {
        use rust_toolchain::{Beta, Channel, Nightly, Stable};

        match (self.0.toolchain().channel(), other.0.toolchain().channel()) {
            (
                Channel::Stable(Stable { version: lhs }),
                Channel::Stable(Stable { version: rhs }),
            ) => lhs.eq(rhs),
            (Channel::Beta(Beta { version: lhs }), Channel::Beta(Beta { version: rhs })) => {
                lhs.eq(rhs)
            }
            (Channel::Nightly(Nightly { date: lhs }), Channel::Nightly(Nightly { date: rhs })) => {
                lhs.eq(rhs)
            }
            _ => false,
        }
    }
}

impl Eq for VersionComparator {}

impl PartialOrd for VersionComparator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use rust_toolchain::{Beta, Channel, Nightly, Stable};

        match (self.0.toolchain().channel(), other.0.toolchain().channel()) {
            // If both are stable, compare by version
            (
                Channel::Stable(Stable { version: lhs }),
                Channel::Stable(Stable { version: rhs }),
            ) => lhs.partial_cmp(rhs),
            // If stable is compared against another, stable is greater
            (Channel::Stable(_), _) => Some(Ordering::Greater),
            // If both are beta, compare by version
            (Channel::Beta(Beta { version: lhs }), Channel::Beta(Beta { version: rhs })) => {
                lhs.partial_cmp(rhs)
            }
            // If beta is compared against stable, it is smaller
            (Channel::Beta(_), Channel::Stable(_)) => Some(Ordering::Less),
            // If beta is compared against another (only Nightly possible), it is greater
            (Channel::Beta(_), _) => Some(Ordering::Greater),
            // If both are nightly, compare by date
            (Channel::Nightly(Nightly { date: lhs }), Channel::Nightly(Nightly { date: rhs })) => {
                lhs.partial_cmp(rhs)
            }
            // Nightly is always smaller, if it isn't compared against itself
            (Channel::Nightly(_), _) => Some(Ordering::Less),
        }
    }
}

impl Ord for VersionComparator {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

// #[cfg(test)]
// mod tests {
// fn default_test_subject() -> Release {
//     Release::new_without_components(rust_toolchain::Toolchain::new(
//         rust_toolchain::Channel::Stable,
//         rust_toolchain::ReleaseDate::new(2000, 1, 2),
//         rust_toolchain::Platform::host(),
//         Some(rust_toolchain::RustVersion::new(1, 2, 3)),
//     ))
// }
//
// #[test]
// fn ordering_by_version() {
//     let mut release1 = default_test_subject();
//     release1.toolchain.version = Some(rust_toolchain::RustVersion::new(1, 0, 1));
//     release1.toolchain.date = rust_toolchain::ReleaseDate::new(1, 1, 1);
//
//     let mut release2 = default_test_subject();
//     release2.toolchain.version = Some(rust_toolchain::RustVersion::new(1, 0, 0));
//     release2.toolchain.date = rust_toolchain::ReleaseDate::new(1, 1, 1);
//
//     assert_eq!(release1.cmp(&release2), Ordering::Greater);
// }
//
// #[test]
// fn ordering_by_date() {
//     let mut release1 = default_test_subject();
//     release1.toolchain.version = None;
//     release1.toolchain.date = rust_toolchain::ReleaseDate::new(2000, 1, 3);
//
//     let mut release2 = default_test_subject();
//     release2.toolchain.version = None;
//     release2.toolchain.date = rust_toolchain::ReleaseDate::new(2000, 1, 2);
//
//     assert_eq!(release1.cmp(&release2), Ordering::Greater);
// }
//
// #[test]
// fn ordering_with_version_and_date_left() {
//     let mut release1 = default_test_subject();
//     release1.toolchain.date = rust_toolchain::ReleaseDate::new(2000, 1, 3);
//
//     let mut release2 = default_test_subject();
//     release2.toolchain.version = None;
//
//     // Regardless of the newer release date of toolchain2, toolchain1 will be greater because it has a version
//     assert_eq!(release1.cmp(&release2), Ordering::Greater);
// }
//
// #[test]
// fn ordering_with_version_and_date_right() {
//     let mut release1 = default_test_subject();
//     release1.toolchain.version = None;
//
//     let mut release2 = default_test_subject();
//     release2.toolchain.date = rust_toolchain::ReleaseDate::new(2000, 1, 3);
//
//     // Regardless of the newer release date of release1, release2 will be greater because it has a version
//     assert_eq!(release1.cmp(&release2), Ordering::Less);
// }
// }
