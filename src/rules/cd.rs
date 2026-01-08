use crate::types::{Command, Correction};
use crate::rules::Rule;
use shell_words::split;

#[derive(Debug)]
pub struct CdMkdir;

impl Rule for CdMkdir {
    fn name(&self) -> &str {
        "cd_mkdir"
    }

    fn matches(&self, command: &Command) -> bool {
        let script = command.script.trim();
        let stderr = command.stderr.to_lowercase();

        script.starts_with("cd ") &&
        (stderr.contains("no such file or directory") ||
         stderr.contains("does not exist") ||
         stderr.contains("can't cd to"))
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let parts = match split(&command.script) {
            Ok(p) => p,
            Err(_) => return vec![],
        };

        if parts.len() < 2 {
            return vec![];
        }

        // The path is the second argument (index 1)
        // If there are flags like -P or -L, we might need to be smarter, but usually it's `cd path`
        // Assuming `cd path` for now.
        let path = &parts[1];

        // Safe quoting of path
        let quoted_path = shell_escape::escape(path.clone().into());
        let new_cmd = format!("mkdir -p {} && {}", quoted_path, command.script);

        vec![Correction::new(new_cmd, false, 90)]
    }
}
