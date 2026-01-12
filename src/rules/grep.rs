use crate::types::{Command, Correction};
use crate::rules::Rule;
use shell_words::split;

#[derive(Debug)]
pub struct GrepRecursive;

impl Rule for GrepRecursive {
    fn name(&self) -> &str {
        "grep_recursive"
    }

    fn matches(&self, command: &Command) -> bool {
        let parts = match split(&command.script) {
            Ok(p) => p,
            Err(_) => return false,
        };

        if parts.is_empty() { return false; }

        let cmd = parts[0].as_str();
        let is_grep = cmd == "grep" || cmd == "egrep" || cmd == "fgrep";

        // Check for "Is a directory" in stderr
        is_grep && command.stderr.to_lowercase().contains("is a directory")
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let parts = match split(&command.script) {
            Ok(p) => p,
            Err(_) => return vec![],
        };

        if parts.is_empty() { return vec![]; }

        let mut new_parts = parts.clone();
        // Insert -r after the command name (grep/egrep/fgrep)
        new_parts.insert(1, "-r".to_string());

        let new_cmd = shell_words::join(new_parts);
        vec![Correction::new(new_cmd, false, 90)]
    }
}
