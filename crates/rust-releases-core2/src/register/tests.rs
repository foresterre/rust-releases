use super::*;

fn default_test_subject() -> Release {
    Release::new_without_components(rust_toolchain::Toolchain::new(
        rust_toolchain::Channel::Stable,
        rust_toolchain::ReleaseDate::new(2023, 1, 1),
        rust_toolchain::Platform::host(),
        Some(rust_toolchain::RustVersion::new(1, 0, 0)),
    ))
}

#[test]
fn from_iter_with_different_versions() {
    let release1 = default_test_subject();
    let mut release2 = default_test_subject();
    release2.toolchain_mut().version = Some(rust_toolchain::RustVersion::new(1, 0, 1));

    let releases = vec![
        (rust_toolchain::Platform::host(), release1),
        (rust_toolchain::Platform::host(), release2),
    ];

    let register = Register::from_iter(releases);

    assert_eq!(register.count_releases(), 2);
}

#[test]
fn from_iter_with_different_dates() {
    let release1 = default_test_subject();
    let mut release2 = default_test_subject();
    release2.toolchain_mut().date = rust_toolchain::ReleaseDate::new(2022, 1, 1);

    let releases = vec![
        (rust_toolchain::Platform::host(), release1),
        (rust_toolchain::Platform::host(), release2),
    ];

    let register = Register::from_iter(releases);

    assert_eq!(register.count_releases(), 2);
}
