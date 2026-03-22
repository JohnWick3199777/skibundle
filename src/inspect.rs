use colored::Colorize;

use crate::{cli::InspectArgs, error::Result, manifest::BundleManifest};

pub fn run(args: &InspectArgs) -> Result<()> {
    let manifest = BundleManifest::load(&args.manifest)?;

    println!("{}", "=== Bundle Manifest ===".bold());

    // Helper: print a single label/value row.
    // Label is right-padded to align the colons.
    let row = |label: &str, value: String| {
        println!("  {:20} : {}", label.cyan(), value);
    };

    row("Name", manifest.name.clone());
    row("Created", manifest.created_at.clone());
    row("Schema Version", manifest.schema_version.to_string());
    row(
        "Codebase Path",
        manifest.codebase_path.display().to_string(),
    );
    row("Binary Path", manifest.binary_path.display().to_string());

    // SHA-256: show only the first 16 hex characters followed by "..."
    let sha_preview = if manifest.binary_sha256.len() > 16 {
        format!("{}...", &manifest.binary_sha256[..16])
    } else {
        manifest.binary_sha256.clone()
    };
    row("Binary SHA-256", sha_preview);

    // Runtime: infer from the binary path extension; fall back to the path itself.
    let runtime = match manifest.binary_path.extension().and_then(|e| e.to_str()) {
        Some("py") => "Python".to_string(),
        Some("js") | Some("mjs") | Some("cjs") => "Node.js".to_string(),
        Some("rb") => "Ruby".to_string(),
        Some("sh") | Some("bash") | Some("zsh") => "Shell".to_string(),
        Some("wasm") => "WebAssembly".to_string(),
        // No extension usually means a native binary (ELF/Mach-O/PE).
        None => "Native binary".to_string(),
        Some(ext) => ext.to_string(),
    };
    row("Runtime", runtime);

    // Injected commands: comma-joined.
    let injected = if manifest.injected_commands.is_empty() {
        "none".to_string()
    } else {
        manifest.injected_commands.join(", ")
    };
    row("Injected Commands", injected);

    // Validators passed: comma-joined, green.
    let passed_str = if manifest.validators_passed.is_empty() {
        "none".green().to_string()
    } else {
        manifest.validators_passed.join(", ").green().to_string()
    };
    println!(
        "  {:20} : {}",
        "Validators Passed".cyan(),
        passed_str
    );

    // Validators failed: comma-joined, red; "none" in green when empty.
    let failed_str = if manifest.validators_failed.is_empty() {
        "none".green().to_string()
    } else {
        manifest.validators_failed.join(", ").red().to_string()
    };
    println!(
        "  {:20} : {}",
        "Validators Failed".cyan(),
        failed_str
    );

    Ok(())
}
