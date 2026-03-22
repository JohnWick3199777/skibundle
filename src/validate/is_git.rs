use std::path::Path;
use super::{CheckResult, Validator};

pub struct IsGitValidator;

impl Validator for IsGitValidator {
    fn name(&self) -> &'static str {
        "is_git"
    }

    fn check(&self, codebase: &Path) -> CheckResult {
        let git_dir = codebase.join(".git");
        if git_dir.is_dir() {
            CheckResult {
                name: self.name(),
                passed: true,
                message: "Found .git/ directory".to_string(),
            }
        } else {
            CheckResult {
                name: self.name(),
                passed: false,
                message: "No .git/ directory found — not a git repository".to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn passes_when_git_dir_exists() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".git")).unwrap();
        let result = IsGitValidator.check(tmp.path());
        assert!(result.passed);
        assert_eq!(result.message, "Found .git/ directory");
    }

    #[test]
    fn fails_when_no_git_dir() {
        let tmp = TempDir::new().unwrap();
        let result = IsGitValidator.check(tmp.path());
        assert!(!result.passed);
        assert!(result.message.contains("not a git repository"));
    }
}
