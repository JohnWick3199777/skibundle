use crate::codebase::CliCodebase;
use super::{CheckResult, Validator};

pub struct HasReadmeValidator;

impl Validator for HasReadmeValidator {
    fn name(&self) -> &'static str {
        "has_readme"
    }

    fn check(&self, codebase: &CliCodebase) -> CheckResult {
        if codebase.properties.has_readme {
            CheckResult {
                name: self.name(),
                passed: true,
                message: "Found README".to_string(),
            }
        } else {
            CheckResult {
                name: self.name(),
                passed: false,
                message: "No README.md or README found".to_string(),
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
    fn passes_with_readme_md() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README.md"), "# Project").unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasReadmeValidator.check(&cb);
        assert!(result.passed);
        assert!(result.message.contains("README"));
    }

    #[test]
    fn passes_with_plain_readme() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README"), "Project info").unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasReadmeValidator.check(&cb);
        assert!(result.passed);
        assert!(result.message.contains("README"));
    }

    #[test]
    fn fails_when_no_readme() {
        let tmp = TempDir::new().unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasReadmeValidator.check(&cb);
        assert!(!result.passed);
        assert_eq!(result.message, "No README.md or README found");
    }
}
