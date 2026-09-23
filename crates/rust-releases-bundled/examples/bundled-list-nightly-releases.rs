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
        let version = release.version().date.ymd();
        let toolchains = release.toolchains.len();
        let hosts = release
            .toolchains
            .iter()
            .map(|t| format!("{}", t.host()))
            .collect::<Vec<String>>()
            .join(",");

        println!("nightly version: {version} has {toolchains} toolchains\nhosts: {hosts}\n");
    }

    Ok(())
}
