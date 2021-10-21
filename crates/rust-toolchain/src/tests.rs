use yare::parameterized;

use crate::Channel;

#[parameterized(
    beta = { "beta", Channel::Beta },
    nightly = { "nightly", Channel::Nightly },
    stable = { "stable", Channel::Stable }
)]
fn existing_channels(channel_name: &str, channel: Channel) {
    use std::convert::TryInto;

    let conversion = TryInto::<Channel>::try_into(channel_name);

    assert_eq!(conversion.unwrap(), channel);
}

#[parameterized(
    beta = { "Beta" },
    beta_uppercase = { "BETA" },
    nightly = { "Nightly" },
    nightly_uppercase = { "NIGHTLY" },
    stable = { "Stable" },
    stable_uppercase = { "STABLE" },
    ε = { "" },
    alpha = { "alpha" },
    starting_with_valid = { "betaa" },
    ending_in_valid = { "sstable" },
    concat = { "betastable" }
)]
fn variations_on_existing_channels(channel_name: &str) {
    use std::convert::TryInto;

    let conversion = TryInto::<Channel>::try_into(channel_name);
    let error = conversion.unwrap_err();

    assert_eq!(
        error,
        crate::Error::UnknownChannel(channel_name.to_string())
    );

    let message = format!("Unknown channel '{channel}'", channel = channel_name);

    assert_eq!(error.to_string(), message);
}
