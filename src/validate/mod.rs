use crate::codebase::CliCodebase;

pub mod is_git;
pub mod has_readme;
pub mod has_skill;
pub mod has_ci;
pub mod has_tests;

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub name: &'static str,
    pub passed: bool,
    pub message: String,
}

pub trait Validator: Send + Sync {
    fn name(&self) -> &'static str;
    fn check(&self, codebase: &CliCodebase) -> CheckResult;
}

pub struct ValidationReport {
    pub results: Vec<CheckResult>,
}

impl ValidationReport {
    pub fn passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }

    pub fn failed_count(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    pub fn passed_names(&self) -> Vec<String> {
        self.results
            .iter()
            .filter(|r| r.passed)
            .map(|r| r.name.to_string())
            .collect()
    }

    pub fn failed_names(&self) -> Vec<String> {
        self.results
            .iter()
            .filter(|r| !r.passed)
            .map(|r| r.name.to_string())
            .collect()
    }
}

pub fn run_all(codebase: &CliCodebase) -> ValidationReport {
    let validators: Vec<Box<dyn Validator>> = vec![
        Box::new(is_git::IsGitValidator),
        Box::new(has_readme::HasReadmeValidator),
        Box::new(has_skill::HasSkillValidator),
        Box::new(has_ci::HasCiValidator),
        Box::new(has_tests::HasTestsValidator),
    ];
    let results = validators.iter().map(|v| v.check(codebase)).collect();
    ValidationReport { results }
}
