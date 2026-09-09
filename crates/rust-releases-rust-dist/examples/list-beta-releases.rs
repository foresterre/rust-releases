use rust_releases_rust_dist::RustDist;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dist = RustDist::new_aws_cached_client()?;
    let releases = dist.beta().fetch()?;

    println!("{} beta releases", releases.len());

    for release in releases.iter() {
        let version = release.version();

        match version.prerelease {
            Some(prerelease) => println!("{}-beta.{prerelease}", version.version),
            None => println!("{}-beta", version.version),
        }
    }

    Ok(())
}
