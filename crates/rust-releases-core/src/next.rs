#![allow(missing_docs)]
use std::collections::HashMap;
use crate::Channel;
use crate::semver::Version;
use std::borrow::Borrow;

// NB: hidden internals so we can change to chrono::NaiveDate if we wish so
#[derive(Debug, Clone)]
pub struct Date {
    year: u16,
    month: u16,
    day: u8,
}

#[derive(Debug)]
pub struct Release {
    channel: ToolchainChannel,
    date: Option<Date>,
    platform_support: HashMap<Host, Components>,
}

impl Release {
    /// Construct a new stable release
    pub fn new_stable(version: semver::Version) -> Self {
        Self {                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               0]9
            channel: ToolchainChannel::Versioned(crate::Channel::Stable, semver),
            date: None,
            platform_support: HashMap::new(),
        }
    }

    /// Get the Rust version for this release
    pub fn version(&self) -> Option<&semver::Version> {
        match &self.channel {
            ToolchainChannel::Versioned(_channel, version) =>  ,
            ToolchainChannel::Unversioned(_channel) => None,
        }
    }
}

// TODO: better name so it doesnt clash with Channel?
#[derive(Debug)]
pub enum ToolchainChannel {
    Unversioned(UnversionedToolchain),
    Versioned(VersionedToolchain),
}

impl ToString for ToolchainChannel {
    fn to_string(&self) -> String {
        match self {
            Self::Unversioned(inner) => inner.to_string(),
            Self::Versioned(inner) => inner.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct UnversionedToolchain(crate::Channel);

impl GetChannel for UnversionedToolchain {
    fn channel(&self) -> Channel {
        self.0
    }
}

#[derive(Debug)]
pub struct VersionedToolchain(crate::Channel, semver::Version);

impl GetChannel for VersionedToolchain {
    fn channel(&self) -> Channel {
        self.0
    }
}

impl GetVersion for VersionedToolchain {
    fn version(&self) -> &Version {
        &self.1
    }
}

// --

#[derive(Debug)]
pub struct Platform {
    components: Components,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Host {
    host: TargetTriple,
}

#[derive(Debug)]
pub struct Components(Vec<Component>);

#[derive(Debug)]
pub struct Component {
    id: String,
}

// ---

#[derive(Debug)]
pub struct Toolchain {
    channel: ToolchainChannel,
    date: Date,
    host: TargetTriple,
    components: Vec<Component>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct TargetTriple {
    triple: String,
}

pub trait GetChannel {
    fn channel(&self) -> crate::Channel;
}

pub trait GetVersion {
    fn version(&self) -> &semver::Version;
}