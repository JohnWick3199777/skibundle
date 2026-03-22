use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// Detected runtime/language of the codebase.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Runtime {
    Rust,
    Node,
    Python,
    Go,
    Unknown,
}

impl Runtime {
    fn detect(root: &Path) -> Self {
        if root.join("Cargo.toml").exists() {
            Runtime::Rust
        } else if root.join("package.json").exists() {
            Runtime::Node
        } else if root.join("pyproject.toml").exists() || root.join("setup.py").exists() {
            Runtime::Python
        } else if root.join("go.mod").exists() {
            Runtime::Go
        } else {
            Runtime::Unknown
        }
    }
}

impl std::fmt::Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Runtime::Rust => "rust",
            Runtime::Node => "node",
            Runtime::Python => "python",
            Runtime::Go => "go",
            Runtime::Unknown => "unknown",
        };
        write!(f, "{s}")
    }
}

/// Structural properties detected from a codebase directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodebaseProperties {
    /// Has a `.git/` directory at root.
    pub is_git: bool,
    /// Has a `README.md` or `README` at root.
    pub has_readme: bool,
    /// Has a `skill.md`, `.skill`, or `skill/` at root.
    pub has_skill: bool,
    /// Has `.github/workflows/`, `.gitlab-ci.yml`, or `.circleci/` at root.
    pub has_ci: bool,
    /// Has a `tests/` directory or `.rs` files containing `#[test]`.
    pub has_tests: bool,
    /// Has a `CLAUDE.md` at root.
    pub has_claude_md: bool,
    /// Detected runtime / language.
    pub runtime: Runtime,
}

impl CodebaseProperties {
    fn detect(root: &Path) -> Self {
        CodebaseProperties {
            is_git: root.join(".git").exists(),
            has_readme: root.join("README.md").exists() || root.join("README").exists(),
            has_skill: root.join("skill.md").exists()
                || root.join(".skill").exists()
                || root.join("skill").is_dir(),
            has_ci: root.join(".github/workflows").is_dir()
                || root.join(".gitlab-ci.yml").exists()
                || root.join(".circleci").is_dir(),
            has_tests: root.join("tests").is_dir() || has_rust_tests(root),
            has_claude_md: root.join("CLAUDE.md").exists(),
            runtime: Runtime::detect(root),
        }
    }

    /// Returns the names of all properties that evaluated to `false`.
    pub fn missing(&self) -> Vec<&'static str> {
        let mut out = Vec::new();
        if !self.is_git { out.push("is_git"); }
        if !self.has_readme { out.push("has_readme"); }
        if !self.has_skill { out.push("has_skill"); }
        if !self.has_ci { out.push("has_ci"); }
        if !self.has_tests { out.push("has_tests"); }
        if !self.has_claude_md { out.push("has_claude_md"); }
        out
    }

    /// Returns `true` if all boolean properties pass.
    pub fn all_pass(&self) -> bool {
        self.is_git
            && self.has_readme
            && self.has_skill
            && self.has_ci
            && self.has_tests
            && self.has_claude_md
    }
}

/// A CLI codebase rooted at a directory on disk.
///
/// Constructed via [`CliCodebase::from_path`]; structural properties are
/// detected eagerly at construction time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliCodebase {
    /// Canonical absolute path to the codebase root.
    pub path: PathBuf,
    /// Name — derived from the directory name.
    pub name: String,
    /// All detected structural properties.
    pub properties: CodebaseProperties,
}

impl CliCodebase {
    /// Detect and construct a [`CliCodebase`] from `path`.
    ///
    /// Returns an error if the path does not exist or cannot be canonicalized.
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref().canonicalize()?;
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unnamed".to_string());
        let properties = CodebaseProperties::detect(&path);
        Ok(CliCodebase { path, name, properties })
    }

    /// Serialize the model to a pretty-printed JSON string.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

/// Scan the root directory for any `.rs` file containing `#[test]`.
/// Only looks one level deep inside `src/` to stay fast.
fn has_rust_tests(root: &Path) -> bool {
    let src = root.join("src");
    if !src.is_dir() {
        return false;
    }
    let Ok(entries) = std::fs::read_dir(&src) else { return false };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.extension().and_then(|e| e.to_str()) == Some("rs") {
            if let Ok(contents) = std::fs::read_to_string(&p) {
                if contents.contains("#[test]") {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn scaffold(dirs: &[&str], files: &[&str]) -> TempDir {
        let tmp = tempfile::tempdir().unwrap();
        for d in dirs { fs::create_dir_all(tmp.path().join(d)).unwrap(); }
        for f in files { fs::write(tmp.path().join(f), "").unwrap(); }
        tmp
    }

    #[test]
    fn detects_git_repo() {
        let tmp = scaffold(&[".git"], &[]);
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        assert!(cb.properties.is_git);
    }

    #[test]
    fn detects_missing_properties() {
        let tmp = scaffold(&[], &[]);
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        assert!(cb.properties.missing().contains(&"is_git"));
        assert!(cb.properties.missing().contains(&"has_readme"));
    }

    #[test]
    fn detects_rust_runtime() {
        let tmp = scaffold(&[], &["Cargo.toml"]);
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        assert_eq!(cb.properties.runtime, Runtime::Rust);
    }

    #[test]
    fn serializes_to_json() {
        let tmp = scaffold(&[".git"], &["README.md", "Cargo.toml"]);
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let json = cb.to_json().unwrap();
        assert!(json.contains("\"is_git\": true"));
        assert!(json.contains("\"runtime\": \"rust\""));
    }
}
