use super::*;
use rust_toolchain::{Channel, Platform, ReleaseDate, RustVersion, Toolchain};

mod first {
    use super::*;

    #[test]
    fn present() {
        let date = ReleaseDate::new(2023, 1, 1);
        let stable = Channel::stable(RustVersion::new(1, 0, 0));

        let actual_release = Release::new_without_components(
            date.clone(),
            Toolchain::new(stable.clone(), Platform::host()),
        );

        let mut releases = ReleaseSet::default();
        releases.push(actual_release);

        let expected_release =
            Release::new_without_components(date, Toolchain::new(stable, Platform::host()));
        assert_eq!(releases.first(), Some(&expected_release));
    }

    #[test]
    fn absent() {
        let releases = ReleaseSet::default();
        assert!(releases.first().is_none());
    }
}

#[test]
fn uniqueness_on_channel() {
    let date = ReleaseDate::new(2023, 1, 1);
    let stable = Channel::stable(RustVersion::new(1, 0, 0));
    let beta = Channel::beta(RustVersion::new(1, 0, 0));

    let r1 =
        Release::new_without_components(date.clone(), Toolchain::new(stable, Platform::host()));
    let r2 = Release::new_without_components(date, Toolchain::new(beta, Platform::host()));

    let set = ReleaseSet::from_iter([r1, r2]);
    assert_eq!(set.len(), 2);
}

#[test]
fn uniqueness_on_release_date() {
    let date1 = ReleaseDate::new(2023, 1, 1);
    let date2 = ReleaseDate::new(2022, 1, 1);
    let channel = Channel::stable(RustVersion::new(1, 0, 0));

    let r1 =
        Release::new_without_components(date1, Toolchain::new(channel.clone(), Platform::host()));

    let r2 = Release::new_without_components(date2, Toolchain::new(channel, Platform::host()));

    let set = ReleaseSet::from_iter([r1, r2]);

    // Only 1 is collected into the release set, since two identical channel, version
    // combinations are expected to be unique; a different version should
    // be published with a new version number or different nightly release date;
    // i.e. have a different channel spec.
    assert_eq!(set.len(), 1);
}

#[test]
fn uniqueness_on_version() {
    let date = ReleaseDate::new(2023, 1, 1);
    let channel1 = Channel::stable(RustVersion::new(1, 0, 0));
    let channel2 = Channel::stable(RustVersion::new(1, 1, 0));

    let r1 =
        Release::new_without_components(date.clone(), Toolchain::new(channel1, Platform::host()));
    let r2 = Release::new_without_components(date, Toolchain::new(channel2, Platform::host()));

    let set = ReleaseSet::from_iter([r1, r2]);
    assert_eq!(set.len(), 2);
}
