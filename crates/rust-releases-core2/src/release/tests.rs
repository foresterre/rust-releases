use crate::Release;

#[test]
fn extensions() {
    let channel = rust_toolchain::Channel::Nightly;
    let release_date = rust_toolchain::ReleaseDate::new(2023, 1, 1);
    let platform = rust_toolchain::Platform::host();
    let version = None;

    let toolchain = rust_toolchain::Toolchain::new(channel, release_date, platform, version);

    let release = Release::new(toolchain, vec![]);

    let default_components = release
        .default_components()
        .collect::<Vec<&rust_toolchain::Component>>();
    let extension_components = release.extension_components().collect::<Vec<_>>();

    assert!(default_components.is_empty());
    assert!(extension_components.is_empty());
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
