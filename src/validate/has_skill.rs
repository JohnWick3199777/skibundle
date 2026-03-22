use std::path::Path;
use super::{CheckResult, Validator};

pub struct HasSkillValidator;

impl Validator for HasSkillValidator {
    fn name(&self) -> &'static str {
        "has_skill"
    }

    fn check(&self, codebase: &Path) -> CheckResult {
        // Check skill.md file
        let skill_md = codebase.join("skill.md");
        if skill_md.is_file() {
            return CheckResult {
                name: self.name(),
                passed: true,
                message: "Found skill.md".to_string(),
            };
        }

        // Check .skill file
        let dot_skill = codebase.join(".skill");
        if dot_skill.is_file() {
            return CheckResult {
                name: self.name(),
                passed: true,
                message: "Found .skill".to_string(),
            };
        }

        // Check skill/ directory
        let skill_dir = codebase.join("skill");
        if skill_dir.is_dir() {
            return CheckResult {
                name: self.name(),
                passed: true,
                message: "Found skill/ directory".to_string(),
            };
        }

        CheckResult {
            name: self.name(),
            passed: false,
            message: "No skill definition found (expected skill.md, .skill, or skill/)".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn passes_with_skill_md() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("skill.md"), "# Skill").unwrap();
        let result = HasSkillValidator.check(tmp.path());
        assert!(result.passed);
        assert!(result.message.contains("skill.md"));
    }

    #[test]
    fn passes_with_dot_skill() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join(".skill"), "skill config").unwrap();
        let result = HasSkillValidator.check(tmp.path());
        assert!(result.passed);
        assert!(result.message.contains(".skill"));
    }

    #[test]
    fn passes_with_skill_dir() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join("skill")).unwrap();
        let result = HasSkillValidator.check(tmp.path());
        assert!(result.passed);
        assert!(result.message.contains("skill/"));
    }

    #[test]
    fn fails_when_no_skill_definition() {
        let tmp = TempDir::new().unwrap();
        let result = HasSkillValidator.check(tmp.path());
        assert!(!result.passed);
        assert!(result.message.contains("No skill definition found"));
    }
}
