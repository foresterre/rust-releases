use std::collections::BTreeSet;

/// A data structure consisting of the known Rust releases.
///
/// Whether a release is known, and how much information is known about a release,
/// depends on the source used to build up this information.
pub struct Releases {
    releases: BTreeSet<Release>,
}

impl Releases {
    pub fn find(&self) -> Option<&Release> {
        todo!()
    }

    /// Least recent to most recent
    pub fn all_ascending(&self) -> impl Iterator<Item = Release> {
        todo!();
    }

    /// Most recent to least recent
    pub fn all_descending(&self) -> impl Iterator<Item = Release> {
        todo!();
    }

    pub fn last(&self) -> &Release {
        todo!();
    }
}

pub struct ReleasesBuilder {
    f: (),
}

impl ReleasesBuilder {
    pub fn from_parser<P>(parser: P) -> Self {
        todo!()
    }

    pub fn build(self) -> Releases {
        todo!()
    }
}

pub struct Release {
    toolchain: rust_toolchain::Toolchain,
}

#[test]
fn test() {
    let builder = ReleasesBuilder::from_parser(todo!()).build();

    let last_release = builder.last();
}
