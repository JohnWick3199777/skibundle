use std::path::Path;
use std::fs;
use super::{CheckResult, Validator};

pub struct HasTestsValidator;

impl Validator for HasTestsValidator {
    fn name(&self) -> &'static str {
        "has_tests"
    }

    fn check(&self, codebase: &Path) -> CheckResult {
        // Check for a tests/ directory at the codebase root
        let tests_dir = codebase.join("tests");
        if tests_dir.is_dir() {
            return CheckResult {
                name: self.name(),
                passed: true,
                message: "Found tests/ directory".to_string(),
            };
        }

        // Scan src/*.rs files for #[test]
        let src_dir = codebase.join("src");
        if src_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(&src_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                        if let Ok(contents) = fs::read_to_string(&path) {
                            if contents.contains("#[test]") {
                                let file_name = path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("unknown");
                                return CheckResult {
                                    name: self.name(),
                                    passed: true,
                                    message: format!(
                                        "Found #[test] attribute in src/{}",
                                        file_name
                                    ),
                                };
                            }
                        }
                    }
                }
            }
        }

        CheckResult {
            name: self.name(),
            passed: false,
            message: "No tests found".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn passes_with_tests_directory() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join("tests")).unwrap();
        let result = HasTestsValidator.check(tmp.path());
        assert!(result.passed);
        assert!(result.message.contains("tests/"));
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
        let result = HasTestsValidator.check(tmp.path());
        assert!(result.passed);
        assert!(result.message.contains("#[test]"));
        assert!(result.message.contains("lib.rs"));
    }

    #[test]
    fn fails_when_no_tests_found() {
        let tmp = TempDir::new().unwrap();
        let src_dir = tmp.path().join("src");
        fs::create_dir(&src_dir).unwrap();
        fs::write(src_dir.join("main.rs"), "fn main() {}\n").unwrap();
        let result = HasTestsValidator.check(tmp.path());
        assert!(!result.passed);
        assert_eq!(result.message, "No tests found");
    }
}
