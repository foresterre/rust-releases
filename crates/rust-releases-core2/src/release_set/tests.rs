use super::*;
use rust_toolchain::{Channel, Platform, ReleaseDate, RustVersion, Toolchain};

#[test]
fn uniqueness() {
    let items = vec![
        Toolchain::new(
            Channel::Stable,
            ReleaseDate::new(2023, 1, 1),
            Platform::host(),
            Some(RustVersion::new(1, 0, 0)),
        ),
        Toolchain::new(
            Channel::Nightly,
            ReleaseDate::new(2023, 1, 1),
            Platform::host(),
            Some(RustVersion::new(1, 0, 0)),
        ),
    ]
    .into_iter()
    .map(|t| Release::new_without_components(t));

    let set = ReleaseSet::from_iter(items);

    assert_eq!(set.len(), 2);
}
