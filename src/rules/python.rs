use crate::types::{Command, Correction};
use crate::rules::Rule;
use regex::Regex;
use once_cell::sync::Lazy;
use std::path::Path;

#[derive(Debug)]
pub struct PythonExecute;

impl Rule for PythonExecute {
    fn name(&self) -> &str {
        "python_execute"
    }

    fn matches(&self, command: &Command) -> bool {
        // Match if the command script ends with .py but wasn't found,
        // OR if the command is just a filename that exists as filename.py

        // The original rule matches if the command is a file that ends in .py but permission denied?
        // OR if the user typed `script` and `script.py` exists.

        // Let's implement the "forgot .py" case first.
        let script = command.script.trim();
        if script.ends_with(".py") { return false; } // Already has .py

        let path_with_py = format!("{}.py", script);
        Path::new(&path_with_py).exists()
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let new_cmd = format!("{}.py", command.script);
        vec![Correction::new(new_cmd, false, 100)]
    }
}

#[derive(Debug)]
pub struct PipUnknownCommand;

impl Rule for PipUnknownCommand {
    fn name(&self) -> &str {
        "pip_unknown_command"
    }

    fn matches(&self, command: &Command) -> bool {
        command.script.starts_with("pip") &&
        (command.stderr.contains("unknown command") ||
         command.stderr.contains("no such option")) &&
        command.stderr.contains("maybe you meant")
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        // Output example:
        // ERROR: unknown command "installl", maybe you meant "install"

        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"maybe you meant "([^"]+)""#).unwrap());

        let mut corrections = Vec::new();
        if let Some(caps) = RE.captures(&command.stderr) {
            if let Some(suggestion) = caps.get(1) {
                 let suggestion_str = suggestion.as_str();
                 // We need to replace the wrong subcommand.
                 // Simple approach: look for the subcommand that isn't "pip" and is close?
                 // Or just replace the typo if we can find it in the script.

                 // Better: Regex found the suggestion, but we don't know EXACTLY which word was the typo
                 // without parsing the stderr "unknown command 'foo'".

                 // Let's try to extract the typo from stderr too.
                 static RE_TYPO: Lazy<Regex> = Lazy::new(|| Regex::new(r#"unknown command "([^"]+)""#).unwrap());

                 if let Some(typo_caps) = RE_TYPO.captures(&command.stderr) {
                     if let Some(typo) = typo_caps.get(1) {
                         let typo_str = typo.as_str();
                         let new_cmd = command.script.replace(typo_str, suggestion_str);
                         corrections.push(Correction::new(new_cmd, false, 100));
                     }
                 }
            }
        }
        corrections
    }
}
