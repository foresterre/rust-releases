use crate::manifest::{Detail, ReleaseManifest};
use crate::releases::ReleaseCollection;
use rust_releases_core::RustRelease;
use rust_releases_io::BoxFuture;
use std::fmt::Debug;

// The release manifest of a release is addressed by the channel it belongs to, so every caller of
// these helpers hands them the client method which fetches it.
pub fn extend<V, C, F, E>(
    client: &C,
    release: RustRelease<V>,
    detail: Detail,
    manifest: F,
) -> Result<RustRelease<V>, E>
where
    V: Debug,
    F: Fn(&C, &V) -> Result<ReleaseManifest, E>,
{
    if detail.is_empty() {
        return Ok(release);
    }

    let manifest = manifest(client, &release.version)?;

    Ok(manifest.extend(release, detail))
}

pub fn extend_all<R, C, F, E>(client: &C, releases: R, detail: Detail, manifest: F) -> Result<R, E>
where
    R: ReleaseCollection,
    F: Fn(&C, &R::Version) -> Result<ReleaseManifest, E>,
{
    if detail.is_empty() {
        return Ok(releases);
    }

    let mut extended = R::empty();

    for release in releases {
        if has_detail(&release, detail) {
            extended.add(release);
            continue;
        }

        extended.add(extend(client, release, detail, &manifest)?);
    }

    Ok(extended)
}

pub async fn extend_async<V, C, F, E>(
    client: &C,
    release: RustRelease<V>,
    detail: Detail,
    manifest: F,
) -> Result<RustRelease<V>, E>
where
    V: Debug,
    F: for<'a> Fn(&'a C, &'a V) -> BoxFuture<'a, Result<ReleaseManifest, E>>,
{
    if detail.is_empty() {
        return Ok(release);
    }

    let manifest = manifest(client, &release.version).await?;

    Ok(manifest.extend(release, detail))
}

pub async fn extend_all_async<R, C, F, E>(
    client: &C,
    releases: R,
    detail: Detail,
    manifest: F,
) -> Result<R, E>
where
    R: ReleaseCollection,
    F: for<'a> Fn(&'a C, &'a R::Version) -> BoxFuture<'a, Result<ReleaseManifest, E>>,
{
    if detail.is_empty() {
        return Ok(releases);
    }

    let mut extended = R::empty();

    for release in releases {
        if has_detail(&release, detail) {
            extended.add(release);
            continue;
        }

        extended.add(extend_async(client, release, detail, &manifest).await?);
    }

    Ok(extended)
}

// A release which already has the details attached, so an additional request can be skipped
fn has_detail<V: Debug>(release: &RustRelease<V>, detail: Detail) -> bool {
    (!detail.includes_release_date() || release.release_date.is_some())
        && (!detail.includes_toolchains() || !release.toolchains.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_releases_core::Stable;
    use rust_releases_core::rust_release::date::Date;
    use rust_releases_core::rust_release::toolchain::{Channel, Target, Toolchain};
    use std::collections::HashSet;

    fn toolchain() -> Toolchain {
        Toolchain::new(
            Channel::Stable(Stable::new(1, 53, 0)),
            None,
            Target::from_target_triple_or_unknown("x86_64-apple-darwin"),
            HashSet::new(),
            HashSet::new(),
        )
    }

    #[test]
    fn a_release_without_detail() {
        let release = RustRelease::new(Stable::new(1, 53, 0), None, []);

        assert!(!has_detail(&release, Detail::release_date()));
        assert!(!has_detail(&release, Detail::toolchains()));
        assert!(has_detail(&release, Detail::default()));
    }

    #[test]
    fn a_release_with_a_release_date() {
        let release = RustRelease::new(Stable::new(1, 53, 0), Some(Date::new(2021, 6, 17)), []);

        assert!(has_detail(&release, Detail::release_date()));
        assert!(!has_detail(&release, Detail::all()));
    }

    #[test]
    fn a_release_with_every_field() {
        let release = RustRelease::new(
            Stable::new(1, 53, 0),
            Some(Date::new(2021, 6, 17)),
            [toolchain()],
        );

        assert!(has_detail(&release, Detail::all()));
    }
}
