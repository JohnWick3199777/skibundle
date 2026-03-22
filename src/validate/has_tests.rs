use crate::codebase::CliCodebase;
use super::{CheckResult, Validator};

pub struct HasTestsValidator;

impl Validator for HasTestsValidator {
    fn name(&self) -> &'static str {
        "has_tests"
    }

    fn check(&self, codebase: &CliCodebase) -> CheckResult {
        if codebase.properties.has_tests {
            CheckResult {
                name: self.name(),
                passed: true,
                message: "Found tests".to_string(),
            }
        } else {
            CheckResult {
                name: self.name(),
                passed: false,
                message: "No tests found".to_string(),
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
    fn passes_with_tests_directory() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join("tests")).unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasTestsValidator.check(&cb);
        assert!(result.passed);
    }

    #[test]
    fn passes_when_src_file_has_test_attribute() {
        let tmp = TempDir::new().unwrap();
        let src_dir = tmp.path().join("src");
        fs::create_dir(&src_dir).unwrap();
        fs::write(
            src_dir.join("lib.rs"),
            "#[cfg(test)]\nmod tests {\n    #[test]\n    fn it_works() {}\n}\n",
        )
        .unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasTestsValidator.check(&cb);
        assert!(result.passed);
    }

    #[test]
    fn fails_when_no_tests_found() {
        let tmp = TempDir::new().unwrap();
        let src_dir = tmp.path().join("src");
        fs::create_dir(&src_dir).unwrap();
        fs::write(src_dir.join("main.rs"), "fn main() {}\n").unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasTestsValidator.check(&cb);
        assert!(!result.passed);
        assert_eq!(result.message, "No tests found");
    }
}
