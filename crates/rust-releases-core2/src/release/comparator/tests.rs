use super::*;

fn default_test_subject() -> rust_toolchain::Toolchain {
    rust_toolchain::Toolchain::new(
        rust_toolchain::Channel::Stable,
        rust_toolchain::ReleaseDate::new(2000, 1, 2),
        rust_toolchain::Platform::host(),
        Some(rust_toolchain::RustVersion::new(1, 2, 3)),
    )
}

#[test]
fn ordering_by_version() {
    let mut t1 = default_test_subject();
    t1.date = rust_toolchain::ReleaseDate::new(2000, 0, 0);
    t1.version = Some(rust_toolchain::RustVersion::new(1, 0, 0));

    let mut t2 = default_test_subject();
    t1.date = rust_toolchain::ReleaseDate::new(2000, 0, 0);
    t2.version = Some(rust_toolchain::RustVersion::new(0, 0, 0));

    let c1 = RustToolchainComparator::from(&t1);
    let c2 = RustToolchainComparator::from(&t2);

    assert_eq!(c1.cmp(&c2), Ordering::Greater);
}

#[test]
fn ordering_by_date() {
    let mut t1 = default_test_subject();
    t1.version = None;
    t1.date = rust_toolchain::ReleaseDate::new(2000, 1, 1);

    let mut t2 = default_test_subject();
    t2.version = None;
    t2.date = rust_toolchain::ReleaseDate::new(1999, 1, 1);

    let c1 = RustToolchainComparator::from(&t1);
    let c2 = RustToolchainComparator::from(&t2);

    assert_eq!(c1.cmp(&c2), Ordering::Greater);
}

#[test]
fn ordering_with_version_and_date_left() {
    let mut t1 = default_test_subject();
    t1.date = rust_toolchain::ReleaseDate::new(1999, 0, 0);
    t1.version = Some(rust_toolchain::RustVersion::new(1, 0, 0));

    let mut t2 = default_test_subject();
    t2.date = rust_toolchain::ReleaseDate::new(2000, 0, 0);
    t2.version = None;

    let c1 = RustToolchainComparator::from(&t1);
    let c2 = RustToolchainComparator::from(&t2);

    // Regardless of the newer release date of toolchain2, toolchain1 will be greater because it has a version
    assert_eq!(c1.cmp(&c2), Ordering::Greater);
}

#[test]
fn ordering_with_version_and_date_right() {
    let mut t1 = default_test_subject();
    t1.date = rust_toolchain::ReleaseDate::new(2000, 0, 0);
    t1.version = None;

    let mut t2 = default_test_subject();
    t2.date = rust_toolchain::ReleaseDate::new(1999, 0, 0);
    t2.version = Some(rust_toolchain::RustVersion::new(1, 0, 0));

    let c1 = RustToolchainComparator::from(&t1);
    let c2 = RustToolchainComparator::from(&t2);

    // Regardless of the newer release date of toolchain1, toolchain2 will be greater because it has a version
    assert_eq!(c1.cmp(&c2), Ordering::Less);
}
