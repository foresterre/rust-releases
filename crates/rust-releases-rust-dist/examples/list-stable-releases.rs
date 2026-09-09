use rust_releases_rust_dist::RustDist;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = RustDist::new_aws_cached_client()?;
    let releases = source.fetch()?;

    println!("{} stable releases", releases.len());

    for release in releases.iter() {
        match release.release_date() {
            Some(date) => println!("{} ({})", release.version().version, date.ymd()),
            None => println!("{}", release.version().version),
        }
    }

    Ok(())
}
