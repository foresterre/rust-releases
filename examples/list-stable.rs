use rust_releases::FetchResources;
use rust_releases_core::{Channel, ReleaseIndex};
use rust_releases_rust_changelog::RustChangelog;
use std::io::{stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = RustChangelog::fetch_channel(Channel::Stable)?;
    let index = ReleaseIndex::from_source(source)?;

    let stdout = stdout();
    let mut output = stdout.lock();

    output.write("source: rust-changelog\n".as_bytes())?;
    output.write(format!("parsed-releases: {}\n", index.releases().len()).as_bytes())?;

    for release in index.releases() {
        output.write(format!("{}\n", release.version()).as_bytes())?;
    }

    Ok(())
}
