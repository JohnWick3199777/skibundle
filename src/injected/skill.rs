use colored::Colorize;

use crate::cli::SkillArgs;
use crate::error::{AppError, Result};
use crate::manifest::BundleManifest;

pub fn run(args: &SkillArgs) -> Result<()> {
    let path = args.manifest.as_ref().ok_or_else(|| {
        AppError::Other("--manifest <path> is required for ski skill".into())
    })?;

    let manifest = BundleManifest::load(path)?;

    println!("{}", format!("=== Skill: {} ===", manifest.name).bold());
    println!("  Bundled:   {}", manifest.created_at);
    println!("  Codebase:  {}", manifest.codebase_path.display());
    println!("  Binary:    {}", manifest.binary_path.display());

    let commands = manifest.injected_commands.join(", ");
    println!("  Commands:  {}", commands.cyan());

    let passed = if manifest.validators_passed.is_empty() {
        "none".green().to_string()
    } else {
        manifest.validators_passed.join(", ").green().to_string()
    };
    println!("  Passed:    {}", passed);

    let failed = if manifest.validators_failed.is_empty() {
        "none".green().to_string()
    } else {
        manifest.validators_failed.join(", ").red().to_string()
    };
    println!("  Failed:    {}", failed);

    Ok(())
}
