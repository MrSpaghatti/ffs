use crate::types::{Command, Correction};
use crate::rules::Rule;
use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct CargoRule;

impl Rule for CargoRule {
    fn name(&self) -> &str {
        "cargo"
    }

    fn matches(&self, command: &Command) -> bool {
        command.script.starts_with("cargo") &&
        (command.stderr.contains("Did you mean") ||
         command.stderr.contains("no such command") ||
         command.stderr.contains("similar name exists"))
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        // Regex for old cargo: "Did you mean (.*)?"
        // Regex for new cargo: "similar name exists: `(.*)`"
        static RE_OLD: Lazy<Regex> = Lazy::new(|| Regex::new(r"Did you mean (.*)\?").unwrap());
        static RE_NEW: Lazy<Regex> = Lazy::new(|| Regex::new(r"similar name exists: `(.*)`").unwrap());

        let mut corrections = Vec::new();

        let mut suggestions = Vec::new();

        if let Some(caps) = RE_OLD.captures(&command.stderr) {
            if let Some(suggestion) = caps.get(1) {
                 suggestions.extend(suggestion.as_str().split('\n').map(|s| s.trim()));
            }
        }

        if let Some(caps) = RE_NEW.captures(&command.stderr) {
            if let Some(suggestion) = caps.get(1) {
                 suggestions.push(suggestion.as_str().trim());
            }
        }

        if !suggestions.is_empty() {
             let parts: Vec<&str> = command.script.split_whitespace().collect();
             if parts.len() >= 2 {
                 let wrong_subcommand = parts[1];
                 for s in suggestions {
                     // Replace only the first occurrence of the wrong subcommand
                     let new_cmd = command.script.replacen(wrong_subcommand, s, 1);
                     corrections.push(Correction::new(new_cmd, false, 100));
                 }
             }
        }

        corrections
    }
}
