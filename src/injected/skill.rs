use colored::Colorize;
use std::path::PathBuf;

use crate::cli::SkillArgs;
use crate::manifest::BundleManifest;

pub fn run(args: &SkillArgs) -> crate::error::Result<()> {
    let manifest_path = args
        .manifest
        .clone()
        .unwrap_or_else(|| PathBuf::from("skill.manifest.json"));

    let manifest = BundleManifest::load(&manifest_path)?;

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
