use rust_releases_bundled::BundledReleases;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = BundledReleases::new();
    let releases = source.nightly();

    println!(
        "{} nightly releases, bundled on {}",
        releases.len(),
        source.generated_on().ymd()
    );

    for release in releases.iter() {
        println!("{}", release.version().date.ymd());
    }

    Ok(())
}
