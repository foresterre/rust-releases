use crate::Distribution;
use rust_toolchain::Channel;
use std::cmp::Ordering;

/// Ordering follows the following rules:
/// NB: `*` can be interpreted as any of the comparator operations: `>` or `<`.
///
/// 1. `stable > beta > nightly`.
/// 2. When comparing any releases of the same channel, `a` and `b`, and `version(a) * version(b)`, then `a * b`.
///     * Only `stable` and `beta` have a version, so two `nightly` releases will be considered equal.
/// 3. When comparing two releases of the same channel, `a` and `b`, and `date(a) * date(b)`, then `a * b`.
///     * Only `nightly` has a date, so if the version of two `stable` or `beta` releases match, they will be considered equal.
#[derive(Clone, Debug, Eq)]
pub struct CompareRelease(pub Distribution);

impl PartialEq for CompareRelease {
    fn eq(&self, other: &Self) -> bool {
        let lhs = &self.0;
        let rhs = &other.0;

        match (lhs, rhs) {
            (l, r) if (lhs.is_stable() && rhs.is_stable()) || (lhs.is_beta() && rhs.is_beta()) => {
                l.toolchain().version() == r.toolchain().version()
            }
            (l, r) if lhs.is_nightly() && rhs.is_nightly() => {
                l.toolchain().nightly_date() == r.toolchain().nightly_date()
            }
            _ => false,
        }
    }
}

impl PartialOrd for CompareRelease {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for CompareRelease {
    fn cmp(&self, other: &Self) -> Ordering {
        // pre: `lhs` and `rhs` must be the same channel!
        fn compare_same_channel(lhs: &Distribution, rhs: &Distribution) -> Ordering {
            let lhs_toolchain = lhs.toolchain();
            let rhs_toolchain = rhs.toolchain();

            lhs_toolchain.version().cmp(&rhs_toolchain.version()).then(
                lhs_toolchain
                    .nightly_date()
                    .cmp(&rhs_toolchain.nightly_date()),
            )
        }

        let lhs_channel = CompareChannel(self.0.toolchain().channel());
        let rhs_channel = CompareChannel(other.0.toolchain().channel());

        lhs_channel
            .cmp(&rhs_channel)
            .then_with(|| compare_same_channel(&self.0, &other.0))
    }
}

#[cfg(test)]
mod tests_compare_release {
    use crate::set::compare::CompareRelease;
    use crate::Distribution;
    use rust_toolchain::{Channel, Platform, ReleaseDate, RustVersion};
    use std::cmp::Ordering;
    use yare::parameterized;

    fn default_test_subject() -> Distribution {
        let date = ReleaseDate::new(0, 0, 0);

        Distribution::new_without_components(
            date,
            rust_toolchain::Toolchain::new(
                Channel::stable(RustVersion::new(1, 2, 3)),
                Platform::host(),
            ),
        )
    }

