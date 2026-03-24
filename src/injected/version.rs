use colored::Colorize;
use std::path::Path;

use crate::manifest::BundleManifest;

/// Injected `version` command for bundled CLIs.
/// `manifest = None` → print skillb version; `Some(path)` → show bundle info.
pub fn run(manifest: Option<&Path>) -> crate::error::Result<()> {
    match manifest {
        None => {
            println!("{} {}", "ski".bold(), env!("CARGO_PKG_VERSION").green());
            println!(\"  powered by skillb v{}\", env!(\"CARGO_PKG_VERSION\").dimmed());
        }
        Some(path) => {
            let manifest = BundleManifest::load(path)?;
            println!("{} @ {}", manifest.name.bold(), manifest.created_at.dimmed());
            let sha_prefix = &manifest.binary_sha256[..16.min(manifest.binary_sha256.len())];
            println!(
                "  schema v{}  |  binary SHA-256: {}...",
                manifest.schema_version, sha_prefix
            );
        }
    }
    Ok(())
}
