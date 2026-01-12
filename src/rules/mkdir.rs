use crate::types::{Command, Correction};
use crate::rules::Rule;
use shell_words::split;

#[derive(Debug)]
pub struct MkdirP;

impl Rule for MkdirP {
    fn name(&self) -> &str {
        "mkdir_p"
    }

    fn matches(&self, command: &Command) -> bool {
        // Use split to safely parse command
        let parts = match split(&command.script) {
            Ok(p) => p,
            Err(_) => return false,
        };

        if parts.is_empty() { return false; }

        // Exact match for "mkdir"
        if parts[0] != "mkdir" { return false; }

        command.stderr.contains("No such file or directory")
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
         let parts = match split(&command.script) {
            Ok(p) => p,
            Err(_) => return vec![],
        };

        if parts.is_empty() { return vec![]; }

        let mut new_parts = parts.clone();
        new_parts[0] = "mkdir -p".to_string(); // Replace argv[0]

        // shell_words::join might quote "mkdir -p" as one argument which is WRONG for execution
        // We want `mkdir -p arg1 arg2 ...`
        // Actually, we should probably construct it as `mkdir` `-p` `arg1` ...
        // But `mkdir -p` is two arguments.

        // Let's reconstruct manually or correctly.
        // If we change parts[0] to "mkdir" and insert "-p" at parts[1]

        let mut fixed_parts = Vec::new();
        fixed_parts.push("mkdir".to_string());
        fixed_parts.push("-p".to_string());
        // Append the rest
        for part in parts.iter().skip(1) {
            fixed_parts.push(part.clone());
        }

        let new_cmd = shell_words::join(fixed_parts);
        vec![Correction::new(new_cmd, false, 90)]
    }
}
