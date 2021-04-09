use crate::{ChannelManifestsError, ChannelManifestsResult};
use rust_releases_core::Channel;

#[derive(Debug)]
pub(crate) struct MetaManifest {
    manifests: Vec<ManifestSource>,
}

impl MetaManifest {
    pub(crate) fn try_from_str<T: AsRef<str>>(item: T) -> ChannelManifestsResult<Self> {
        let item = item.as_ref();

        let manifests = item
            .lines()
            .map(ManifestSource::try_from_str)
            .collect::<ChannelManifestsResult<Vec<_>>>()?;

        Ok(Self { manifests })
    }

    pub(crate) fn manifests(&self) -> &[ManifestSource] {
        &self.manifests
    }
}

#[derive(Debug)]
pub(crate) struct ManifestSource {
    url: String,
    channel: Channel,
    date: String,
}

impl ManifestSource {
    pub(crate) fn try_from_str<T: AsRef<str>>(item: T) -> ChannelManifestsResult<Self> {
        let item = item.as_ref();

        let channel = Self::parse_channel(item)?;
        let date = Self::parse_date(item)?;

        Ok(Self {
            url: format!("https://{}", item),
            channel,
            date,
        })
    }

    pub(crate) fn url(&self) -> &str {
        &self.url
    }

    pub(crate) fn channel(&self) -> Channel {
        self.channel
    }

    pub(crate) fn date(&self) -> &str {
        &self.date
    }

    fn parse_date(input: &str) -> ChannelManifestsResult<String> {
        // an input has the following form:
        // `static.rust-lang.org/dist/YYYY-MM-DD/channel-rust-CHANNEL.toml`
        // where YYYY    is the year,
        //   and MM      is the month,
        //   and DD      is the day,
        //   and CHANNEL is one of 'stable' | 'beta' | 'nightly'

        let date = input
            .get(26..36)
            .ok_or(ChannelManifestsError::ParseManifestDate)?;
        Ok(date.to_string())
    }

    fn parse_channel(input: &str) -> ChannelManifestsResult<Channel> {
        Ok(if input.contains("beta") {
            Channel::Beta
        } else if input.contains("nightly") {
            Channel::Nightly
        } else if input.contains("stable") {
            Channel::Stable
        } else {
            return Err(ChannelManifestsError::ParseManifestSource);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_releases_io::Document;

    #[test]
    fn test_parse_meta_manifest() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "/../../resources/channel_manifests/manifests.txt",
        ]
        .join("");
        let meta_file = Document::LocalPath(path.into());

        let buffer = meta_file.load().unwrap();
        let meta_manifest = MetaManifest::try_from_str(String::from_utf8(buffer).unwrap());
        assert!(meta_manifest.is_ok());
    }
}
