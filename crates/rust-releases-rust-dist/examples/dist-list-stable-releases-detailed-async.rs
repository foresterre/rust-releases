use rust_releases_core::StableReleases;
use rust_releases_rust_dist::{Detail, RustDist};

const RECENT_RELEASES: usize = 3;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dist = RustDist::new_async_aws_cached_client().await?;
    let releases = dist.stable().fetch_async().await?;

    let recent = releases
        .into_iter()
        .rev()
        .take(RECENT_RELEASES)
        .collect::<StableReleases>();

    let detailed = dist
        .stable()
        .extend_all_async(recent, Detail::all())
        .await?;

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
