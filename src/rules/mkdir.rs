use crate::types::{Command, Correction};
use crate::rules::Rule;

#[derive(Debug)]
pub struct MkdirP;

impl Rule for MkdirP {
    fn name(&self) -> &str {
        "mkdir_p"
    }

    fn matches(&self, command: &Command) -> bool {
        command.script.starts_with("mkdir") &&
        command.stderr.contains("No such file or directory")
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let new_cmd = command.script.replace("mkdir", "mkdir -p");
        vec![Correction::new(new_cmd, false, 90)]
    }
}
