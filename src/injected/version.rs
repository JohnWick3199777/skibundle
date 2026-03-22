use colored::Colorize;

use crate::cli::VersionArgs;
use crate::manifest::BundleManifest;

pub fn run(args: &VersionArgs) -> crate::error::Result<()> {
    // Without --manifest: print ski/skilib version from Cargo.toml
    match &args.manifest {
        None => {
            println!(
                "{} {}",
                "ski".bold(),
                env!("CARGO_PKG_VERSION").green()
            );
            println!("  powered by skilib v{}", env!("CARGO_PKG_VERSION").dimmed());
        }
        Some(path) => {
            let manifest = BundleManifest::load(path)?;
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
        }
    }

    Ok(())
}
