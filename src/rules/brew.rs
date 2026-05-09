use crate::types::{Command, Correction};
use crate::rules::Rule;
use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct BrewUnknownCommand;

impl Rule for BrewUnknownCommand {
    fn name(&self) -> &str {
        "brew_unknown_command"
    }

    fn matches(&self, command: &Command) -> bool {
        command.script.starts_with("brew ") &&
        (command.stderr.contains("Unknown command") ||
         command.stderr.contains("Error: No such command"))
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let mut corrections = Vec::new();
        static RE_SUGGEST: Lazy<Regex> = Lazy::new(|| Regex::new(r"Did you mean (.*)\?").unwrap());

        if let Some(suggest_caps) = RE_SUGGEST.captures(&command.stderr) {
            if let Some(suggestion) = suggest_caps.get(1) {
                let parts: Vec<&str> = command.script.split_whitespace().collect();
                if parts.len() >= 2 {
                    let broken_cmd = parts[1];
                    let new_cmd = command.script.replacen(broken_cmd, suggestion.as_str().trim(), 1);
                    corrections.push(Correction::new(new_cmd, false, 100));
                }
            }
        }

        corrections
    }
}
