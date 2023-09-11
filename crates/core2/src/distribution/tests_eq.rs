// Tests for PartialEq and Eq

use super::*;

fn default_test_subject() -> Distribution {
    let date = rust_toolchain::ReleaseDate::new(2000, 1, 2);

    Distribution::new_without_components(
        date,
        rust_toolchain::Toolchain::new(
            rust_toolchain::Channel::stable(rust_toolchain::RustVersion::new(1, 2, 3)),
            rust_toolchain::Platform::host(),
        ),
    )
}

#[test]
fn partial_eq_identity() {
    let toolchain1 = default_test_subject();
    let toolchain2 = default_test_subject();

    assert_eq!(toolchain1, toolchain2);
}

#[test]
fn partial_neq_on_toolchain_channel() {
    let mut toolchain1 = default_test_subject();
    toolchain1.toolchain.channel =
        rust_toolchain::Channel::beta(rust_toolchain::RustVersion::new(1, 2, 4));

    let toolchain2 = default_test_subject();

    assert_ne!(toolchain1, toolchain2);
}

#[test]
fn partial_neq_on_release_date() {
    let mut toolchain1 = default_test_subject();
    toolchain1.date = rust_toolchain::ReleaseDate::new(2000, 1, 1);

    let toolchain2 = default_test_subject();

    assert_ne!(toolchain1, toolchain2);
}

#[test]
fn partial_neq_on_toolchain_on_platform() {
    let mut toolchain1 = default_test_subject();
    toolchain1.toolchain.platform =
        rust_toolchain::Platform::try_from_target_triple("x86_64-unknown-haiku").unwrap();

    let toolchain2 = default_test_subject();

    assert_ne!(toolchain1, toolchain2);
}

#[test]
fn partial_neq_on_channel_with_version() {
    let mut toolchain1 = default_test_subject();
    toolchain1.toolchain.channel =
        rust_toolchain::Channel::stable(rust_toolchain::RustVersion::new(1, 2, 4));

    let toolchain2 = default_test_subject();

    assert_ne!(toolchain1, toolchain2);
}

#[test]
fn partial_neq_on_channel_without_version() {
    let mut toolchain1 = default_test_subject();
    toolchain1.toolchain.channel =
        rust_toolchain::Channel::nightly(rust_toolchain::ReleaseDate::new(2001, 1, 1));

    let toolchain2 = default_test_subject();

    assert_ne!(toolchain1, toolchain2);
}

#[test]
fn partial_neq_on_toolchain_on_components() {
    let mut toolchain1 = default_test_subject();
    toolchain1
        .components
        .push(rust_toolchain::Component::new_component("hello"));

    let toolchain2 = default_test_subject();

    assert_ne!(toolchain1, toolchain2);
}
