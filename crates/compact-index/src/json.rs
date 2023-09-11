use crate::channel_manifest::ChannelManifest;

pub struct JsonManifest;

impl TryFrom<ChannelManifest> for JsonManifest {
    type Error = ();

    fn try_from(_value: ChannelManifest) -> Result<Self, Self::Error> {
        todo!()
    }
}
