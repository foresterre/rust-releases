use rust_releases_core::StableReleases;
use rust_releases_rust_dist::{Detail, RustDist};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dist = RustDist::new_aws_cached_client()?;
    let releases = dist.stable().fetch()?;

    let recent = releases
        .into_iter()
        .rev()
        .take(3) // Show extended info's for the last 3 releases :D
        .collect::<StableReleases>();

    let detailed = dist.stable().extend_all(recent, Detail::all())?;

    for release in detailed.iter() {
        match release.release_date() {
            Some(date) => println!("{} ({})", release.version().version, date.ymd()),
            None => println!("{}", release.version().version),
        }

        for toolchain in release.toolchains_iter() {
            println!(
                "  {host}: {components} components, {targets} targets",
                host = toolchain.host(),
                components = toolchain.components().len(),
                targets = toolchain.targets().len(),
            );
        }
    }

    Ok(())
}
