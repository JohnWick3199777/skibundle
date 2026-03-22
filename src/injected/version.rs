use colored::Colorize;
use std::path::PathBuf;

use crate::cli::VersionArgs;
use crate::manifest::BundleManifest;

pub fn run(args: &VersionArgs) -> crate::error::Result<()> {
    let manifest_path = args
        .manifest
        .clone()
        .unwrap_or_else(|| PathBuf::from("skill.manifest.json"));

    let manifest = BundleManifest::load(&manifest_path)?;

    println!(
        "{} @ {}",
        manifest.name.bold(),
        manifest.created_at.dimmed()
    );

    let sha_prefix = &manifest.binary_sha256[..16.min(manifest.binary_sha256.len())];
    println!(
        "  schema v{}  |  binary SHA-256: {}...",
        manifest.schema_version, sha_prefix
    );

    Ok(())
}
