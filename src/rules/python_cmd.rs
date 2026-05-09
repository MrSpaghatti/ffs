use crate::types::{Command, Correction};
use crate::rules::Rule;
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct PythonCommand;

impl Rule for PythonCommand {
    fn name(&self) -> &str {
        "python_command"
    }

    fn matches(&self, command: &Command) -> bool {
        let script_parts = match shell_words::split(&command.script) {
            Ok(parts) => parts,
            Err(_) => return false,
        };
        if script_parts.is_empty() {
            return false;
        }
        
        let path_str = &script_parts[0];
        let path = Path::new(path_str);
        
        let stderr_lower = command.stderr.to_lowercase();
        let has_error = stderr_lower.contains("permission denied") || stderr_lower.contains("exec format error");
        
        if !has_error {
            return false;
        }

        if !path.exists() || !path.is_file() {
            return false;
        }

        if let Ok(metadata) = fs::metadata(path) {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = metadata.permissions().mode();
                if mode & 0o111 != 0 {
                    return false;
                }
            }
        }

        if let Ok(content) = fs::read_to_string(path) {
            if content.starts_with("#!") {
                return false;
            }
        } else {
            return false;
        }

        true
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let new_cmd = format!("python {}", command.script);
        vec![Correction::new(new_cmd, false, 100)]
    }
}
