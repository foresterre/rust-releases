use rust_releases_github::GithubReleases;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = GithubReleases::new_ureq_cached_client()?;
    let releases = source.fetch()?;

    println!(
        "{} stable releases of '{}'",
        releases.len(),
        source.repository()
    );

    for release in releases.iter() {
        match release.release_date() {
            Some(date) => println!("{} ({})", release.version().version, date.ymd()),
            None => println!("{}", release.version().version),
        }
    }

    Ok(())
}
