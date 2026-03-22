use std::path::Path;
use super::{CheckResult, Validator};

pub struct HasReadmeValidator;

impl Validator for HasReadmeValidator {
    fn name(&self) -> &'static str {
        "has_readme"
    }

    fn check(&self, codebase: &Path) -> CheckResult {
        let candidates = ["README.md", "README"];
        for candidate in &candidates {
            let path = codebase.join(candidate);
            if path.exists() {
                return CheckResult {
                    name: self.name(),
                    passed: true,
                    message: format!("Found {}", candidate),
                };
            }
        }
        CheckResult {
            name: self.name(),
            passed: false,
            message: "No README.md or README found".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn passes_with_readme_md() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README.md"), "# Project").unwrap();
        let result = HasReadmeValidator.check(tmp.path());
        assert!(result.passed);
        assert!(result.message.contains("README.md"));
    }

    #[test]
    fn passes_with_plain_readme() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README"), "Project info").unwrap();
        let result = HasReadmeValidator.check(tmp.path());
        assert!(result.passed);
        assert!(result.message.contains("README"));
    }

    #[test]
    fn fails_when_no_readme() {
        let tmp = TempDir::new().unwrap();
        let result = HasReadmeValidator.check(tmp.path());
        assert!(!result.passed);
        assert_eq!(result.message, "No README.md or README found");
    }
}
