use std::collections::{BTreeSet, HashMap};
use std::iter;

mod register;
mod release;

/// todo!
pub use register::Register;
/// todo!
pub use release::Release;

#[test]
fn rust_releases() {
    let input = vec![];
    let register = Register::from_iter(input);

    assert_eq!(register.count_releases(), 0);
}
