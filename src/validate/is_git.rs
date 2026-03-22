use crate::codebase::CliCodebase;
use super::{CheckResult, Validator};

pub struct IsGitValidator;

impl Validator for IsGitValidator {
    fn name(&self) -> &'static str {
        "is_git"
    }

    fn check(&self, codebase: &CliCodebase) -> CheckResult {
        if codebase.properties.is_git {
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
    use crate::codebase::CliCodebase;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn passes_when_git_dir_exists() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".git")).unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = IsGitValidator.check(&cb);
        assert!(result.passed);
        assert_eq!(result.message, "Found .git/ directory");
    }

    #[test]
    fn fails_when_no_git_dir() {
        let tmp = TempDir::new().unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = IsGitValidator.check(&cb);
        assert!(!result.passed);
        assert!(result.message.contains("not a git repository"));
    }
}
