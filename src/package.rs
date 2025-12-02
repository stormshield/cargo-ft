use std::error::Error;
use std::fmt;

use anstream::eprintln;
use cargo_metadata::{Metadata, Package};
use error_stack::{Report, ResultExt};

use crate::color::warn;
use crate::package::metadata::{FtWorkspaceMetadata, ParseMetadataError};

mod metadata;

pub use metadata::FtMetadata;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PackageMetadataError {
    name: String,
}

impl PackageMetadataError {
    fn from(name: impl Into<String>) -> impl FnOnce() -> Self {
        || Self { name: name.into() }
    }
}

impl fmt::Display for PackageMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not parse {} metadata", self.name)
    }
}

impl Error for PackageMetadataError {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FtPackage<'p> {
    pub package: &'p Package,
    metadata: FtMetadata,
}

impl<'p> FtPackage<'p> {
    pub fn parse_metadata(
        metadata: &Metadata,
        packages: &[&'p Package],
    ) -> Result<Vec<Self>, Report<PackageMetadataError>> {
        let context = PackageMetadataError::from;

        let workspace_metadata = FtWorkspaceMetadata::parse(metadata.workspace_metadata.clone())
            .change_context_lazy(context("workspace"))?;

        packages
            .iter()
            .map(|p| {
                let metadata = FtMetadata::parse(&workspace_metadata, p.metadata.clone())
                    .or_else(|err| match err.current_context() {
                        ParseMetadataError::Invalid => Err(err),
                        ParseMetadataError::Missing => {
                            eprintln!(
                                "{:>12} default behavior for {} because {err}",
                                warn("Using"),
                                p.name,
                            );
                            Ok(FtMetadata::default())
                        }
                    })
                    .change_context_lazy(context(&p.name))?;

                Ok(Self { package: p, metadata })
            })
            .collect()
    }

    pub fn metadata(&self) -> &FtMetadata {
        &self.metadata
    }
}