    #[parameterized(
        stable = { Channel::stable(RustVersion::new(0, 0, 0)) },
        beta = { Channel::beta(RustVersion::new(0, 0, 0)) },
        nightly = { Channel::nightly(ReleaseDate::new(0, 0, 0)) },
    )]
    fn identity(channel: Channel) {
        let mut release0 = default_test_subject();
        release0.toolchain_mut().channel = channel.clone();
        let mut release1 = default_test_subject();
        release1.toolchain_mut().channel = channel;

        let lhs = CompareRelease(release0);
        let rhs = CompareRelease(release1);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs.cmp(&rhs), Ordering::Equal);
    }

    #[test]
    fn compare_stable_to_stable() {
        let mut release0 = default_test_subject();
        release0.toolchain_mut().channel = Channel::stable(RustVersion::new(1, 0, 0));
        let mut release1 = default_test_subject();
        release1.toolchain_mut().channel = Channel::stable(RustVersion::new(0, 0, 0));

        let lhs = CompareRelease(release0);
        let rhs = CompareRelease(release1);

        // lhs != rhs
        assert_ne!(lhs, rhs);
        // lhs > rhs
        assert_eq!(lhs.cmp(&rhs), Ordering::Greater);
    }

    #[test]
    fn compare_stable_to_beta() {
        let mut release0 = default_test_subject();
        release0.toolchain_mut().channel = Channel::stable(RustVersion::new(1, 0, 0));
        let mut release1 = default_test_subject();
        release1.toolchain_mut().channel = Channel::beta(RustVersion::new(0, 0, 0));

        let stable = CompareRelease(release0);
        let beta = CompareRelease(release1);

        // lhs != rhs
        assert_ne!(stable, beta);
        // lhs > rhs
        assert_eq!(stable.cmp(&beta), Ordering::Greater);
    }

    #[test]
    fn compare_stable_to_nightly() {
        let mut release0 = default_test_subject();
        release0.toolchain_mut().channel = Channel::stable(RustVersion::new(1, 0, 0));
        let mut release1 = default_test_subject();
        release1.toolchain_mut().channel = Channel::nightly(ReleaseDate::new(0, 0, 0));

        let stable = CompareRelease(release0);
        let nightly = CompareRelease(release1);

        // lhs != rhs
        assert_ne!(stable, nightly);
        // lhs > rhs
        assert_eq!(stable.cmp(&nightly), Ordering::Greater);
    }

    #[test]
    fn compare_beta_to_beta() {
        let mut release0 = default_test_subject();
        release0.toolchain_mut().channel = Channel::stable(RustVersion::new(1, 0, 0));
        let mut release1 = default_test_subject();
        release1.toolchain_mut().channel = Channel::stable(RustVersion::new(0, 0, 0));

        let lhs = CompareRelease(release0);
        let rhs = CompareRelease(release1);

        // lhs != rhs
        assert_ne!(lhs, rhs);
        // lhs > rhs
        assert_eq!(lhs.cmp(&rhs), Ordering::Greater);
    }

    #[test]
    fn compare_beta_to_stable() {
        let mut release0 = default_test_subject();
        release0.toolchain_mut().channel = Channel::beta(RustVersion::new(1, 0, 0));
        let mut release1 = default_test_subject();
        release1.toolchain_mut().channel = Channel::stable(RustVersion::new(0, 0, 0));

        let beta = CompareRelease(release0);
        let stable = CompareRelease(release1);

        // lhs != rhs
        assert_ne!(beta, stable);
        // lhs < rhs
        assert_eq!(beta.cmp(&stable), Ordering::Less);
    }

    #[test]
    fn compare_beta_to_nightly() {
        let mut release0 = default_test_subject();
        release0.toolchain_mut().channel = Channel::beta(RustVersion::new(1, 0, 0));
        let mut release1 = default_test_subject();
        release1.toolchain_mut().channel = Channel::nightly(ReleaseDate::new(0, 0, 0));

        let beta = CompareRelease(release0);
        let nightly = CompareRelease(release1);

        // lhs != rhs
        assert_ne!(beta, nightly);
        // lhs > rhs
        assert_eq!(beta.cmp(&nightly), Ordering::Greater);
    }

    #[test]
    fn compare_nightly_to_nightly() {
        let mut release0 = default_test_subject();
        release0.toolchain_mut().channel = Channel::nightly(ReleaseDate::new(1, 0, 0));
        let mut release1 = default_test_subject();
        release1.toolchain_mut().channel = Channel::nightly(ReleaseDate::new(0, 0, 0));

        let lhs = CompareRelease(release0);
        let rhs = CompareRelease(release1);

        // lhs != rhs
        assert_ne!(lhs, rhs);
        // lhs > rhs
        assert_eq!(lhs.cmp(&rhs), Ordering::Greater);
    }

    #[test]
    fn compare_nightly_to_stable() {
        let mut release0 = default_test_subject();
        release0.toolchain_mut().channel = Channel::nightly(ReleaseDate::new(1, 0, 0));
        let mut release1 = default_test_subject();
        release1.toolchain_mut().channel = Channel::stable(RustVersion::new(0, 0, 0));

        let nightly = CompareRelease(release0);
        let stable = CompareRelease(release1);

        // lhs != rhs
        assert_ne!(nightly, stable);
        // lhs < rhs
        assert_eq!(nightly.cmp(&stable), Ordering::Less);
    }

    #[test]
    fn compare_nightly_to_beta() {
        let mut release0 = default_test_subject();
        release0.toolchain_mut().channel = Channel::nightly(ReleaseDate::new(1, 0, 0));
        let mut release1 = default_test_subject();
        release1.toolchain_mut().channel = Channel::beta(RustVersion::new(0, 0, 0));

        let nightly = CompareRelease(release0);
        let beta = CompareRelease(release1);

        // lhs != rhs
        assert_ne!(nightly, beta);
        // lhs < rhs
        assert_eq!(nightly.cmp(&beta), Ordering::Less);
    }
}

// channel

#[derive(Debug, Eq, PartialEq)]
pub struct CompareChannel<'channel>(pub &'channel Channel);

impl PartialOrd for CompareChannel<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for CompareChannel<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0, other.0) {
            (Channel::Stable(lhs), Channel::Stable(rhs)) => lhs.version.cmp(&rhs.version),
            (Channel::Stable(_), Channel::Beta(_) | Channel::Nightly(_)) => Ordering::Greater,
            (Channel::Beta(_), Channel::Stable(_)) => Ordering::Less,
            (Channel::Beta(lhs), Channel::Beta(rhs)) => lhs.version.cmp(&rhs.version),
            (Channel::Beta(_), Channel::Nightly(_)) => Ordering::Greater,
            (Channel::Nightly(_), Channel::Stable(_) | Channel::Beta(_)) => Ordering::Less,
            (Channel::Nightly(lhs), Channel::Nightly(rhs)) => lhs.date.cmp(&rhs.date),
        }
    }
}

