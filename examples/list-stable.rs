use rust_releases::FetchResources;
use rust_releases_core::{Channel, ReleaseIndex};
use rust_releases_rust_changelog::RustChangelog;
#[cfg(feature = "rust-dist")]
use rust_releases_rust_dist::RustDist;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::env::args()
        .nth(1)
        .unwrap_or("rust-changelog".to_string());
    let source_type = SourceType::from(source.as_str());

    let client = Client::with_source(source_type);
    let index = client.build_index();

    let releases = index.releases();
    let releases_count = releases.len();

    println!("source: {:?}", source_type);
    println!("releases count: {}", releases_count);

    let versions = releases
        .iter()
        .map(|release| format!("{}", release.version()))
        .collect::<Vec<String>>()
        .join("\n");

    println!("release versions:\n{}", versions);

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum SourceType {
    RustChangelog,
    #[cfg(feature = "rust-dist")]
    RustDist,
}

impl<'a> From<&'a str> for SourceType {
    fn from(value: &'a str) -> Self {
        match value {
            "rust-changelog" | "rustchangelog" | "changelog" => SourceType::RustChangelog,
            "rust-dist" | "rustdist" | "dist" => {
                #[cfg(feature = "rust-dist")]
                {
                    SourceType::RustDist
                }

                #[cfg(not(feature = "rust-dist"))]
                {
                    panic!(
                        "Source '{}' is not supported, to enable, compile with `--features rust-dist`", value
                    )
                }
            }
            elsy => panic!("Source '{}' is not supported", elsy),
        }
    }
}

struct Client {
    source_type: SourceType,
}

impl Client {
    pub fn with_source(source_type: SourceType) -> Self {
        Self { source_type }
    }

    pub fn build_index(&self) -> ReleaseIndex {
        let channel = Channel::Stable;

        match self.source_type {
            SourceType::RustChangelog => {
                let intermediate = RustChangelog::fetch_channel(channel)
                    .expect("Unable to fetch 'rust-changelog'");
                ReleaseIndex::from_source(intermediate)
                    .expect("Unable to build index for 'rust-changelog'")
            }
            #[cfg(feature = "rust-dist")]
            SourceType::RustDist => {
                let intermediate =
                    RustDist::fetch_channel(channel).expect("Unable to fetch 'rust-dist'");
                ReleaseIndex::from_source(intermediate)
                    .expect("Unable to build index for 'rust-dist'")
            }
        }
    }
}
