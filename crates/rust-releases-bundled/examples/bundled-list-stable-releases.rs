use rust_releases_bundled::BundledReleases;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = BundledReleases::new();
    let releases = source.stable();

    println!(
        "{} stable releases, bundled on {}",
        releases.len(),
        source.generated_on().ymd()
    );

    for release in releases.iter() {
        match release.release_date() {
            Some(date) => println!("{} ({})", release.version().version, date.ymd()),
            None => println!("{}", release.version().version),
        }
    }

    Ok(())
}
