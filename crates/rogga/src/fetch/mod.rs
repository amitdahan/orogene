use std::path::Path;

use async_trait::async_trait;
use futures::io::AsyncRead;
use oro_package_spec::PackageSpec;

use crate::error::Result;
use crate::package::Package;
use crate::packument::{Packument, VersionMetadata};

pub use dir::DirFetcher;
pub use git::GitFetcher;
pub use registry::RegistryFetcher;

mod dir;
mod git;
mod registry;

#[async_trait]
pub trait PackageFetcher: std::fmt::Debug + Send + Sync {
    async fn name(&self, spec: &PackageSpec, base_dir: &Path) -> Result<String>;
    async fn metadata(&self, pkg: &Package) -> Result<VersionMetadata>;
    async fn packument(&self, pkg: &PackageSpec, base_dir: &Path) -> Result<Packument>;
    async fn tarball(&self, pkg: &Package) -> Result<Box<dyn AsyncRead + Unpin + Send + Sync>>;
}
