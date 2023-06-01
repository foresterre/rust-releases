use super::*;
use rust_toolchain::{Channel, Platform, ReleaseDate, RustVersion, Toolchain};

#[test]
fn from_iter() {
    let platform = Platform::host();
    let date = ReleaseDate::new(2023, 1, 1);
    let version = RustVersion::new(1, 0, 0);

    let toolchain1 = Toolchain::new(
        Channel::Stable,
        date.clone(),
        platform.clone(),
        Some(version.clone()),
    );

    let toolchain2 = Toolchain::new(
        Channel::Nightly,
        date.clone(),
        platform.clone(),
        Some(version.clone()),
    );

    let releases = vec![
        (Platform::host(), Release::new(toolchain1, vec![])),
        (Platform::host(), Release::new(toolchain2, vec![])),
    ];

    let register = Register::from_iter(releases);

    dbg!(&register);

    assert_eq!(register.count_releases(), 2);
}
