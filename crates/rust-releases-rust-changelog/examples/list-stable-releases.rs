use rust_releases_rust_changelog::RustChangelog;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = RustChangelog::new_ureq_cached_client()?;
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
