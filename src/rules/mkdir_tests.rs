
#[cfg(test)]
mod tests {
    use crate::rules::mkdir::MkdirP;
    use crate::types::Command;
    use crate::rules::Rule;

    #[test]
    fn test_mkdir_p() {
        let rule = MkdirP;
        let command = Command::new(
            "mkdir foo/bar".to_string(),
            "".to_string(),
            "mkdir: cannot create directory ‘foo/bar’: No such file or directory".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, "mkdir -p foo/bar");
    }

    #[test]
    fn test_mkdir_p_no_match() {
        let rule = MkdirP;
        let command = Command::new(
            "mkdir foo".to_string(),
            "".to_string(),
            "".to_string(),
        );

        assert!(!rule.matches(&command));
    }
}
