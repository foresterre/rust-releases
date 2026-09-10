use rust_releases::core::{Stable, StableReleases};
use rust_releases::{BundledReleases, GithubReleases, RustChangelog, RustDist};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sources = [
        (
            "rust-changelog",
            RustChangelog::new_ureq_cached_client()?.fetch()?,
        ),
        ("bundled", BundledReleases::new().stable()),
        (
            "rust-dist",
            RustDist::new_aws_cached_client()?.stable().fetch()?,
        ),
        ("github", GithubReleases::new_ureq_cached_client()?.fetch()?),
    ];

    for (name, releases) in &sources {
        println!("{name}: {} stable releases", releases.len());
    }

    let merged = sources
        .iter()
        .map(|(_, releases)| releases.clone())
        .reduce(StableReleases::merge)
        .unwrap_or_else(StableReleases::empty);

    println!("merged: {} stable releases", merged.len());
    println!();

    print!("{:<10}", "missing");
    for (name, _) in &sources {
        print!("{name:<16}");
    }
    println!();

    for release in merged.iter() {
        let present = sources
            .iter()
            .map(|(_, releases)| has(releases, release.version()))
            .collect::<Vec<_>>();

        if present.iter().all(|present| *present) {
            continue;
        }

        print!("{:<10}", release.version().version.to_string());
        for present in present {
            print!("{:<16}", if present { "" } else { "x" });
        }
        println!();
    }

    Ok(())
}

fn has(releases: &StableReleases, version: &Stable) -> bool {
    releases.iter().any(|release| release.version() == version)
}
