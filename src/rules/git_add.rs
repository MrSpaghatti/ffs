use crate::types::{Command, Correction};
use crate::rules::Rule;
use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct GitAdd;

impl Rule for GitAdd {
    fn name(&self) -> &str {
        "git_add"
    }

    fn matches(&self, command: &Command) -> bool {
        let script = command.script.trim();
        if !script.starts_with("git add") {
            return false;
        }

        let stderr = &command.stderr;

        static RE_PATHSPEC: Lazy<Regex> = Lazy::new(|| Regex::new(r"pathspec '.*' did not match any file").unwrap());
        static RE_NOTHING: Lazy<Regex> = Lazy::new(|| Regex::new(r"Nothing specified, nothing added\.").unwrap());
        static RE_GENERIC: Lazy<Regex> = Lazy::new(|| Regex::new(r"did not match any file").unwrap());

        RE_PATHSPEC.is_match(stderr) || RE_NOTHING.is_match(stderr) || RE_GENERIC.is_match(stderr)
    }

    fn generate_corrections(&self, _command: &Command) -> Vec<Correction> {
        let mut corrections = Vec::new();

        corrections.push(Correction::new("git add -A".to_string(), true, 100));
        corrections.push(Correction::new("git add .".to_string(), true, 90));

        corrections
    }
}
