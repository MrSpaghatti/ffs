use crate::types::{Command, Correction};
use crate::rules::Rule;
use std::env;
use std::fs;
use std::path::Path;
use strsim::levenshtein;
use std::collections::HashSet;
use shell_words::split;

#[derive(Debug)]
pub struct UnknownCommand;

impl Rule for UnknownCommand {
    fn name(&self) -> &str {
        "unknown_command"
    }

    fn matches(&self, command: &Command) -> bool {
        let script = &command.script;
        if script.is_empty() { return false; }

        let lower_stderr = command.stderr.to_lowercase();
        lower_stderr.contains("command not found") ||
        lower_stderr.contains("unknown command") ||
        lower_stderr.contains("is not recognized as an internal or external command")
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let parts = match split(&command.script) {
            Ok(p) => p,
            Err(_) => return vec![], // If we can't parse it, we can't fix it properly
        };

        if parts.is_empty() {
            return vec![];
        }
        let typed_command = &parts[0];

        // Find similar commands
        let candidates = find_similar_commands(typed_command);

        let mut corrections = Vec::new();
        for candidate in candidates {
             // We need to reconstruct the command.
             // Ideally we replace the first word.
             // But simple string replacement might be dangerous if the command appears elsewhere.
             // Since `parts` is the parsed command, we can replace parts[0] and join.

             let mut new_parts = parts.clone();
             new_parts[0] = candidate;
             let new_cmd = shell_words::join(new_parts);

             corrections.push(Correction::new(new_cmd, false, 50)); // Lower priority than specific rules
        }

        corrections
    }
}

fn find_similar_commands(target: &str) -> Vec<String> {
    let path_var = env::var("PATH").unwrap_or_default();
    let mut candidates = HashSet::new(); // Use Set to avoid duplicates

    for path_str in env::split_paths(&path_var) {
        if let Ok(entries) = fs::read_dir(path_str) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_name) = entry.file_name().into_string() {
                         if is_executable(&entry.path()) {
                             let dist = levenshtein(target, &file_name);
                             let max_dist = 3; // Tolerance

                             if dist > 0 && dist <= max_dist {
                                 // Filter by ratio to avoid short command false positives
                                 let ratio = dist as f64 / target.len().max(file_name.len()) as f64;
                                 if ratio < 0.5 {
                                     candidates.insert(file_name);
                                 }
                             }
                         }
                    }
                }
            }
        }
    }

    // Sort by distance
    let mut sorted_candidates: Vec<String> = candidates.into_iter().collect();
    sorted_candidates.sort_by(|a, b| {
        levenshtein(target, a).cmp(&levenshtein(target, b))
    });

    sorted_candidates.truncate(3); // Top 3
    sorted_candidates
}

fn is_executable(path: &Path) -> bool {
    // In Unix, check execute permission. In Windows, check extension.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = path.metadata() {
            return metadata.permissions().mode() & 0o111 != 0;
        }
    }
    #[cfg(windows)]
    {
        if let Some(ext) = path.extension() {
             let ext_str = ext.to_string_lossy().to_lowercase();
             return ext_str == "exe" || ext_str == "cmd" || ext_str == "bat";
        }
    }

    #[cfg(not(any(unix, windows)))]
    {
        // Fallback for other OS or if we can't determine
        if let Ok(metadata) = path.metadata() {
            // Rough check: is it a file?
             return metadata.is_file();
        }
    }

    false
}
