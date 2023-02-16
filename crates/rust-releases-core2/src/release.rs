pub struct Release {
    channel: Channel,
}

pub enum Channel {
    Stable,
    Beta,
    Nightly,
    Versioned(Version),
}

type Version = ();
