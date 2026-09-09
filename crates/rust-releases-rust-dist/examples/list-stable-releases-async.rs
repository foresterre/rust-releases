use rust_releases_rust_dist::RustDist;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = RustDist::new_async_aws_cached_client().await?;
    let releases = source.fetch_async().await?;

    println!("{} stable releases", releases.len());

    for release in releases.iter() {
        match release.release_date() {
            Some(date) => println!("{} ({})", release.version().version, date.ymd()),
            None => println!("{}", release.version().version),
        }
    }

    Ok(())
}
