use super::*;

fn default_test_subject() -> Release {
    Release::new_without_components(rust_toolchain::Toolchain::new(
        rust_toolchain::Channel::Stable,
        rust_toolchain::ReleaseDate::new(2000, 1, 2),
        rust_toolchain::Platform::host(),
        Some(rust_toolchain::RustVersion::new(1, 2, 3)),
    ))
}

#[test]
fn ordering_by_version() {
    let mut release1 = default_test_subject();
    release1.toolchain.version = Some(rust_toolchain::RustVersion::new(1, 0, 1));
    release1.toolchain.date = rust_toolchain::ReleaseDate::new(1, 1, 1);

    let mut release2 = default_test_subject();
    release2.toolchain.version = Some(rust_toolchain::RustVersion::new(1, 0, 0));
    release2.toolchain.date = rust_toolchain::ReleaseDate::new(1, 1, 1);

    assert_eq!(release1.cmp(&release2), Ordering::Greater);
}

#[test]
fn ordering_by_date() {
    let mut release1 = default_test_subject();
    release1.toolchain.version = None;
    release1.toolchain.date = rust_toolchain::ReleaseDate::new(2000, 1, 3);

    let mut release2 = default_test_subject();
    release2.toolchain.version = None;
    release2.toolchain.date = rust_toolchain::ReleaseDate::new(2000, 1, 2);

    assert_eq!(release1.cmp(&release2), Ordering::Greater);
}

#[test]
fn ordering_with_version_and_date_left() {
    let mut release1 = default_test_subject();
    release1.toolchain.date = rust_toolchain::ReleaseDate::new(2000, 1, 3);

    let mut release2 = default_test_subject();
    release2.toolchain.version = None;

    // Regardless of the newer release date of toolchain2, toolchain1 will be greater because it has a version
    assert_eq!(release1.cmp(&release2), Ordering::Greater);
}

#[test]
fn ordering_with_version_and_date_right() {
    let mut release1 = default_test_subject();
    release1.toolchain.version = None;

    let mut release2 = default_test_subject();
    release2.toolchain.date = rust_toolchain::ReleaseDate::new(2000, 1, 3);

    // Regardless of the newer release date of release1, release2 will be greater because it has a version
    assert_eq!(release1.cmp(&release2), Ordering::Less);
}
