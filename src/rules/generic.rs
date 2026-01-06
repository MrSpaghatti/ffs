use crate::types::{Command, Correction};
use crate::rules::Rule;
use which::which;

#[derive(Debug)]
pub struct UnknownCommand;

impl Rule for UnknownCommand {
    fn name(&self) -> &str {
        "unknown_command"
    }

    fn matches(&self, command: &Command) -> bool {
        // Typical "command not found" messages
        let script = &command.script;
        if script.is_empty() { return false; }

        command.stderr.contains("command not found") ||
        command.stderr.contains("is not recognized as an internal or external command") ||
        command.stderr.contains("not found")
    }

    fn generate_corrections(&self, _command: &Command) -> Vec<Correction> {
        // Stub implementation
        vec![]
    }
}
