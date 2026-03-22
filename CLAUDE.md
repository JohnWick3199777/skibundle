# skibundle — CLAUDE.md

## What is this?

**skibundle** is a Rust CLI tool that wraps a CLI codebase and bundles it into a portable binary artifact. It validates structural properties of the codebase, records a signed manifest, and injects default commands into the bundled CLI.

---

## Core Commands

| Command | Description |
|---|---|
| `skibundle bundle` | Validate + bundle a codebase+binary into a manifest |
| `skibundle validate` | Run structural checks on a codebase or existing manifest |
| `skibundle inspect` | Pretty-print a bundle manifest |
| `skibundle version` | Show version info from a manifest |
| `skibundle skill` | Show skill metadata from a manifest |

---

## Module Structure

```
src/
  main.rs              # Entry point, CLI dispatch, exit codes
  cli.rs               # All clap structs (Cli, Commands, *Args)
  error.rs             # AppError (thiserror), Result<T> alias
  manifest.rs          # BundleManifest struct, load/save, SHA-256
  bundle.rs            # `bundle` command logic
  inspect.rs           # `inspect` command logic
  validate/
    mod.rs             # Validator trait, CheckResult, ValidationReport, run_all()
    is_git.rs          # Checks for .git/ directory
    has_readme.rs      # Checks for README.md at root
    has_skill.rs       # Checks for skill.md / .skill / skill/ dir
    has_ci.rs          # Checks for .github/workflows/, .gitlab-ci.yml, .circleci/
    has_tests.rs       # Checks for tests/ dir or #[test] attributes in .rs files
  injected/
    mod.rs             # Re-exports
    version.rs         # version subcommand (reads manifest, prints name+version)
    skill.rs           # skill subcommand (prints skill metadata from manifest)
```

---

## Key Types

### `Validator` trait (`validate/mod.rs`)
```rust
pub trait Validator: Send + Sync {
    fn name(&self) -> &'static str;
    fn check(&self, codebase: &Path) -> CheckResult;
}
```
To add a new validator: create a new file in `validate/`, implement `Validator`, register in `run_all()`. That's it.

### `CheckResult`
```rust
pub struct CheckResult {
    pub name: &'static str,
    pub passed: bool,
    pub message: String,
}
```

### `BundleManifest` (`manifest.rs`)
```rust
pub struct BundleManifest {
    pub schema_version: u32,
    pub name: String,
    pub created_at: String,           // RFC 3339
    pub codebase_path: PathBuf,       // canonical absolute path
    pub binary_path: PathBuf,         // canonical absolute path
    pub binary_sha256: String,        // hex SHA-256 of binary at bundle time
    pub validators_passed: Vec<String>,
    pub validators_failed: Vec<String>,
    pub injected_commands: Vec<String>, // ["version", "skill", "validate", "inspect"]
}
```

Manifest format is **JSON** (pretty-printed). Chosen because it's readable in `jq`/shell pipelines and `serde_json::to_string_pretty` is zero-effort.

---

## Codebase Validators

These run during `bundle` and `validate`:

| Check | What it looks for |
|---|---|
| `is_git` | `.git/` directory at codebase root |
| `has_readme` | `README.md` or `README` at root |
| `has_skill` | `skill.md`, `.skill`, or `skill/` directory |
| `has_ci` | `.github/workflows/`, `.gitlab-ci.yml`, or `.circleci/` |
| `has_tests` | `tests/` directory or `#[test]` in any `.rs` file |

Validators are **warning-level by default** during bundle (reported but don't block). Pass `--strict` to treat any failure as a hard error.

---

## Exit Codes

| Code | Meaning |
|---|---|
| `0` | Success |
| `1` | Validation failed (checks did not pass) |
| `2` | Usage / argument error (clap handles this) |
| `3` | IO or manifest parse error |

---

## Dependencies

```toml
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
colored = "2"
chrono = { version = "0.4", features = ["serde"] }
sha2 = "0.10"
hex = "0.4"
```

**No `anyhow`** — we use `thiserror` with typed errors so `main.rs` can set specific exit codes per error class.
**No `tokio`** — everything is synchronous filesystem I/O.
**No `walkdir`** yet — add lazily if `has_tests` needs recursive `.rs` scanning.

---

## Implementation Order

1. `Cargo.toml` — add deps, run `cargo check`
2. `error.rs` — AppError + Result alias
3. `cli.rs` — clap structs, wire into main with `todo!()`, run `--help`
4. `validate/mod.rs` — Validator trait + empty `run_all()`
5. Individual validators (`is_git` → `has_readme` → `has_skill` → `has_ci` → `has_tests`)
6. `manifest.rs` — BundleManifest with serde, SHA-256 helper
7. `bundle.rs` — main integration: validate → hash → manifest → save
8. `validate.rs` (command handler) — `--codebase` and `--manifest` paths
9. `inspect.rs` — load + pretty-print manifest
10. `injected/version.rs` + `injected/skill.rs`
11. Exit codes in `main.rs`
12. Integration tests in `tests/integration_test.rs`

---

## Design Decisions

- **`bundle` not `wrap`** — the CLI command is `bundle` (aligns with the binary artifact metaphor).
- **`Send + Sync` on `Validator`** — zero cost now, enables `rayon` parallelism later without API change.
- **`has_tests` strategy** — check `tests/` dir first (O(1), language-agnostic); fall back to scanning for `#[test]` in `.rs` files only if dir is absent.
- **SHA-256 on binary** — `validate --manifest` re-hashes the binary and compares against the recorded hash to detect drift between bundle time and now.
- **`--skip-validation` flag** on `bundle` — escape hatch for CI environments where some checks are not applicable.
