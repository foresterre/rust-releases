use crate::paging::PageSize;
use rust_releases_core::rust_release::date::Date;
use rust_releases_core::rust_release::toolchain::RustVersion;
use rust_releases_core::{RustRelease, Stable};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct ReleasePage {
    releases: Vec<Release>,
}

impl ReleasePage {
    pub fn parse(buffer: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(buffer).map(|releases| Self { releases })
    }

    pub fn is_full(&self, page_size: PageSize) -> bool {
        self.releases.len() >= usize::from(page_size.get())
    }
}

impl IntoIterator for ReleasePage {
    type Item = Release;
    type IntoIter = std::vec::IntoIter<Release>;

    fn into_iter(self) -> Self::IntoIter {
        self.releases.into_iter()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Release {
    tag_name: String,
    #[serde(default)]
    draft: bool,
    #[serde(default)]
    prerelease: bool,
    #[serde(default)]
    published_at: Option<String>,
}

impl Release {
    pub fn into_stable_release(
        self,
    ) -> Result<Option<RustRelease<Stable>>, InvalidPublicationDate> {
        if self.draft || self.prerelease {
            return Ok(None);
        }

        let Some(version) = stable_version(&self.tag_name) else {
            return Ok(None);
        };

        let release_date = match self.published_at {
            Some(timestamp) => match publication_date(&timestamp) {
                Some(date) => Some(date),
                None => {
                    return Err(InvalidPublicationDate {
                        tag: self.tag_name,
                        timestamp,
                    });
                }
            },
            None => None,
        };

        Ok(Some(RustRelease::new(version, release_date, [])))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidPublicationDate {
    pub tag: String,
    pub timestamp: String,
}

fn stable_version(tag: &str) -> Option<Stable> {
    let tag = tag.strip_prefix('v').unwrap_or(tag);

    let version = match tag.split('.').count() {
        2 => RustVersion::from_str(&format!("{tag}.0")).ok()?,
        3 => RustVersion::from_str(tag).ok()?,
        _ => return None,
    };

    Some(Stable::from(version))
}

fn publication_date(timestamp: &str) -> Option<Date> {
    let mut components = timestamp.split('T').next()?.split('-');

    let year = component::<u16>(components.next()?, 4)?;
    let month = component::<u8>(components.next()?, 2)?;
    let day = component::<u8>(components.next()?, 2)?;

    if components.next().is_some() {
        return None;
    }

    Some(Date::new(year, month, day))
}

fn component<T: FromStr>(value: &str, digits: usize) -> Option<T> {
    if value.len() != digits || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }

    value.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn release(tag: &str, published_at: Option<&str>) -> Release {
        Release {
            tag_name: tag.to_string(),
            draft: false,
            prerelease: false,
            published_at: published_at.map(str::to_string),
        }
    }

    #[test]
    fn parse_page() {
        let page = br#"[
            {
                "tag_name": "1.97.1",
                "name": "Rust 1.97.1",
                "draft": false,
                "prerelease": false,
                "published_at": "2026-07-16T12:29:15Z",
                "unknown_field": { "nested": [1, 2, 3] }
            }
        ]"#;

        let page = ReleasePage::parse(page).unwrap();

        assert!(page.is_full(PageSize::MIN));

        let release = page.into_iter().next().unwrap();

        assert_eq!(release.tag_name, "1.97.1");
        assert!(!release.draft);
        assert!(!release.prerelease);
        assert_eq!(
            release.published_at.as_deref(),
            Some("2026-07-16T12:29:15Z")
        );
    }

    #[test]
    fn parse_page_without_optional_fields() {
        let page = ReleasePage::parse(br#"[{ "tag_name": "1.97.1" }]"#).unwrap();
        let release = page.into_iter().next().unwrap();

        assert!(!release.draft);
        assert!(!release.prerelease);
        assert_eq!(release.published_at, None);
    }

    #[test]
    fn parse_empty_page() {
        let page = ReleasePage::parse(b"[]").unwrap();

        assert!(!page.is_full(PageSize::MIN));
        assert_eq!(page.into_iter().count(), 0);
    }

    #[test]
    fn a_page_is_full_when_it_holds_as_many_releases_as_requested() {
        let page = ReleasePage::parse(br#"[{ "tag_name": "1.97.1" }]"#).unwrap();

        assert!(page.is_full(PageSize::MIN));
        assert!(!page.is_full(PageSize::new(2).unwrap()));
    }

    #[test]
    fn parse_page_without_tag() {
        assert!(ReleasePage::parse(br#"[{ "name": "Rust 1.97.1" }]"#).is_err());
    }

    #[test]
    fn into_stable_release() {
        let release = release("1.97.1", Some("2026-07-16T12:29:15Z"))
            .into_stable_release()
            .unwrap()
            .unwrap();

        assert_eq!(release.version(), &Stable::new(1, 97, 1));
        assert_eq!(release.release_date(), Some(&Date::new(2026, 7, 16)));
        assert!(release.toolchains().is_empty());
    }

    #[test]
    fn into_stable_release_without_publication_date() {
        let release = release("1.97.1", None)
            .into_stable_release()
            .unwrap()
            .unwrap();

        assert_eq!(release.release_date(), None);
    }

    #[test]
    fn into_stable_release_with_an_invalid_publication_date() {
        let error = release("1.97.1", Some("yesterday"))
            .into_stable_release()
            .unwrap_err();

        assert_eq!(
            error,
            InvalidPublicationDate {
                tag: "1.97.1".to_string(),
                timestamp: "yesterday".to_string(),
            }
        );
    }

    #[test]
    fn a_draft_is_not_a_stable_release() {
        let draft = Release {
            draft: true,
            ..release("1.97.1", Some("2026-07-16T12:29:15Z"))
        };

        assert_eq!(draft.into_stable_release().unwrap(), None);
    }

    #[test]
    fn a_prerelease_is_not_a_stable_release() {
        let prerelease = Release {
            prerelease: true,
            ..release("1.97.1", Some("2026-07-16T12:29:15Z"))
        };

        assert_eq!(prerelease.into_stable_release().unwrap(), None);
    }

    #[test]
    fn an_unrecognized_tag_is_not_a_stable_release() {
        let release = release("1.0.0-alpha", Some("2026-07-16T12:29:15Z"));

        assert_eq!(release.into_stable_release().unwrap(), None);
    }

    #[yare::parameterized(
        three_components = { "1.97.1", Stable::new(1, 97, 1) },
        first_stable = { "1.0.0", Stable::new(1, 0, 0) },
        pre_one_dot_oh = { "0.12.0", Stable::new(0, 12, 0) },
        two_components = { "0.10", Stable::new(0, 10, 0) },
        two_components_single_digit = { "0.1", Stable::new(0, 1, 0) },
        prefixed_with_a_v = { "v1.97.1", Stable::new(1, 97, 1) },
        prefixed_with_a_v_and_two_components = { "v0.10", Stable::new(0, 10, 0) },
    )]
    fn accepted_tag(tag: &str, expected: Stable) {
        assert_eq!(stable_version(tag), Some(expected));
    }

    #[yare::parameterized(
        empty = { "" },
        alpha = { "1.0.0-alpha" },
        alpha_2 = { "1.0.0-alpha.2" },
        beta = { "1.97.0-beta.1" },
        nightly = { "nightly" },
        single_component = { "1" },
        four_components = { "1.2.3.4" },
        trailing_dot = { "1.2." },
        leading_zero = { "1.02.0" },
        not_a_number = { "1.x.0" },
        whitespace = { " 1.2.3" },
        release_prefix = { "release-1.2.3" },
    )]
    fn rejected_tag(tag: &str) {
        assert_eq!(stable_version(tag), None);
    }

    #[yare::parameterized(
        timestamp = { "2026-07-16T12:29:15Z", Date::new(2026, 7, 16) },
        timestamp_with_offset = { "2015-05-15T00:00:00+02:00", Date::new(2015, 5, 15) },
        date_only = { "2015-05-15", Date::new(2015, 5, 15) },
        leading_zeroes = { "0001-02-03T00:00:00Z", Date::new(1, 2, 3) },
    )]
    fn accepted_publication_date(timestamp: &str, expected: Date) {
        assert_eq!(publication_date(timestamp), Some(expected));
    }

    #[yare::parameterized(
        empty = { "" },
        prose = { "the day before yesterday" },
        unpadded_month = { "2026-7-16T12:29:15Z" },
        unpadded_day = { "2026-07-6T12:29:15Z" },
        short_year = { "26-07-16" },
        missing_day = { "2026-07" },
        additional_component = { "2026-07-16-01" },
        signed_year = { "+026-07-16" },
        not_a_date = { "yyyy-mm-dd" },
    )]
    fn rejected_publication_date(timestamp: &str) {
        assert_eq!(publication_date(timestamp), None);
    }
}
