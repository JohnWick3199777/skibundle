use std::path::Path;
use super::{CheckResult, Validator};

pub struct HasCiValidator;

impl Validator for HasCiValidator {
    fn name(&self) -> &'static str {
        "has_ci"
    }

    fn check(&self, codebase: &Path) -> CheckResult {
        // Check .github/workflows/ directory
        let github_workflows = codebase.join(".github").join("workflows");
        if github_workflows.is_dir() {
            return CheckResult {
                name: self.name(),
                passed: true,
                message: "Found .github/workflows/ directory".to_string(),
            };
        }

        // Check .gitlab-ci.yml file
        let gitlab_ci = codebase.join(".gitlab-ci.yml");
        if gitlab_ci.is_file() {
            return CheckResult {
                name: self.name(),
                passed: true,
                message: "Found .gitlab-ci.yml".to_string(),
            };
        }

        // Check .circleci/ directory
        let circleci = codebase.join(".circleci");
        if circleci.is_dir() {
            return CheckResult {
                name: self.name(),
                passed: true,
                message: "Found .circleci/ directory".to_string(),
            };
        }

        CheckResult {
            name: self.name(),
            passed: false,
            message: "No CI configuration found".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn passes_with_github_workflows() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join(".github").join("workflows")).unwrap();
        let result = HasCiValidator.check(tmp.path());
        assert!(result.passed);
        assert!(result.message.contains(".github/workflows/"));
    }

    #[test]
    fn passes_with_gitlab_ci() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join(".gitlab-ci.yml"), "stages: [test]").unwrap();
        let result = HasCiValidator.check(tmp.path());
        assert!(result.passed);
        assert!(result.message.contains(".gitlab-ci.yml"));
    }

    #[test]
    fn passes_with_circleci() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".circleci")).unwrap();
        let result = HasCiValidator.check(tmp.path());
        assert!(result.passed);
        assert!(result.message.contains(".circleci/"));
    }

    #[test]
    fn fails_when_no_ci_config() {
        let tmp = TempDir::new().unwrap();
        let result = HasCiValidator.check(tmp.path());
        assert!(!result.passed);
        assert_eq!(result.message, "No CI configuration found");
    }
}
