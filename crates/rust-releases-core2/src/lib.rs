use std::collections::{BTreeSet, HashMap};
use std::iter;

mod register;
mod release;

/// todo!
pub use register::Register;
/// todo!
pub use release::Release;

#[test]
fn test() {
    pub struct ReleasesBuilder {
        f: (),
    }

    impl ReleasesBuilder {
        pub fn from_parser<P>(parser: P) -> Self {
            todo!()
        }

        pub fn build(self) -> Register {
            todo!()
        }
    }

    let builder = ReleasesBuilder::from_parser(todo!()).build();

    let last_release = builder.last();
}

#[test]
fn rust_releases() {
    let input = vec![];

    let register = Register::from_iter(input.iter());
}
