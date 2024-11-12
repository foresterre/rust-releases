// TODO: move to separate crate which depends on this crate

use crate::channel::ChannelKind;
use crate::{Channel, ReleaseDate, RustVersion, Target, Toolchain};
use std::borrow::Cow;
use std::io::Read;
use std::str;
use std::str::FromStr;
use std::{io, process};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unable to execute rustup: '{0}'")]
    Io(#[from] io::Error),
    #[error("No output")]
    NoOutput,
    #[error("Unable to parse '{0}': {1}")]
    Parse(String, ParseError),
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Couldn't find the target triple")]
    MissingTargetTriple,

    #[error("Couldn't find the version info")]
    MissingVersionInfo,

    #[error("Couldn't split the active toolchain description into a channel and target")]
    UnreadableFormat,

    #[error("Unable to parse channel kind")]
    ChannelKind,

    #[error("Unable to parse target")]
    Target,
}

pub struct RustupToolchain {
    toolchain: Toolchain,
}

impl RustupToolchain {
    pub fn active() -> Result<Self, Error> {
        let output = run_rustup()?;
        let (first_line, second_line) = lines(output)?;

        let Some((channel_description, target_description)) = first_line.split_once('-') else {
            return Err(Error::Parse(
                output.to_string(),
                ParseError::UnreadableFormat,
            ));
        };

        let Some((channel_description, target_description)) = second_line.split_once('-') else {
            return Err(Error::Parse(
                output.to_string(),
                ParseError::UnreadableFormat,
            ));
        };

        todo!()

        // let target = parse_target(target_description)?;
        // Toolchain::new()
        //
        // //
        // Ok()
    }

    pub fn installed() -> Result<Self, Error> {
        todo!()
    }
}

impl From<RustupToolchain> for Toolchain {
    fn from(value: RustupToolchain) -> Self {
        value.toolchain
    }
}

fn run_rustup() -> Result<Cow<'static, str>, Error> {
    let mut handle = process::Command::new("rustup")
        .args(["show", "active-toolchain", "--verbose"])
        .stdout(process::Stdio::piped())
        .spawn()?;

    let mut stdout = handle.stdout.take().ok_or_else(|| Error::NoOutput)?;

    let mut buffer = Vec::new();
    stdout.read_to_end(&mut buffer)?;

    Ok(String::from_utf8_lossy(&buffer))
}

fn lines(output: Cow<'_, str>) -> Result<(String, String), Error> {
    let mut lines = output.lines();

    let Some(first_line) = lines.next().map(|line| {
        line.chars()
            .take_while(|&c| !c.is_whitespace())
            .collect::<String>() // skip the (default) tag if it exists
    }) else {
        return Err(Error::Parse(
            output.to_string(),
            ParseError::MissingTargetTriple,
        ));
    };

    let Some(second_line) = lines.next() else {
        return Err(Error::Parse(
            output.to_string(),
            ParseError::MissingVersionInfo,
        ));
    };

    Ok((first_line, second_line.to_string()))
}

fn parse_channel(description: &str, version_description: &str) -> Result<Channel, Error> {
    let ck = ChannelKind::try_from_str(description)
        .map_err(|_| Error::Parse(description.to_string(), ParseError::ChannelKind))?;

    match ck {
        ChannelKind::Stable => Ok(Channel::stable(RustVersion::try_from_str(
            version_description,
        )?)),
        ChannelKind::Beta => Ok(Channel::beta(RustVersion::try_from_str(
            version_description,
        )?)),
        ChannelKind::Nightly => Ok(Channel::nightly(ReleaseDate::try_from_str(
            version_description,
        )?)),
    }
}

fn parse_target(target_description: &str) -> Result<Target, Error> {
    Target::try_from_target_triple(target_description.trim())
        .map_err(|e| Error::Parse(target_description.to_string(), ParseError::Target))
}
