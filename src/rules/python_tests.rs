#[cfg(test)]
mod tests {
    use crate::rules::python::{PythonExecute, PipUnknownCommand};
    use crate::types::Command;
    use crate::rules::Rule;
    use tempfile::NamedTempFile;

    #[test]
    fn test_python_execute() {
        let rule = PythonExecute;

        // Create dummy .py file
        let file = NamedTempFile::with_suffix(".py").unwrap();
        let path = file.path().to_str().unwrap();
        // The command is the filename without .py
        let script_name = path.strip_suffix(".py").unwrap();

        let command = Command::new(
            script_name.to_string(),
            "".to_string(),
            "".to_string(), // Stderr often empty if shell just couldn't find it? Or "command not found"
        );

        // We need the file to actually exist for the rule to match
        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, format!("{}.py", script_name));
    }

    #[test]
    fn test_pip_unknown_command() {
        let rule = PipUnknownCommand;
        let command = Command::new(
            "pip instatl numpy".to_string(),
            "".to_string(),
            "ERROR: unknown command \"instatl\", maybe you meant \"install\"".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, "pip install numpy");
    }
}
