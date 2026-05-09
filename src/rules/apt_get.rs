use crate::types::{Command, Correction};
use crate::rules::Rule;
use shell_words::split;
use strsim::levenshtein;

#[derive(Debug)]
pub struct AptGet;

impl Rule for AptGet {
    fn name(&self) -> &str {
        "apt_get"
    }

    fn matches(&self, command: &Command) -> bool {
        let parts = match split(&command.script) {
            Ok(p) => p,
            Err(_) => return false,
        };

        if parts.is_empty() { return false; }

        let cmd = &parts[0];
        if cmd != "apt-get" && cmd != "apt" && cmd != "apt-cache" {
            if parts.len() < 2 || parts[0] != "sudo" || (parts[1] != "apt-get" && parts[1] != "apt" && parts[1] != "apt-cache") {
                return false;
            }
        }

        let stderr = &command.stderr;

        stderr.contains("E: Invalid operation") ||
        stderr.contains("E: Could not open lock file") ||
        stderr.contains("Permission denied") ||
        stderr.contains("E: Could not get lock")
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let parts = match split(&command.script) {
            Ok(p) => p,
            Err(_) => return vec![],
        };

        if parts.is_empty() { return vec![]; }

        let stderr = &command.stderr;

        if stderr.contains("E: Could not open lock file") ||
           stderr.contains("Permission denied") ||
           stderr.contains("E: Could not get lock") {

            if parts[0] != "sudo" {
                let new_cmd = format!("sudo {}", command.script);
                return vec![Correction::new(new_cmd, false, 100)];
            }
            return vec![];
        }

        if stderr.contains("E: Invalid operation") {
            let valid_ops = vec![
                "install", "remove", "update", "upgrade", "autoremove",
                "search", "show", "purge", "clean", "autoclean",
                "dist-upgrade", "full-upgrade", "source", "build-dep",
                "check", "download", "changelog"
            ];

            let mut new_parts = parts.clone();

            if let Some(pos) = stderr.find("E: Invalid operation ") {
                let after_prefix = &stderr[pos + "E: Invalid operation ".len()..];
                let invalid_op = after_prefix.split_whitespace().next().unwrap_or("");

                if let Some(op_index) = new_parts.iter().position(|p| p == invalid_op) {
                    if invalid_op == "uninstall" {
                        new_parts[op_index] = "remove".to_string();
                        let new_cmd = shell_words::join(new_parts);
                        return vec![Correction::new(new_cmd, false, 100)];
                    }

                    let mut best_match = "";
                    let mut best_dist = usize::MAX;

                    for op in &valid_ops {
                        let dist = levenshtein(invalid_op, op);
                        if dist < best_dist {
                            best_dist = dist;
                            best_match = op;
                        }
                    }

                    if !best_match.is_empty() && best_dist <= 3 {
                        new_parts[op_index] = best_match.to_string();
                        let new_cmd = shell_words::join(new_parts);
                        return vec![Correction::new(new_cmd, false, 100)];
                    }
                }
            }
        }

        vec![]
    }
}