#[cfg(test)]
mod tests_channel_comparator {
    use crate::set::compare::CompareChannel;
    use rust_toolchain::{Channel, ReleaseDate, RustVersion};
    use std::cmp::Ordering;
    use yare::parameterized;

    #[parameterized(
        stable = { Channel::stable(RustVersion::new(0, 0, 0)) },
        beta = { Channel::beta(RustVersion::new(0, 0, 0)) },
        nightly = { Channel::nightly(ReleaseDate::new(0, 0, 0)) },
    )]
    fn identity(instance: Channel) {
        let lhs = CompareChannel(&instance);
        let rhs = CompareChannel(&instance);

        assert_eq!(lhs.cmp(&rhs), Ordering::Equal);
    }

    #[test]
    fn compare_stable_to_stable() {
        let v0 = Channel::stable(RustVersion::new(0, 0, 0));
        let v1 = Channel::stable(RustVersion::new(1, 0, 0));

        let lhs = CompareChannel(&v0);
        let rhs = CompareChannel(&v1);

        // lhs != rhs
        assert_ne!(lhs, rhs);
        // lhs < rhs
        assert_eq!(lhs.cmp(&rhs), Ordering::Less);
    }

    #[test]
    fn compare_stable_to_beta() {
        let v0 = Channel::stable(RustVersion::new(1, 0, 0));
        let v1 = Channel::beta(RustVersion::new(0, 0, 0));

        let stable = CompareChannel(&v0);
        let beta = CompareChannel(&v1);

        // stable != beta
        assert_ne!(stable, beta);
        // stable > beta
        assert_eq!(stable.cmp(&beta), Ordering::Greater);
    }

    #[test]
    fn compare_stable_to_nightly() {
        let v0 = Channel::stable(RustVersion::new(1, 0, 0));
        let v1 = Channel::nightly(ReleaseDate::new(0, 0, 0));

        let stable = CompareChannel(&v0);
        let nightly = CompareChannel(&v1);

        // stable != nightly
        assert_ne!(stable, nightly);
        // stable > nightly
        assert_eq!(stable.cmp(&nightly), Ordering::Greater);
    }

    #[test]
    fn compare_beta_to_beta() {
        let v0 = Channel::beta(RustVersion::new(0, 0, 0));
        let v1 = Channel::beta(RustVersion::new(1, 0, 0));

        let lhs = CompareChannel(&v0);
        let rhs = CompareChannel(&v1);

        // lhs != rhs
        assert_ne!(lhs, rhs);
        // lhs < rhs
        assert_eq!(lhs.cmp(&rhs), Ordering::Less);
    }

    #[test]
    fn compare_beta_to_stable() {
        let v0 = Channel::beta(RustVersion::new(1, 0, 0));
        let v1 = Channel::stable(RustVersion::new(0, 0, 0));

        let beta = CompareChannel(&v0);
        let stable = CompareChannel(&v1);

        // beta != stable
        assert_ne!(beta, stable);
        // beta < stable
        assert_eq!(beta.cmp(&stable), Ordering::Less);
    }

    #[test]
    fn compare_beta_to_nightly() {
        let v0 = Channel::beta(RustVersion::new(1, 0, 0));
        let v1 = Channel::nightly(ReleaseDate::new(0, 0, 0));

        let beta = CompareChannel(&v0);
        let nightly = CompareChannel(&v1);

        // beta != nightly
        assert_ne!(beta, nightly);
        // beta > nightly
        assert_eq!(beta.cmp(&nightly), Ordering::Greater);
    }

    #[test]
    fn compare_nightly_to_nightly() {
        let v0 = Channel::nightly(ReleaseDate::new(0, 0, 0));
        let v1 = Channel::nightly(ReleaseDate::new(1, 0, 0));

        let lhs = CompareChannel(&v0);
        let rhs = CompareChannel(&v1);

        // lhs != rhs
        assert_ne!(lhs, rhs);
        // lhs < rhs
        assert_eq!(lhs.cmp(&rhs), Ordering::Less);
    }

    #[test]
    fn compare_nightly_to_stable() {
        let v0 = Channel::nightly(ReleaseDate::new(1, 0, 0));
        let v1 = Channel::stable(RustVersion::new(0, 0, 0));

        let nightly = CompareChannel(&v0);
        let stable = CompareChannel(&v1);

        // nightly != stable
        assert_ne!(nightly, stable);
        // nightly < stable
        assert_eq!(nightly.cmp(&stable), Ordering::Less);
    }

    #[test]
    fn compare_nightly_to_beta() {
        let v0 = Channel::nightly(ReleaseDate::new(1, 0, 0));
        let v1 = Channel::beta(RustVersion::new(0, 0, 0));

        let nightly = CompareChannel(&v0);
        let beta = CompareChannel(&v1);

        // nightly != nightly
        assert_ne!(nightly, beta);
        // nightly < beta
        assert_eq!(nightly.cmp(&beta), Ordering::Less);
    }
}
