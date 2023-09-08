use crate::Distribution;

#[test]
fn extensions() {
    let release_date = rust_toolchain::ReleaseDate::new(2023, 1, 1);
    let channel = rust_toolchain::Channel::nightly(release_date.clone());
    let platform = rust_toolchain::Platform::host();

    let toolchain = rust_toolchain::Toolchain::new(channel, platform);
    let release = Distribution::new(release_date, toolchain, []);

    let default_components = release.default_components().count();
    let extension_components = release.extension_components().count();

    assert_eq!(default_components, 0);
    assert_eq!(extension_components, 0);
}

#[test]
fn find_component_returns_none_if_release_has_no_components() {
    let release_date = rust_toolchain::ReleaseDate::new(2023, 1, 1);
    let channel = rust_toolchain::Channel::nightly(release_date.clone());
    let platform = rust_toolchain::Platform::host();

    let toolchain = rust_toolchain::Toolchain::new(channel, platform);
    let release = Distribution::new(release_date, toolchain, []);

    let component = release.find_component("hello");
    assert!(component.is_none());
}
