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

        // If the original script was parsed as unquoted, but it has spaces, it will be multiple parts.
        // E.g., `cp file.txt /foo/bar baz/qux.txt` -> ["cp", "file.txt", "/foo/bar", "baz/qux.txt"]
        // Let's rely on standard shell words splitting. If it was properly quoted by user, shell_words::split handles it.
        // If it wasn't properly quoted (like in the test "cp file.txt /foo/bar baz/qux.txt"),
        // our split will give 4 parts. The test actually intends for the destination to be a single argument with spaces.
        // Wait, if it wasn't quoted, `shell_words::split` will yield 4 parts. The last one is "baz/qux.txt", parent is "baz".
        // The test explicitly says `let cmd4 = Command::new("cp file.txt /foo/bar baz/qux.txt", "", ...);`
        // So the last argument is "baz/qux.txt". The `test_generate_corrections` fails because it asserts "mkdir -p '/foo/bar baz' && cp file.txt '/foo/bar baz/qux.txt'".
        // But the script is not quoted.

        // The test was probably wrong. Let's fix the test to pass properly quoted script so that shell_words parses it as 3 parts.

        let dest = &parts[parts.len() - 1];

        let path = Path::new(dest);

        // We want to create the directory of the destination
        let dir_to_create = if dest.ends_with('/') {
            // It's explicitly a directory
            dest.clone()
        } else {
            // It might be a file, get its parent directory
            match path.parent() {
                Some(p) if !p.as_os_str().is_empty() => p.to_string_lossy().into_owned(),
                _ => return vec![], // No parent directory to create
            }
        };

        let quoted_dir = shell_escape::escape(dir_to_create.into());
        let new_script = shell_words::join(&parts);
        let new_cmd = format!("mkdir -p {} && {}", quoted_dir, new_script);

        vec![Correction::new(new_cmd, false, 90)]
    }
}
