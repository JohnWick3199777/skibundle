mod bundle;
mod cli;
mod codebase;
mod error;
mod injected;
mod inspect;
mod manifest;
mod validate;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Bundle(args) => bundle::run(args),
        Commands::Validate(args) => validate_cmd::run(args),
        Commands::Inspect(args) => inspect::run(args),
        Commands::Version(args) => injected::version::run(args),
        Commands::Skill(args) => injected::skill::run(args),
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        let code = match e {
            error::AppError::ValidationFailed { .. } => 1,
            error::AppError::Io(_) | error::AppError::Json(_) => 3,
            error::AppError::Other(_) => 3,
        };
        std::process::exit(code);
    }
}

mod validate_cmd {
    use crate::{
        cli::ValidateArgs,
        error::{AppError, Result},
        manifest::BundleManifest,
        validate,
    };
    use colored::Colorize;
    use std::path::PathBuf;

    pub fn run(args: &ValidateArgs) -> Result<()> {
        match (&args.codebase, &args.manifest) {
            (Some(path), _) => validate_path(path, args.strict),
            (_, Some(mpath)) => validate_manifest(mpath, args.strict),
            (None, None) => Err(AppError::Other(
                "provide --codebase <path> or --manifest <path>".into(),
            )),
        }
    }

    fn validate_path(path: &PathBuf, strict: bool) -> Result<()> {
        let path = path.canonicalize()?;
        let report = validate::run_all(&path);
        print_report(&report);
        if strict && !report.passed() {
            return Err(AppError::ValidationFailed {
                count: report.failed_count(),
            });
        }
        Ok(())
    }

    fn validate_manifest(mpath: &PathBuf, strict: bool) -> Result<()> {
        let manifest = BundleManifest::load(mpath)?;

        // Re-run structural checks on recorded codebase path
        let report = validate::run_all(&manifest.codebase_path);
        print_report(&report);

        // Check binary drift
        let current_sha = crate::manifest::compute_sha256(&manifest.binary_path)?;
        if current_sha == manifest.binary_sha256 {
            println!("  {} binary SHA-256 matches manifest", "PASS".green());
        } else {
            println!(
                "  {} binary SHA-256 drift detected\n      recorded: {}\n      current:  {}",
                "FAIL".red(),
                &manifest.binary_sha256[..16],
                &current_sha[..16],
            );
            if strict {
                return Err(AppError::ValidationFailed { count: 1 });
            }
        }

        if strict && !report.passed() {
            return Err(AppError::ValidationFailed {
                count: report.failed_count(),
            });
        }
        Ok(())
    }

    fn print_report(report: &validate::ValidationReport) {
        for r in &report.results {
            let tag = if r.passed { "PASS".green() } else { "WARN".yellow() };
            println!("  {tag} {name}: {msg}", name = r.name, msg = r.message);
        }
        println!(
            "\nValidation: {}/{} checks passed",
            report.results.len() - report.failed_count(),
            report.results.len(),
        );
    }
}
