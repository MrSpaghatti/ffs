#[cfg(test)]
mod tests {
    use crate::rules::rm::RmDir;
    use crate::types::Command;
    use crate::rules::Rule;

    #[test]
    fn test_rm_dir() {
        let rule = RmDir;
        let command = Command::new(
            "rm foo".to_string(),
            "".to_string(),
            "rm: cannot remove 'foo': Is a directory".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, "rm -rf foo");
    }

    #[test]
    fn test_rm_dir_no_match() {
        let rule = RmDir;

        let command_not_rm = Command::new(
            "rmdir foo".to_string(),
            "".to_string(),
            "rmdir: failed to remove 'foo': Directory not empty".to_string(),
        );
        assert!(!rule.matches(&command_not_rm));

        let command_no_error = Command::new(
            "rm foo".to_string(),
            "".to_string(),
            "".to_string(),
        );
        assert!(!rule.matches(&command_no_error));

        let command_has_r = Command::new(
            "rm -r foo".to_string(),
            "".to_string(),
            "rm: cannot remove 'foo': Is a directory".to_string(),
        );
        assert!(!rule.matches(&command_has_r));

        let command_has_rf = Command::new(
            "rm -rf foo".to_string(),
            "".to_string(),
            "rm: cannot remove 'foo': Is a directory".to_string(),
        );
        assert!(!rule.matches(&command_has_rf));
    }
}
