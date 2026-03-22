use crate::codebase::CliCodebase;
use super::{CheckResult, Validator};

pub struct HasSkillValidator;

impl Validator for HasSkillValidator {
    fn name(&self) -> &'static str {
        "has_skill"
    }

    fn check(&self, codebase: &CliCodebase) -> CheckResult {
        if codebase.properties.has_skill {
            CheckResult {
                name: self.name(),
                passed: true,
                message: "Found skill definition".to_string(),
            }
        } else {
            CheckResult {
                name: self.name(),
                passed: false,
                message: "No skill definition found (expected skill.md, .skill, or skill/)".to_string(),
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
    fn passes_with_skill_md() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("skill.md"), "# Skill").unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasSkillValidator.check(&cb);
        assert!(result.passed);
    }

    #[test]
    fn passes_with_dot_skill() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join(".skill"), "skill config").unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasSkillValidator.check(&cb);
        assert!(result.passed);
    }

    #[test]
    fn passes_with_skill_dir() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join("skill")).unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasSkillValidator.check(&cb);
        assert!(result.passed);
    }

    #[test]
    fn fails_when_no_skill_definition() {
        let tmp = TempDir::new().unwrap();
        let cb = CliCodebase::from_path(tmp.path()).unwrap();
        let result = HasSkillValidator.check(&cb);
        assert!(!result.passed);
        assert!(result.message.contains("No skill definition found"));
    }
}
