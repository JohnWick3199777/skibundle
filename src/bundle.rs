use colored::Colorize;
use std::path::Path;

use crate::{
    cli::BundleArgs,
    error::{AppError, Result},
    manifest::{self, BundleManifest},
    validate,
};

pub fn run(args: &BundleArgs) -> Result<()> {
    // 1. Canonicalize paths, returning a clear error if either doesn't exist.
    let codebase_path = args.codebase.canonicalize().map_err(|_| {
        AppError::Other(format!(
            "Codebase path does not exist or is inaccessible: {}",
            args.codebase.display()
        ))
    })?;

    let binary_path = args.binary.canonicalize().map_err(|_| {
        AppError::Other(format!(
            "Binary path does not exist or is inaccessible: {}",
            args.binary.display()
        ))
    })?;

    // 2. Run validation unless --skip-validation was passed.
    let (passed_names, failed_names) = if args.skip_validation {
        (Vec::new(), Vec::new())
    } else {
        let report = validate::run_all(&codebase_path);

        for result in &report.results {
            if result.passed {
                println!("{} {} — {}", "PASS".green(), result.name, result.message);
            } else {
                println!("{} {} — {}", "WARN".yellow(), result.name, result.message);
            }
        }

        let total = report.results.len();
        let passed_count = total - report.failed_count();
        println!("Validation: {}/{} checks passed", passed_count, total);

        (report.passed_names(), report.failed_names())
    };

    // 3. Compute the SHA-256 digest of the binary.
    let sha256 = manifest::compute_sha256(&binary_path).map_err(|e| {
        AppError::Other(format!(
            "Failed to compute SHA-256 for {}: {}",
            binary_path.display(),
            e
        ))
    })?;

    // 4. Construct the manifest.
    let bundle_manifest = BundleManifest::new(
        args.name.clone(),
        codebase_path,
        binary_path,
        sha256,
        passed_names,
        failed_names,
    );

    // 5. Save the manifest to the output path.
    bundle_manifest.save(Path::new(&args.out)).map_err(|e| {
        AppError::Other(format!(
            "Failed to write manifest to {}: {}",
            args.out.display(),
            e
        ))
    })?;

    // 6. Print a success line.
    println!("{}", format!("Bundled \u{2192} {}", args.out.display()).bold());

    Ok(())
}
