pub enum Channel {
    Stable(Stable),
    Beta(Beta),
    Nightly(Nightly),
}

pub struct Stable;

pub struct Beta;

pub struct Nightly;
