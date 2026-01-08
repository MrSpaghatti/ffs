use crate::types::{Command, Correction};
use crate::rules::Rule;

#[derive(Debug)]
pub struct Sudo;

impl Rule for Sudo {
    fn name(&self) -> &str {
        "sudo"
    }

    fn matches(&self, command: &Command) -> bool {
        let stderr = command.stderr.to_lowercase();
        let stdout = command.stdout.to_lowercase();

        (stderr.contains("permission denied") ||
         stderr.contains("eacces") ||
         stdout.contains("permission denied") ||
         stdout.contains("eacces") ||
         stderr.contains("requires root privileges") ||
         stderr.contains("must be run as root")) &&
         !command.script.trim().starts_with("sudo")
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let new_cmd = format!("sudo {}", command.script);
        vec![Correction::new(new_cmd, false, 100)]
    }
}
