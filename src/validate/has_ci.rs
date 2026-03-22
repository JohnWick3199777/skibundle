use crate::codebase::CliCodebase;
use super::{CheckResult, Validator};

pub struct HasCiValidator;

impl Validator for HasCiValidator {
    fn name(&self) -> &'static str {
        "has_ci"
    }

    fn check(&self, codebase: &CliCodebase) -> CheckResult {
        if codebase.properties.has_ci {
            CheckResult {
                name: self.name(),
                passed: true,
                message: "Found CI configuration".to_string(),
            }
        } else {
            CheckResult {
                name: self.name(),
                passed: false,
                message: "No CI configuration found".to_string(),
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
    fn passes_with_github_workflows() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join(".github").join("workflows")).unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasCiValidator.check(&cb);
        assert!(result.passed);
    }

    #[test]
    fn passes_with_gitlab_ci() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join(".gitlab-ci.yml"), "stages: [test]").unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasCiValidator.check(&cb);
        assert!(result.passed);
    }

    #[test]
    fn passes_with_circleci() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".circleci")).unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasCiValidator.check(&cb);
        assert!(result.passed);
    }

    #[test]
    fn fails_when_no_ci_config() {
        let tmp = TempDir::new().unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasCiValidator.check(&cb);
        assert!(!result.passed);
        assert_eq!(result.message, "No CI configuration found");
    }
}
