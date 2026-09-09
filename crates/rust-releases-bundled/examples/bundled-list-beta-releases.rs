use rust_releases_bundled::BundledReleases;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = BundledReleases::new();
    let releases = source.beta();

    println!(
        "{} beta releases, bundled on {}",
        releases.len(),
        source.generated_on().ymd()
    );

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
