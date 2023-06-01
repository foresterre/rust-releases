use crate::release::CompareRustToolchain;
use crate::Release;
use std::cmp::Ordering;

mod ordering {
    use super::*;

    #[test]
    fn ordering_by_version() {
        let channel = rust_toolchain::Channel::Stable;
        let release_date = rust_toolchain::ReleaseDate::new(2022, 1, 1);
        let platform = rust_toolchain::Platform::host();

        let toolchain1 = rust_toolchain::Toolchain::new(
            channel,
            release_date.clone(),
            platform.clone(),
            Some(rust_toolchain::RustVersion::new(1, 0, 0)),
        );

        let toolchain2 = rust_toolchain::Toolchain::new(
            channel,
            release_date,
            platform,
            Some(rust_toolchain::RustVersion::new(0, 0, 0)),
        );

        let c1 = CompareRustToolchain::new(&toolchain1);
        let c2 = CompareRustToolchain::new(&toolchain2);

        assert_eq!(c1.cmp(&c2), Ordering::Greater);
    }

    #[test]
    fn ordering_by_date() {
        let channel = rust_toolchain::Channel::Stable;
        let release_date1 = rust_toolchain::ReleaseDate::new(2023, 1, 1);
        let release_date2 = rust_toolchain::ReleaseDate::new(2022, 1, 1);

        let platform = rust_toolchain::Platform::host();

        let toolchain1 =
            rust_toolchain::Toolchain::new(channel, release_date1, platform.clone(), None);

        let toolchain2 = rust_toolchain::Toolchain::new(channel, release_date2, platform, None);

        let c1 = CompareRustToolchain::new(&toolchain1);
        let c2 = CompareRustToolchain::new(&toolchain2);

        assert_eq!(c1.cmp(&c2), Ordering::Greater);
    }

    #[test]
    fn ordering_with_version_and_date_left() {
        let channel = rust_toolchain::Channel::Stable;
        let release_date1 = rust_toolchain::ReleaseDate::new(2022, 1, 1);
        let release_date2 = rust_toolchain::ReleaseDate::new(2023, 1, 1);

        let platform = rust_toolchain::Platform::host();

        let toolchain1 = rust_toolchain::Toolchain::new(
            channel,
            release_date1,
            platform.clone(),
            Some(rust_toolchain::RustVersion::new(1, 0, 0)),
        );

        let toolchain2 = rust_toolchain::Toolchain::new(channel, release_date2, platform, None);

        let c1 = CompareRustToolchain::new(&toolchain1);
        let c2 = CompareRustToolchain::new(&toolchain2);

        // Regardless of the newer release date of toolchain2, toolchain1 will be greater because it has a version
        assert_eq!(c1.cmp(&c2), Ordering::Greater);
    }

    #[test]
    fn ordering_with_version_and_date_right() {
        let channel = rust_toolchain::Channel::Stable;
        let release_date1 = rust_toolchain::ReleaseDate::new(2023, 1, 1);
        let release_date2 = rust_toolchain::ReleaseDate::new(2022, 1, 1);

        let platform = rust_toolchain::Platform::host();

        let toolchain1 =
            rust_toolchain::Toolchain::new(channel, release_date1, platform.clone(), None);

        let toolchain2 = rust_toolchain::Toolchain::new(
            channel,
            release_date2,
            platform,
            Some(rust_toolchain::RustVersion::new(1, 0, 0)),
        );

        let c1 = CompareRustToolchain::new(&toolchain1);
        let c2 = CompareRustToolchain::new(&toolchain2);

        // Regardless of the newer release date of toolchain1, toolchain2 will be greater because it has a version
        assert_eq!(c1.cmp(&c2), Ordering::Less);
    }
}

#[test]
fn find_component() {
    let release: Release = todo!();

    release.find_component("hello-world");
}

#[test]
fn extensions() {
    let channel = rust_toolchain::Channel::Nightly;
    let release_date = rust_toolchain::ReleaseDate::new(2023, 1, 1);
    let platform = rust_toolchain::Platform::host();
    let version = None;

    let toolchain = rust_toolchain::Toolchain::new(channel, release_date, platform, version);

    let release = Release::new(toolchain, vec![]);

    let ext = release.extension_components();
    let ext = release.default_components();
}

#[test]
fn find_component_returns_none_if_release_has_no_components() {
    let channel = rust_toolchain::Channel::Nightly;
    let release_date = rust_toolchain::ReleaseDate::new(2023, 1, 1);
    let platform = rust_toolchain::Platform::host();
    let version = None;

    let toolchain = rust_toolchain::Toolchain::new(channel, release_date, platform, version);

    let release = Release::new(toolchain, vec![]);
    let component = release.find_component("hello");

    assert!(component.is_none());
}
