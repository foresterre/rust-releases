// Shared by the integration tests, each of which uses a part of it
#![allow(dead_code)]

use rust_releases_core::{Beta, BetaReleases, Nightly, NightlyReleases, Stable, StableReleases};
use rust_releases_io::BoxFuture;
use rust_releases_rust_dist::{AsyncDistClient, DistClient, ReleaseManifest};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Request {
    StableReleases,
    BetaReleases,
    NightlyReleases,
    StableManifest(Stable),
    BetaManifest(Beta),
    NightlyManifest(Nightly),
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[error("the distribution is unavailable")]
pub struct DistError;

// A stand-in for the Rust distribution, which serves the releases and release manifests a test
// hands it, and records what was asked of it.
#[derive(Clone, Default)]
pub struct FakeDist {
    state: Arc<State>,
}

#[derive(Default)]
struct State {
    stable: Mutex<StableReleases>,
    beta: Mutex<BetaReleases>,
    nightly: Mutex<NightlyReleases>,
    manifest: Mutex<Option<Vec<u8>>>,
    requests: Mutex<Vec<Request>>,
    fault: Mutex<bool>,
}

impl FakeDist {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn serve_stable(&self, releases: StableReleases) -> &Self {
        *self.state.stable.lock().unwrap() = releases;

        self
    }

    pub fn serve_beta(&self, releases: BetaReleases) -> &Self {
        *self.state.beta.lock().unwrap() = releases;

        self
    }

    pub fn serve_nightly(&self, releases: NightlyReleases) -> &Self {
        *self.state.nightly.lock().unwrap() = releases;

        self
    }

    pub fn serve_manifest(&self, manifest: &str) -> &Self {
        self.serve_manifest_bytes(read(&resource(manifest)))
    }

    pub fn serve_manifest_bytes(&self, manifest: Vec<u8>) -> &Self {
        *self.state.manifest.lock().unwrap() = Some(manifest);

        self
    }

    pub fn serve_fault(&self) -> &Self {
        *self.state.fault.lock().unwrap() = true;

        self
    }

    pub fn requests(&self) -> Vec<Request> {
        self.state.requests.lock().unwrap().clone()
    }

    fn releases<T: Clone>(&self, request: Request, served: &Mutex<T>) -> Result<T, DistError> {
        self.record(request)?;

        Ok(served.lock().unwrap().clone())
    }

    fn manifest(&self, request: Request) -> Result<ReleaseManifest, DistError> {
        self.record(request)?;

        let manifest = self.state.manifest.lock().unwrap();
        let manifest = manifest.as_deref().ok_or(DistError)?;

        ReleaseManifest::parse(manifest).map_err(|_| DistError)
    }

    fn record(&self, request: Request) -> Result<(), DistError> {
        self.state.requests.lock().unwrap().push(request);

        match *self.state.fault.lock().unwrap() {
            true => Err(DistError),
            false => Ok(()),
        }
    }
}

impl DistClient for FakeDist {
    type Error = DistError;

    fn stable_releases(&self) -> Result<StableReleases, Self::Error> {
        self.releases(Request::StableReleases, &self.state.stable)
    }

    fn beta_releases(&self) -> Result<BetaReleases, Self::Error> {
        self.releases(Request::BetaReleases, &self.state.beta)
    }

    fn nightly_releases(&self) -> Result<NightlyReleases, Self::Error> {
        self.releases(Request::NightlyReleases, &self.state.nightly)
    }

    fn stable_manifest(&self, version: &Stable) -> Result<ReleaseManifest, Self::Error> {
        self.manifest(Request::StableManifest(version.clone()))
    }

    fn beta_manifest(&self, version: &Beta) -> Result<ReleaseManifest, Self::Error> {
        self.manifest(Request::BetaManifest(version.clone()))
    }

    fn nightly_manifest(&self, version: &Nightly) -> Result<ReleaseManifest, Self::Error> {
        self.manifest(Request::NightlyManifest(version.clone()))
    }
}

impl AsyncDistClient for FakeDist {
    type Error = DistError;

    fn stable_releases(&self) -> BoxFuture<'_, Result<StableReleases, Self::Error>> {
        let result = DistClient::stable_releases(self);

        Box::pin(async move { result })
    }

    fn beta_releases(&self) -> BoxFuture<'_, Result<BetaReleases, Self::Error>> {
        let result = DistClient::beta_releases(self);

        Box::pin(async move { result })
    }

    fn nightly_releases(&self) -> BoxFuture<'_, Result<NightlyReleases, Self::Error>> {
        let result = DistClient::nightly_releases(self);

        Box::pin(async move { result })
    }

    fn stable_manifest<'a>(
        &'a self,
        version: &'a Stable,
    ) -> BoxFuture<'a, Result<ReleaseManifest, Self::Error>> {
        let result = DistClient::stable_manifest(self, version);

        Box::pin(async move { result })
    }

    fn beta_manifest<'a>(
        &'a self,
        version: &'a Beta,
    ) -> BoxFuture<'a, Result<ReleaseManifest, Self::Error>> {
        let result = DistClient::beta_manifest(self, version);

        Box::pin(async move { result })
    }

    fn nightly_manifest<'a>(
        &'a self,
        version: &'a Nightly,
    ) -> BoxFuture<'a, Result<ReleaseManifest, Self::Error>> {
        let result = DistClient::nightly_manifest(self, version);

        Box::pin(async move { result })
    }
}

pub fn scratch_cache_folder(test: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!(
        "rust-releases-rust-dist-{}-{}",
        std::process::id(),
        test
    ));

    let _ = std::fs::remove_dir_all(&folder);

    folder
}

fn resource(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../resources/channel_manifests")
        .join(name)
}

fn read(path: &Path) -> Vec<u8> {
    std::fs::read(path)
        .unwrap_or_else(|error| panic!("unable to read '{}': {error}", path.display()))
}
