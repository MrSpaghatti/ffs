#[cfg(test)]
mod tests {
    use crate::rules::cd::CdMkdir;
    use crate::types::Command;
    use crate::rules::Rule;

    #[test]
    fn test_cd_mkdir() {
        let rule = CdMkdir;
        let command = Command::new(
            "cd foo/bar".to_string(),
            "".to_string(),
            "bash: cd: foo/bar: No such file or directory".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, "mkdir -p foo/bar && cd foo/bar");
    }
}
