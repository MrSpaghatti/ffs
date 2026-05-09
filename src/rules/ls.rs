use crate::types::{Command, Correction};
use crate::rules::Rule;
use shell_words::split;

#[derive(Debug)]
pub struct LsAll;

impl Rule for LsAll {
    fn name(&self) -> &str {
        "ls_all"
    }

    fn matches(&self, command: &Command) -> bool {
        let parts = match split(&command.script) {
            Ok(p) => p,
            Err(_) => return false,
        };

        if parts.is_empty() {
            return false;
        }

        if parts[0] != "ls" {
            return false;
        }

        for part in parts.iter().skip(1) {
            if !part.starts_with('-') {
                return false;
            }
        }

        for part in parts.iter().skip(1) {
            if part.starts_with("--") {
                if part == "--all" || part == "--almost-all" {
                    return false;
                }
            } else if part.starts_with('-') {
                if part.contains('a') || part.contains('A') {
                    return false;
                }
            }
        }

        let stdout = command.stdout.trim();
        let stderr = command.stderr.trim();

        stderr.is_empty() && (stdout.is_empty() || stdout == "total 0")
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let mut parts = match split(&command.script) {
            Ok(p) => p,
            Err(_) => return vec![],
        };

        let mut replaced = false;
        for part in parts.iter_mut().skip(1) {
            if part.starts_with('-') && !part.starts_with("--") {
                part.push('a');
                replaced = true;
                break;
            }
        }

        if !replaced {
            parts.insert(1, "-a".to_string());
        }

        let new_cmd = shell_words::join(parts);
        vec![Correction::new(new_cmd, false, 100)]
    }
}
