use rust_releases::{BundledReleases, RustDist};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bundled = BundledReleases::new().beta();
    let dist = RustDist::new_aws_cached_client()?.beta().fetch()?;

    println!("bundled: {} beta releases", bundled.len());
    println!("rust-dist: {} beta releases", dist.len());

    let releases = bundled.merge(dist);

    println!("merged: {} beta releases", releases.len());

    for release in releases.iter() {
        let version = release.version();

        let version = match version.prerelease {
            Some(prerelease) => format!("{}-beta.{}", version.version, prerelease),
            None => format!("{}-beta", version.version),
        };

        match release.release_date() {
            Some(date) => println!("{version} ({})", date.ymd()),
            None => println!("{version}"),
        }
    }

    Ok(())
}
