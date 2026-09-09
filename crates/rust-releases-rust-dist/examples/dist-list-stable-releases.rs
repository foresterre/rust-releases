use rust_releases_rust_dist::RustDist;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dist = RustDist::new_aws_cached_client()?;
    let releases = dist.stable().fetch()?;

    println!("{} stable releases", releases.len());

    for release in releases.iter() {
        println!("{}", release.version().version);
    }

    Ok(())
}
