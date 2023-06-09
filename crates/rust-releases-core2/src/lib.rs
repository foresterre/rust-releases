mod register;
mod release;
mod release_set;

/// todo!
pub use register::Register;
/// todo!
pub use release::Release;
/// todo!
pub use release_set::ReleaseSet;

#[test]
fn rust_releases() {
    let input = vec![];
    let register = Register::from_iter(input);

    assert_eq!(register.count_releases(), 0);
}
