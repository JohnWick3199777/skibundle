use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "ski", version, about = "The ski CLI — bundle, validate, and inspect CLI codebases")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Bundle(BundleArgs),
    Validate(ValidateArgs),
    Inspect(InspectArgs),
    Version(VersionArgs),
    Skill(SkillArgs),
}

#[derive(Debug, Args)]
pub struct BundleArgs {
    /// Name of the skill bundle
    #[arg(long)]
    pub name: String,

    /// Path to the binary
    #[arg(long)]
    pub binary: PathBuf,

    /// Path to the codebase directory
    #[arg(long)]
    pub codebase: PathBuf,

    /// Output path for the manifest
    #[arg(long, default_value = "skill.manifest.json")]
    pub out: PathBuf,

    /// Skip validation after bundling
    #[arg(long, default_value_t = false)]
    pub skip_validation: bool,
}

#[derive(Debug, Args)]
pub struct ValidateArgs {
    /// Path to the codebase directory
    #[arg(long)]
    pub codebase: Option<PathBuf>,

    /// Path to the manifest file
    #[arg(long)]
    pub manifest: Option<PathBuf>,

    /// Enable strict validation mode
    #[arg(long, default_value_t = false)]
    pub strict: bool,
}

#[derive(Debug, Args)]
pub struct InspectArgs {
    /// Path to the manifest file
    #[arg(long)]
    pub manifest: PathBuf,
}

#[derive(Debug, Args)]
pub struct VersionArgs {
    /// Path to the manifest file
    #[arg(long)]
    pub manifest: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct SkillArgs {
    /// Path to the manifest file
    #[arg(long)]
    pub manifest: Option<PathBuf>,
}
