use rust_releases::{BundledReleases, RustDist};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bundled = BundledReleases::new().nightly();
    let dist = RustDist::new_aws_cached_client()?.nightly().fetch()?;

    println!("bundled: {} nightly releases", bundled.len());
    println!("rust-dist: {} nightly releases", dist.len());

    let releases = bundled.merge(dist);

    println!("merged: {} nightly releases", releases.len());

    for release in releases.iter() {
        println!("{}", release.version().date.ymd());
    }

    Ok(())
}
