use rust_releases_rust_dist::RustDist;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dist = RustDist::new_async_aws_cached_client().await?;
    let releases = dist.stable().fetch_async().await?;

    println!("{} stable releases", releases.len());

    for release in releases.iter() {
        println!("{}", release.version().version);
    }

    Ok(())
}
