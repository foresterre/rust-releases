use rust_releases_core::Stable;
use rust_releases_rust_dist::{BlockingAwsDistClient, RustDist};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dist = RustDist::new(BlockingAwsDistClient::new()?);

    let start = std::time::Instant::now();
    let releases = dist.stable().fetch()?;

    println!("stable: {} in {:?}", releases.len(), start.elapsed());
    println!(
        "first: {}, last: {}",
        releases.iter().next().unwrap().version().version,
        releases.iter().last().unwrap().version().version,
    );

    for (major, minor, patch) in [
        (1, 0, 0),
        (1, 1, 0),
        (1, 2, 0),
        (1, 3, 0),
        (1, 4, 0),
        (1, 5, 0),
        (1, 6, 0),
        (1, 7, 0),
        (1, 8, 0),
        (1, 10, 0),
        (1, 53, 0),
    ] {
        let version = Stable::new(major, minor, patch);
        assert!(
            releases.iter().any(|release| release.version() == &version),
            "missing {version:?}"
        );
    }

    println!("1.0.0 up to 1.7.0, and others, are present :)");

    Ok(())
}
