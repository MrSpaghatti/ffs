use crate::types::{Command, Correction};
use crate::rules::Rule;
use shell_words::split;
use std::path::Path;

#[derive(Debug)]
pub struct CpCreateDestination;

impl Rule for CpCreateDestination {
    fn name(&self) -> &str {
        "cp_create_destination"
    }

    fn matches(&self, command: &Command) -> bool {
        let script = command.script.trim();
        let stderr = command.stderr.to_lowercase();

        script.starts_with("cp ") &&
        (stderr.contains("no such file or directory") ||
         stderr.contains("cannot create regular file"))
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let parts = match split(&command.script) {
            Ok(p) => p,
            Err(_) => return vec![],
        };

        if parts.len() < 3 {
            return vec![];
        }

        let dest = &parts[parts.len() - 1];
        let path = Path::new(dest);

        let dir_to_create = if dest.ends_with('/') {
            dest.clone()
        } else {
            match path.parent() {
                Some(p) if !p.as_os_str().is_empty() => p.to_string_lossy().into_owned(),
                _ => return vec![],
            }
        };

        let quoted_dir = shell_escape::escape(dir_to_create.into());
        let new_script = shell_words::join(&parts);
        let new_cmd = format!("mkdir -p {} && {}", quoted_dir, new_script);

        vec![Correction::new(new_cmd, false, 90)]
    }
}
