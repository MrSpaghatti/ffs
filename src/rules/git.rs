use crate::types::{Command, Correction};
use crate::rules::Rule;
use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct GitCheckout;

impl Rule for GitCheckout {
    fn name(&self) -> &str {
        "git_checkout"
    }

    fn matches(&self, command: &Command) -> bool {
        command.script.starts_with("git") &&
        command.stderr.contains("did not match any file(s) known to git")
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let mut corrections = Vec::new();
        if command.script.contains("checkout") && !command.script.contains(" -b") {
             let new_cmd = command.script.replace("checkout", "checkout -b");
             corrections.push(Correction::new(new_cmd, true, 80));
        }
        corrections
    }
}

#[derive(Debug)]
pub struct GitPush;

impl Rule for GitPush {
    fn name(&self) -> &str {
        "git_push"
    }

    fn matches(&self, command: &Command) -> bool {
        command.script.starts_with("git push") &&
        command.stderr.contains("has no upstream branch") &&
        command.stderr.contains("git push --set-upstream")
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"git push --set-upstream \S+ \S+").unwrap());

        let mut corrections = Vec::new();
        if let Some(caps) = RE.captures(&command.stderr) {
            if let Some(suggestion) = caps.get(0) {
                 corrections.push(Correction::new(suggestion.as_str().to_string(), false, 100));
            }
        }
        corrections
    }
}

#[derive(Debug)]
pub struct GitNoCommand;

impl Rule for GitNoCommand {
    fn name(&self) -> &str {
        "git_no_command"
    }

    fn matches(&self, command: &Command) -> bool {
        command.script.starts_with("git") &&
        command.stderr.contains("is not a git command") &&
        command.stderr.contains("Did you mean this?")
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let mut corrections = Vec::new();
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Did you mean this\?\n\s*(.*)").unwrap());

        if let Some(caps) = RE.captures(&command.stderr) {
             if let Some(suggestion) = caps.get(1) {
                 let suggestion_str = suggestion.as_str().trim();
                 let parts: Vec<&str> = command.script.split_whitespace().collect();
                 if parts.len() >= 2 {
                     let wrong_subcommand = parts[1];
                     let new_cmd = command.script.replacen(wrong_subcommand, suggestion_str, 1);
                     corrections.push(Correction::new(new_cmd, false, 100));
                 }
             }
        }
        corrections
    }
}
