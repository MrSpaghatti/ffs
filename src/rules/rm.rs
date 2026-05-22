use crate::types::{Command, Correction};
use crate::rules::Rule;
use shell_words::split;

#[derive(Debug)]
pub struct RmDir;

impl Rule for RmDir {
    fn name(&self) -> &str {
        "rm_dir"
    }

    fn matches(&self, command: &Command) -> bool {
        let parts = match split(&command.script) {
            Ok(p) => p,
            Err(_) => return false,
        };

        if parts.is_empty() { return false; }

        if parts[0] != "rm" { return false; }

        let has_r_flag = parts.iter().any(|part| part == "-r" || part == "-rf" || part == "-fr" || part == "-R");
        if has_r_flag { return false; }

        command.stderr.contains("cannot remove") ||
        command.stderr.contains("Is a directory") ||
        command.stderr.contains("is a directory")
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let parts = match split(&command.script) {
            Ok(p) => p,
            Err(_) => return vec![],
        };

        if parts.is_empty() { return vec![]; }

        let mut fixed_parts = Vec::new();
        fixed_parts.push("rm".to_string());
        fixed_parts.push("-rf".to_string());

        for part in parts.iter().skip(1) {
            fixed_parts.push(part.clone());
        }

        let new_cmd = shell_words::join(fixed_parts);
        vec![Correction::new(new_cmd, false, 90)]
    }
}
